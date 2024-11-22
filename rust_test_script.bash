#!/bin/bash

# This script tests the crates in the rust-examples directory.
# It builds each crate and runs it on the Valida VM and the x86_64-unknown-linux-gnu host, comparing the VM output to the host output.
# It skips any crates that don't have a input file in the test_data directory.
# For the guessing game crate, it is actually random on the host machine so we need to give it the expected output by hand.
# To add a new crate to the test, add a directory with the same name as the crate to the test_data directory and put the input file in it.
# If there is no input, add an empty input file.
# If the crate has randomness, you will need to give it the expected output by hand. Add the crate name to L48 below.
# Make sure that you have set the vm_executable variable to the path to the valida-vm executable (on L15 below).
# If your host is not x86_64 linux, you will need to change the target in the cargo run command on L57 below.
# Simply run the script with: bash rust_test_script.bash.

# configuration
test_data_dir='test_data'
crates_dir='.'
# path to the valida-vm executable
vm_executable='/valida-toolchain/bin/valida'

# utilities
function fail {
  echo $1;
  exit 1
}

# global sanity check
test -d "$test_data_dir" || fail "the test data directory '${test_data_dir}' does not exist, please create it and put the test input data in it"
test -f "$vm_executable" || fail "the vm executable '${vm_executable}' is not a file, please correctly specify the path to the valida-vm executable in the test script"
test -x "$vm_executable" || fail "the vm executable '${vm_executable}' is not executable, please correctly specify the path to the valida-vm executable in the test script"

for crate_test_dir in "$test_data_dir"/*
do {
  crate=$(basename "${crate_test_dir}")

  # build crate, silently to avoid polluting the output
  echo "building ${crate}"
  pushd "${crates_dir}/${crate}"
  if ! cargo +valida build --quiet
  then
    echo "failed to build ${crate}"
    popd
    continue
  fi
  popd

  # test crate. Print the diff (empty if equal) and output a success message if actual output matches expected output.
  echo "testing ${crate}"
  # Guessing game is actually random on host, so we need to give it the expected output by hand, not running it on the host
  if [ "$crate" != "guessing_game" ]; then
    # For any crates that don't have randomness, run on native host to generate expected output
    cargo run --quiet --manifest-path "${crates_dir}/${crate}/Cargo.toml" --target x86_64-unknown-linux-gnu log\
      < "${crate_test_dir}/input" > "${crate_test_dir}/expected_output"
  fi

  # Run on Valida VM to generate the actual output
  "$vm_executable" run "${crates_dir}/${crate}/target/delendum-unknown-baremetal-gnu/debug/${crate}" log \
    < "${crate_test_dir}/input" > "${crate_test_dir}/actual_output"
  diff <(tr -d '\n' < "${crate_test_dir}/actual_output") <(tr -d '\n' < "${crate_test_dir}/expected_output") && echo "${crate} execution test passed"

  # Prove and verify
  echo "PROVING $crate"
  "$vm_executable" prove "${crates_dir}/${crate}/target/delendum-unknown-baremetal-gnu/debug/${crate}" proof \
    < "${crate_test_dir}/input" > "${crate_test_dir}/actual_output"
  echo "VERIFYING $crate"
  "$vm_executable" verify "${crates_dir}/${crate}/target/delendum-unknown-baremetal-gnu/debug/${crate}" proof -o log
}
done
