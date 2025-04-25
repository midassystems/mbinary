#!/bin/bash

BASE_DIR="$(dirname -- "${BASH_SOURCE[0]}")/.."

mkdir -p "$BASE_DIR/include"

if cd "$BASE_DIR/c"; then
	if cargo build --release; then
		cp "$BASE_DIR/target/mbinary.h" "$BASE_DIR/include/mbinary.h"
		echo "mbinary compiled."
	else
		echo "ERROR: cargo build failed"
		return 1
	fi
fi
