#!/bin/bash

# Stop the script if any command fails
set -e

# Define your Docker image and container names
IMAGE_NAME="rustosprojects"
CONTAINER_NAME="rustos_dev"

# Check if the Docker image already exists
if [[ "$(docker images -q $IMAGE_NAME 2> /dev/null)" == "" ]]; then
  echo "Building Docker image '$IMAGE_NAME'..."
  docker build -t $IMAGE_NAME .
else
  echo "Docker image '$IMAGE_NAME' already exists."
fi

# Check if the container is already running
if [ "$(docker ps -aq -f name=^${CONTAINER_NAME}$)" ]; then
    # Check if the container is stopped and start it
    if [ ! "$(docker ps -q -f name=^${CONTAINER_NAME}$)" ]; then
        echo "Starting existing container '$CONTAINER_NAME'..."
        docker start $CONTAINER_NAME
    else
        echo "Container '$CONTAINER_NAME' is already running."
    fi
else
    # Run a new container from the image
    echo "Creating and starting new container '$CONTAINER_NAME'..."
    docker run -dit --name $CONTAINER_NAME -v "$(pwd):/workspace" $IMAGE_NAME
fi

# Attach to the container
echo "Attaching to the container '$CONTAINER_NAME'. Press Ctrl+D to detach."
docker attach $CONTAINER_NAME
