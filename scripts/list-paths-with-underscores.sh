#!/bin/bash

# Find all paths that have an underscore in their name
all_paths_with_underscores=$(find . ! -path '*/.*' \( -type f -o -type d \) -name '*_*' -print)

# Ignore paths that are in the .gitignore file
paths_with_underscores=""
for path in $all_paths_with_underscores; do
    # Check if the path is in the .gitignore file
    if ! git check-ignore -q $path; then
        paths_with_underscores="$paths_with_underscores
$path"
    fi
done

# Trim the leading newline
paths_with_underscores=${paths_with_underscores:1}

# Print the results
if [ -n "$paths_with_underscores" ]; then
    echo "Paths with underscores found:"
    echo "$paths_with_underscores"
    echo ""
    echo "Please rename the paths to not contain underscores, use hyphens instead."
    exit 1
else
    echo "No paths with underscores found."
fi
