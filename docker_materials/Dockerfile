# Start with the latest Rust image
FROM rust:latest

# Avoid prompts during package installation
ARG DEBIAN_FRONTEND=noninteractive

# Install additional development tools
RUN apt-get update && apt-get install -y \
    curl \
    git \
    vim \
    gdb \
    clang \
    lldb \
    valgrind \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory in the container
WORKDIR /workspace

# Keep the container running
CMD ["bash"]
