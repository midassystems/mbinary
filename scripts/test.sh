#!/bin/bash
# shellcheck disable=SC1091

module="$1"

if [ ! "$module" ]; then
	echo "Argument not passed, please add argument {rust | python}"
	exit 1
fi

python() {
	# Ensure we are in the project root
	cd "$(dirname "$0")/.." || exit

	VENV="venv"

	# Check if the virtual environment exists
	if [ ! -d "$VENV" ]; then
		echo "Error: Virtual environment not found at $VENV"
		exit 1
	fi

	# Activate virtual environment
	source venv/bin/activate

	# Run Python tests inside the virtual environment directly
	if cd python; then
		maturin develop
		python3 -m unittest discover
	fi

}

rust() {
	if cd rust; then
		cargo test -- --nocapture
	fi
}

c() {
	if cd c; then
		# Build
		cargo build
		cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -B build
		cmake --build ./build
		if cd build; then
			ctest --verbose
		fi
	fi
}

options() {
	echo "Which tests would you like to run?"
	echo "1 - rust"
	echo "2 - python"

}

# Main
case "$module" in
python)
	python
	;;
rust)
	rust
	;;
c)
	c
	;;
*) echo "Invalid argument, valid arguments {rust|python}" ;;

esac
