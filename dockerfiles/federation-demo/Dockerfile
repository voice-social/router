FROM node:18-alpine

ADD federation-demo/package.json federation-demo/package-lock.json federation-demo/lerna.json /src/

WORKDIR /src
RUN npm install

ADD federation-demo /src
