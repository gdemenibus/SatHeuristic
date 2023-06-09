#!/bin/bash

# Directory containing the files
directory="../data/datasets/hold_data"

# Function to process a file
process_file() {
    file="$1"
    # Run your command with the file as an argument
	../target/release/modeling "$file"
}

# Export the function to make it available to child processes
export -f process_file

# Generate the file list using find and run the command in parallel
find "$directory" -type f -print0 | xargs -0 -n 1 -P 4 bash -c 'process_file "$@"' _

# Note: Adjust the value of `-P` according to the number of parallel processes you want to use.
