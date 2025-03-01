# Use the official Rust image as a parent image
FROM rust:1.72

# Set the working directory inside the container
WORKDIR /usr/src/klearlink-api

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Set the startup command to run the binary
CMD ["./target/release/klearlink-api"]
