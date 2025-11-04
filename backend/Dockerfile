FROM rust:alpine

# Install build dependencies
RUN apk add --no-cache musl-dev

WORKDIR /app

# Install cargo-watch for hot reload
RUN cargo install cargo-watch

# Copy Cargo files first for better caching
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release
RUN rm src/main.rs

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# Expose port
EXPOSE 3001

# Start the application with hot reload using cargo watch
CMD ["cargo", "watch", "-x", "run"]