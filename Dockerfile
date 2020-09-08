FROM rust:latest as rust-build-stage
WORKDIR /app
COPY webgl/. .
RUN cargo update
RUN cargo install wasm-pack
RUN wasm-pack build

FROM node:latest as js-build-stage
WORKDIR /app
COPY . .
COPY --from=rust-build-stage /app/pkg /app/webgl/pkg
RUN ls /app/webgl
RUN yarn
RUN cd /app/frontend && yarn build

FROM nginx as production-stage
RUN mkdir /app
COPY --from=js-build-stage /app/frontend/dist /app
COPY nginx.conf /etc/nginx/nginx.conf
