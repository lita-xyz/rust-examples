# Prove a JsonPath contains a value

## Usage

### Run with Cargo
```bash
$ echo '{"path":"$.foo.bar","expected":{"baz":1},"json":{"foo":{"bar":{"baz":1}}}}' | cargo run
path: $.foo.bar
contains json: {"baz":1}
```

### Prove In Valida zkVM
```bash
$ echo '{"path":"$.foo.bar","expected":{"baz":1},"json":{"foo":{"bar":{"baz":1}}}}' | valida prove target/delendum-unknown-baremetal-gnu/debug/json_contains log
path: $.foo.bar
contains json: {"baz":1}
```
