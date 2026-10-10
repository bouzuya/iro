FROM rust:1.99-slim-trixie AS chef
RUN cargo install cargo-chef
RUN cargo install topcoat-cli@0.10.0 --locked
WORKDIR /app

FROM chef AS planner
COPY . ./
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt-get update && apt-get install -y \
    git \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . ./
RUN cargo build --release
RUN topcoat asset bundle --release

FROM debian:trixie-slim AS runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/iro /app/iro
COPY --from=builder /app/target/release/assets /app/src/app/lab/iro/assets
ENV HOST=0.0.0.0
ENV ORIGIN=http://localhost:3000
ENV PORT=3000
EXPOSE 3000
CMD ["/app/iro"]
