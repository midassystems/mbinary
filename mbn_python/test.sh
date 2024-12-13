#!/bin/bash

# Function to run the build
builder() {
	echo "Building the project..."
	source venv/bin/activate
	python -m build

}

# Function to run the tests
tester() {
	echo "Running tests..."
	source venv/bin/activate
	maturin develop
	python -m unittest discover
}

options() {
	echo "Which tests would you like to run?"
	echo "1 - Test"
	echo "2 - Build"
	echo "3 - Test & Build"

}

# Main
while true; do
	options
	read -r option

	case $option in
	1)
		tester
		break
		;;
	2)
		builder
		break
		;;
	3)
		tester
		builder
		break
		;;
	*) echo "Please choose a different one." ;;
	esac
done
