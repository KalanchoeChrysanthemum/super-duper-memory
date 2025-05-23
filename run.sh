#!/usr/bin/env bash



# Current test command


# Runs main with debug printing 
# and the default test suite.

# Passes extra args to the test command for parsing



# Example Usage:

# ./run.sh echo 'Hello, World!'

# "$*" takes care of the quotes, so no need to wrap your command
# (Also properly handles no args passed)
cargo run --bin main -- -v --test "$*"

