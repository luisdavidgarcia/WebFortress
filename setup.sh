#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

install_linux_dependencies() {
    echo "Updating and installing necessary packages for Linux..."
    sudo apt-get update && sudo apt-get install -y curl git build-essential python3-pip
}

install_mac_dependencies() {
    echo "Updating and installing necessary packages for macOS..."
    # Install Homebrew if not installed
    if ! command -v brew &>/dev/null; then
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"
    fi
    brew install curl git python3
}

install_rust() {
    echo "Installing Rust and Cargo..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env" || export PATH="$HOME/.cargo/bin:$PATH"
}

install_pre_commit() {
    echo "Installing pre-commit..."
    pip3 install pre-commit
}

setup_pre_commit_hooks() {
    echo "Setting up pre-commit hooks..."
    pre-commit install
}

install_docker_linux() {
    if ! command -v docker &>/dev/null; then
        echo "Docker is not installed. Installing Docker for Linux..."
        sudo apt-get install -y docker.io
        sudo usermod -aG docker $USER
        echo "NOTE: You may need to log out and log back in for Docker group changes to take effect."
    else
        echo "Docker is already installed."
    fi
}

install_docker_mac() {
    if ! command -v docker &>/dev/null; then
        echo "Docker is not installed. Installing Docker for macOS..."
        brew install --cask docker
        open /Applications/Docker.app
        echo "Please follow any additional instructions to complete Docker installation."
    else
        echo "Docker is already installed."
    fi
}


# Detect OS and install dependencies
OS="`uname`"
case $OS in
  'Linux')
    install_linux_dependencies
    install_docker_linux
    ;;
  'Darwin')
    install_mac_dependencies
    install_docker_mac
    ;;
  *)
    echo "Operating system $OS not supported."
    exit 1
    ;;
esac

# Install Rust, pre-commit, and set up hooks (common across Linux and macOS)
command -v rustc &>/dev/null || install_rust
command -v pre-commit &>/dev/null || install_pre_commit
setup_pre_commit_hooks

echo "Setup completed successfully."
echo "You can now start working on the project!"
