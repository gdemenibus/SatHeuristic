#!/bin/bash

# Directory containing the files
directory="../data/datasets/j30.sm/"

# Batch size (number of files to process in each batch)
batch_size=5

# Function to process a file
process_file() {
    file="$1"
    # Run your command with the file as an argument
    ../target/release/modeling "$file"
}

# Export the function to make it available to child processes
export -f process_file

# Generate the file list using find
file_list=()
while IFS= read -r -d '' file; do
    file_list+=("$file")
done < <(find "$directory" -type f -print0)

# Function to process a batch of files in parallel
process_batch() {
    files=("$@")
    for file in "${files[@]}"; do
        # Process each file in parallel
        process_file "$file" &
    done
    wait
}

# Export the function to make it available to child processes
export -f process_batch

# Iterate over file list in batches
batch_start=0
while ((batch_start < ${#file_list[@]})); do
    batch_end=$((batch_start + batch_size))
    if ((batch_end > ${#file_list[@]})); then
        batch_end=${#file_list[@]}
    fi

    batch_files=("${file_list[@]:batch_start:batch_size}")

    # Process the batch of files in parallel
    process_batch "${batch_files[@]}"

    batch_start=$((batch_start + batch_size))
done
