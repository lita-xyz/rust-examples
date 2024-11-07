#!/bin/bash

# configuration
test_data_dir='test_data'
crates_dir='.'
# path to the valida-vm executable
vm_executable='../12Sep/valida-toolchain/valida-vm/target/release/valida'

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

  # # check that the input nd output files exist
  # if ! test -f "${crate_test_dir}/input" || ! test -f "${crate_test_dir}/output"
  # then {
  #   echo "skipping ${crate}, missing test data"
  #   continue
  # }
  # fi

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
  diff <(tr -d '\n' < "${crate_test_dir}/actual_output") <(tr -d '\n' < "${crate_test_dir}/expected_output") && echo "${crate} test passed"
}
done
