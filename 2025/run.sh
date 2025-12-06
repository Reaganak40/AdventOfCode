#!/bin/bash

path_of_script=$(dirname "$0")

path_to_inputs="$(cd "$path_of_script/inputs" && pwd)"
if [ ! -d "$path_to_inputs" ]; then
    echo "Inputs directory not found at $path_to_inputs"
    exit 1
fi

export ADVENT_INPUTS_DIR="$path_to_inputs"
cargo run --release
