#!/bin/bash

# Install Java
sudo apt-get update
sudo apt-get install openjdk-17-jdk -y

# Create a directory for WebGoat
mkdir -p webgoat_dir

# Download WebGoat
wget https://github.com/WebGoat/WebGoat/releases/download/v2023.8/webgoat-2023.8.jar -O webgoat_dir/webgoat.jar

# Set permissions
chmod +x webgoat_dir/webgoat.jar

echo "WebGoat installed successfully in webgoat_dir"