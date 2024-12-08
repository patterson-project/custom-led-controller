FROM balenalib/rpi-raspbian:stretch

# Install required dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Create a new directory for the application
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs file to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies
RUN cargo build --release

# Remove the dummy main.rs file
RUN rm src/main.rs

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Expose the port the application runs on
EXPOSE 8080

# Set the entrypoint to run the application
CMD ["./target/release/custom-led-controller"]