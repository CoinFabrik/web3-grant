#!/bin/bash

# Find all paths that have an underscore in their name
paths_with_underscores=$(find . ! -path '*/.*' \( -type f -o -type d \) -name '*_*' -print)

if [ -n "$paths_with_underscores" ]; then
    echo "Paths with underscores found:"
    echo "$paths_with_underscores"
    echo ""
    echo "Please rename the paths to not contain underscores, use hyphens instead."
    exit 1
else
    echo "No paths with underscores found."
fi
