use crate::prelude::graphql::*;
use async_trait::async_trait;
use std::fmt::Debug;
use std::sync::Arc;

/// A cache resolution trait.
///
/// Clients of CachingMap are required to provider a resolver during Map creation. The resolver
/// will be used to find values for cache misses. A Result is expected, because retrieval may fail.
#[async_trait]
pub trait CacheResolver<K, V> {
    async fn retrieve(&self, key: K) -> Result<V, CacheResolverError>;
}

/// A planner key.
///
/// This type consists of a query string, an optional operation string and the
/// [`QueryPlanOptions`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct QueryKey {
    pub(crate) query: String,
    pub(crate) operation: Option<String>,
    pub(crate) options: QueryPlanOptions,
}

/// QueryPlanner can be used to plan queries.
///
/// Implementations may cache query plans.
#[async_trait]
pub trait QueryPlanner: Send + Sync + Debug {
    /// Returns a query plan given the query, operation and options.
    /// Implementations may cache query plans.
    #[must_use = "query plan result must be used"]
    async fn get(
        &self,
        query: String,
        operation: Option<String>,
        options: QueryPlanOptions,
    ) -> Result<Arc<QueryPlan>, QueryPlannerError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::*;

    assert_obj_safe!(QueryPlanner);
}
