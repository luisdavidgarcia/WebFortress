#!/bin/bash

# Stop the script if any command fails
set -e

# Function to install dependencies on Linux
install_linux_dependencies() {
    echo "Updating and installing necessary packages for Linux..."
    sudo apt-get update && sudo apt-get install -y curl git build-essential python3-pip libseccomp-dev
}

# Function to install Rust
install_rust() {
    echo "Installing Rust and Cargo..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env" || export PATH="$HOME/.cargo/bin:$PATH"
}

# Function to install and setup pre-commit
install_and_setup_pre_commit() {
    echo "Installing pre-commit and setting up hooks..."
    pip3 install pre-commit
    pre-commit install
}

# Detect OS and install dependencies
detect_and_install_dependencies() {
    if [ "$(uname)" != "Linux" ]; then
        echo "This script can only be run on a Linux environment."
        exit 1
    fi
    install_linux_dependencies
    install_and_setup_pre_commit
}

# Main function to orchestrate setup
main() {
    detect_and_install_dependencies
}

# Invoke the main function
main
