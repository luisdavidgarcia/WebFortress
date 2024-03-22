#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Update and install necessary packages
echo "Updating and installing necessary packages..."
sudo apt-get update && sudo apt-get install -y curl git build-essential

# Check if Rust is installed, install it if not
if ! command -v rustc &>/dev/null; then
    echo "Installing Rust and Cargo..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust is already installed."
fi

# Add Rust's cargo bin directory to the PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Install rustfmt, clippy
echo "Installing rustfmt and clippy..."
rustup component add rustfmt clippy

# Install pre-commit
echo "Installing pre-commit..."
if ! command -v pre-commit &>/dev/null; then
    if command -v pip &>/dev/null || command -v pip3 &>/dev/null; then
        pip install pre-commit || pip3 install pre-commit
    else
        echo "Error: Python pip is not installed. Attempting to install Python pip..."
        sudo apt-get install -y python3-pip
        pip3 install pre-commit
    fi
else
    echo "pre-commit is already installed."
fi

# Set up pre-commit hooks
echo "Setting up pre-commit hooks..."
pre-commit install

# Check if Docker is installed, install it if not
if ! command -v docker &>/dev/null; then
    echo "Docker is not installed. Installing Docker..."
    sudo apt-get install -y docker.io
else
    echo "Docker is already installed."
fi

# Ensure the user can run Docker commands without sudo (requires logout/login)
if ! groups $USER | grep &>/dev/null '\bdocker\b'; then
    echo "Adding user to the Docker group..."
    sudo usermod -aG docker $USER
    echo "NOTE: You may need to log out and log back in for Docker group changes to take effect."
fi

# Navigate to Docker directory and run Docker setup script
echo "Setting up Docker environment..."
(cd Docker && ./setup.sh)

echo "Setup completed successfully."
echo "You can now start working on the project!"
