# Use an official Ubuntu base image
FROM ubuntu:22.04 

# Avoid prompts during package installation
ARG DEBIAN_FRONTEND=noninteractive

# Install build essentials and tools
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    vim \
    gdb \
    clang \
    lldb \
    valgrind \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Make Rust available in the PATH for this Dockerfile
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory
WORKDIR /workspace

# Keep the container running
CMD ["bash"]
