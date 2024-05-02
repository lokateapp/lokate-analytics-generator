FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json

# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same, all layers should be cached
COPY . .

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends python3.11 python3.11-dev \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Build our project
RUN cargo build --release --bin lokate_analytics_server

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates python3.11 python3.11-dev libpython3.11 libpython3.11-dev python3-pip \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY requirements.txt .
RUN pip install -r requirements.txt --break-system-packages


COPY --from=builder /app/target/release/lokate_analytics_server lokate_analytics_server
COPY configuration configuration
COPY .env .
COPY models models
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./lokate_analytics_server"]


# # Builder stage
# FROM rust:1.76.0 AS builder

# WORKDIR /app
# COPY . .
# RUN cargo build --release

# # Runtime stage
# FROM debian:bullseye-slim AS runtime

# WORKDIR /app
# RUN apt-get update -y \
#     && apt-get install -y --no-install-recommends openssl ca-certificates \
#     && apt-get autoremove -y \
#     && apt-get clean -y \
#     && rm -rf /var/lib/apt/lists/*

# # Copy the compiled binary from the builder environment to our runtime environment
# COPY --from=builder /app/target/release/lokate_analytics_server lokate_analytics_server
# COPY configuration configuration
# ENV APP_ENVIRONMENT production
# ENTRYPOINT ["./lokate_analytics_server"]
