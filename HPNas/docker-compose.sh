#!/bin/bash

# Define the list of directories
directories=(
    "./NAS-Media/"
    "../utils/"
    "./Storage/"
    # Add more directories as needed
)

# Function to display usage
function usage {
    echo "Usage: $0 [--up | --down]"
    exit 1
}

# Check the argument provided
case $1 in
    --up)
        action="up -d"
        ;;
    --down)
        action="down"
        ;;
    *)
        usage
        ;;
esac

# Iterate through the directories and run docker-compose
for dir in "${directories[@]}"; do
    echo "Running docker-compose $action in directory: $dir"
    (cd "$dir" && docker-compose $action)
done

