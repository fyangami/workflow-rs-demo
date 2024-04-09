FROM rust:alpine as build
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo check && cargo build -r

FROM alpine
COPY --from=build /app/target/release/workflow-rs-demo /app
CMD ["/app"]
