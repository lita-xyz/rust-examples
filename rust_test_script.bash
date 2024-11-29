#!/bin/bash

# This script tests the crates in the rust-examples directory.
# It builds each crate and runs it on the Valida VM and the x86_64-unknown-linux-gnu host, comparing the VM output to the host output.
# It skips any crates that don't have a input file in the test_data directory.
# For the guessing game crate, it is actually random on the host machine so we need to give it the expected output by hand.
# To add a new crate to the test, add a directory with the same name as the crate to the test_data directory and put the input file in it.
# If there is no input, add an empty input file.
# If the crate has randomness, you will need to give it the expected output by hand. Add the crate name to the RANDOM_CRATES array below.
# Make sure that you have set the vm_executable variable to the path to the valida-vm executable.
# If your host is not x86_64 linux, you will need to change the target in the cargo run command.
# Run the script in the `rust-examples` directory with: bash rust_test_script.bash
# To test a specific crate: bash rust_test_script.bash -c <crate_name>
# If you want to indicate that a crate is failing, move it's test data to the 'failing' directory.

# configuration
test_data_dir='test_data'
crates_dir='.'
# path to the valida-vm executable
vm_executable='/valida-toolchain/bin/valida'

# utilities
function fail {
    local message="$1"
    local exit_code="${2:-1}"  # Default to 1 if no code provided
    echo "[ERROR] ${message}" >&2  # Print to stderr
    exit "$exit_code"
}

# global sanity check
test -d "$test_data_dir" || fail "the test data directory '${test_data_dir}' does not exist, please create it and put the test input data in it"
test -f "$vm_executable" || fail "the vm executable '${vm_executable}' is not a file, please correctly specify the path to the valida-vm executable in the test script"
test -x "$vm_executable" || fail "the vm executable '${vm_executable}' is not executable, please correctly specify the path to the valida-vm executable in the test script"

# Parse command line arguments
crate_to_test=""
while getopts "c:" opt; do
  case $opt in
    c)
      crate_to_test="$OPTARG"
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      echo "Usage: $0 [-c crate_name]" >&2
      exit 1
      ;;
  esac
done

# If a specific crate was specified, verify it exists
if [ -n "$crate_to_test" ]; then
  if [ ! -d "$test_data_dir/$crate_to_test" ]; then
    fail "crate '$crate_to_test' not found in test data directory"
  fi
fi

if [ -n "$crate_to_test" ]; then
  crate_test_dirs=("$test_data_dir/$crate_to_test")
else
  # Get all test directories except failing
  crate_test_dirs=()
  for dir in "$test_data_dir"/*; do
    if [ "$(basename "$dir")" != "failing" ]; then
      crate_test_dirs+=("$dir")
    fi
  done
fi

for crate_test_dir in "${crate_test_dirs[@]}"
do {
  crate=$(basename "${crate_test_dir}")

  # build crate, silently to avoid polluting the output
  echo "building ${crate}"
  pushd "${crates_dir}/${crate}"
  cargo clean --quiet
  if ! cargo +valida build --quiet
  then
    popd
    fail "failed to build ${crate}"
  fi
  
  # test crate. Print the diff (empty if equal) and output a success message if actual output matches expected output.
  echo "testing ${crate}"
  # Guessing game is actually random on host, so we need to give it the expected output by hand, not running it on the host
  if [ "$crate" != "guessing_game" ]; then
    # For any crates that don't have randomness, run on native host to generate expected output
    if ! cargo run --target x86_64-unknown-linux-gnu log < "../${test_data_dir}/${crate}/input" > "../${test_data_dir}/${crate}/expected_output" 2>/dev/null; then
      # Only fail if the actual exit code is non-zero
      if [ $? -ne 0 ]; then
        echo "Error: Failed to run ${crate} on host. Exit code: $?" >&2
        echo "Current directory: $(pwd)" >&2
        echo "Command was: cargo run --target x86_64-unknown-linux-gnu log < ../test_data/${crate}/input" >&2
        fail "failed to run ${crate} on host"
      fi
    fi
  fi

  # Run on Valida VM to generate the actual output
  if ! "$vm_executable" run "./target/valida-unknown-baremetal-gnu/debug/${crate}" log < "../${crate_test_dir}/input" > "../${crate_test_dir}/actual_output"; then
    fail "failed to run ${crate} on Valida VM"
  fi

  if ! diff <(tr -d '\n' < "../${crate_test_dir}/actual_output") <(tr -d '\n' < "../${crate_test_dir}/expected_output"); then
    fail "${crate} execution test failed - outputs differ"
  fi
  echo "${crate} execution test passed"

  # Prove and verify
  echo "PROVING $crate"
  if ! "$vm_executable" prove "./target/valida-unknown-baremetal-gnu/debug/${crate}" proof < "../${crate_test_dir}/input" > "../${crate_test_dir}/actual_output"; then
    fail "failed to prove ${crate}"
  fi

  echo "VERIFYING $crate"
  if ! "$vm_executable" verify "./target/valida-unknown-baremetal-gnu/debug/${crate}" proof -o log; then
    fail "failed to verify ${crate}"
  fi

  popd
}
done