#!/bin/bash

# Stop the script if any command fails
set -e

# Function to install dependencies on Linux
install_linux_dependencies() {
    echo "Updating and installing necessary packages for Linux..."
    sudo apt-get update && sudo apt-get install -y curl git build-essential python3-pip docker.io libseccomp-dev
    sudo usermod -aG docker $USER
    echo "NOTE: You may need to log out and log back in for Docker group changes to take effect."
}

# Function to install dependencies on macOS
install_mac_dependencies() {
    echo "Updating and installing necessary packages for macOS..."
    if ! command -v brew &>/dev/null; then
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"
    fi
    brew install curl git python3 docker
    open /Applications/Docker.app
    echo "Please follow any additional instructions to complete Docker installation."
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
    OS="`uname`"
    case $OS in
      'Linux')
        install_linux_dependencies
        ;;
      'Darwin')
        install_mac_dependencies
        ;;
      *)
        echo "Operating system $OS not supported."
        exit 1
        ;;
    esac

    install_rust
    install_and_setup_pre_commit
}

# Docker image and container management
manage_docker() {
    IMAGE_NAME="rustosprojects"
    CONTAINER_NAME="rustos_dev"

    if [[ "$(docker images -q $IMAGE_NAME 2> /dev/null)" == "" ]]; then
      echo "Building Docker image '$IMAGE_NAME'..."
      docker build -t $IMAGE_NAME .
    else
      echo "Docker image '$IMAGE_NAME' already exists."
    fi

    if [ "$(docker ps -aq -f name=^${CONTAINER_NAME}$)" ]; then
        if [ ! "$(docker ps -q -f name=^${CONTAINER_NAME}$)" ]; then
            echo "Starting existing container '$CONTAINER_NAME'..."
            docker start $CONTAINER_NAME
        else
            echo "Container '$CONTAINER_NAME' is already running."
        fi
    else
        echo "Creating and starting new container '$CONTAINER_NAME'..."
        docker run -dit --name $CONTAINER_NAME -v "$(pwd):/workspace" $IMAGE_NAME
    fi

    echo "Attaching to the container '$CONTAINER_NAME'. Press Ctrl+D to detach."
    docker attach $CONTAINER_NAME
}

# Main function to orchestrate setup and docker management
main() {
    detect_and_install_dependencies
    manage_docker
}

# Invoke the main function
main
