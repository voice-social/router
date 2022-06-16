use async_trait::async_trait;
use bytes::Bytes;
use futures::lock::Mutex;
use redis::{aio::Connection, AsyncCommands, Client};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tower::BoxError;

use crate::{CacheResolver, CacheResolverError};

pub struct RedisCache<K, V> {
    client: Client,
    connection: Mutex<Connection>,
    resolver: Box<dyn CacheResolver<K, V> + Send + Sync>,
}

impl<K, V> RedisCache<K, V> {
    pub async fn new(
        url: &str,
        resolver: Box<(dyn CacheResolver<K, V> + Send + Sync)>,
    ) -> Result<Self, BoxError> {
        let client = Client::open(url)?;
        let connection = client.get_tokio_connection().await?;

        Ok(RedisCache {
            client,
            connection: Mutex::new(connection),
            resolver,
        })
    }
}

#[async_trait]
impl<K: Clone + Into<String> + Send, V: Serialize + DeserializeOwned + Send> CacheResolver<K, V>
    for RedisCache<K, V>
{
    async fn retrieve(&self, key: K) -> Result<V, CacheResolverError> {
        //FIXME: connection pooling? Reconnection?
        let ks: String = key.clone().into();
        let res: Result<Vec<u8>, _> = self.connection.lock().await.get(ks).await;
        match res {
            Ok(value) => {
                let v = serde_json::from_slice(&value).expect("FIXME");
                Ok(v)
            }
            Err(e) => {
                tracing::error!("err: {:?}", e);
                let k = key.clone();
                let value = self.resolver.retrieve(k).await?;
                let v = serde_json::to_vec(&value).expect("FIXME");
                let _: () = self
                    .connection
                    .lock()
                    .await
                    .set(key.into(), v)
                    .await
                    .expect("FIXME");
                Ok(value)
            }
        }
    }
}
