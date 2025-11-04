FROM rust:alpine

# Install build dependencies
RUN apk add --no-cache musl-dev

WORKDIR /app

# Install cargo-watch for hot reload
RUN cargo install cargo-watch

# Copy Cargo files first for better caching
COPY Cargo.toml ./

# Create dummy lib.rs (since this is a library crate)
RUN mkdir -p src && echo "// dummy" > src/lib.rs

# Create benches directory structure to satisfy Cargo.toml
RUN mkdir -p benches && \
    echo "fn main() {}" > benches/csv_parsing.rs && \
    echo "fn main() {}" > benches/stats_calculation.rs && \
    echo "fn main() {}" > benches/api_endpoints.rs

# Build dependencies
RUN cargo build --release
RUN rm -rf src benches

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# Expose port
EXPOSE 3001

# Start the application with hot reload using cargo watch
CMD ["cargo", "watch", "-x", "run"]