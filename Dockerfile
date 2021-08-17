FROM rust:1.54-slim as planner
WORKDIR app
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:1.54-slim as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.54-slim as builder
WORKDIR app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

FROM rust:1.54-slim as runtime
WORKDIR app
COPY --from=builder /app/target/release/rust-ticketing /usr/local/bin
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/rust-ticketing"]
