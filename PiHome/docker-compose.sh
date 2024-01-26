#!/bin/bash

# Define the list of directories
directories=(
    "../utils/"  # Add more directories as needed
    "./Security"
)

# Function to display usage
function usage {
    echo "Usage: $0 [--up | --down]"
    exit 1
}

# Check the number of arguments provided
if [ $# -ne 1 ]; then
    usage
fi

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
    (cd "$dir" && docker compose $action)
done
