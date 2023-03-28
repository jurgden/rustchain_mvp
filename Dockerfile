# Use Rust 1.46 as the base image
FROM rust:1.46

# Set the working directory
WORKDIR /usr/src/rustchain_mvp

# Copy the Cargo.toml file to the working directory
COPY Cargo.toml .

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo 'fn main() {}' > src/main.rs

# Build dependencies
RUN cargo build --release
RUN rm -rf src

# Copy the source code and other files to the working directory
COPY . .

# Build the project
RUN cargo build --release

# Set the entrypoint
CMD ["cargo", "run", "--release"]

