#!/bin/bash

# Function to run the build
build() {
	echo "Building the project..."
	source venv/bin/activate
	python -m build

}

# Function to run the tests
test() {
	echo "Running tests..."
	source venv/bin/activate
	maturin develop
	python -m unittest discover
}

# Check the command-line argument
case "$1" in
build)
	build
	;;
test)
	test
	;;
test-and-build)
	test
	build
	;;
*)
	echo "Usage: $0 {build|test|build-and-test}"
	exit 1
	;;
esac

# source venv/bin/activate
# maturin develop
# python -m unittest discover
