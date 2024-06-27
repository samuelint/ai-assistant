#!/bin/bash
# Get the directory of the current script
script_dir=$(dirname "$(readlink -f "$0")")

# Define the relative paths to the binaries
tauri_dist_binary="$script_dir/src-tauri/bin"
core_dist_binary="$script_dir/../core/dist"

cp -r "$core_dist_binary"/* "$tauri_dist_binary"