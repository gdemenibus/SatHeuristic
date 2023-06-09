#!/bin/bash

# Directory path
directory="../data/"

# Iterate over files in the directory
for file in "$directory"/*
do
    if [ -f "$file" ]; then
        # Execute the pipe command for each file
        grep -v -E '^\s{4,}' < "$file" | sed '/^\s*$/d' | awk '{ if ($13 == "3") $2 = "1"; print }' > "${file}.filtered"
        
        # Replace the original file with the filtered file
        mv "${file}.filtered" "$file"
    fi
done
