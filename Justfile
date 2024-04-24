backends := "target/backends"

[private]
@backend-netcat:
  cp examples/backend/qibo-backend-netcat {{backends}}

[private]
@backend-echo:
  cargo build --example qibo-backend-echo
  cp ./target/debug/examples/qibo-backend-echo {{backends}}


install-backends: && backend-netcat backend-echo
  @mkdir -p target/backends

example-backend: install-backends
  cargo run --example backend
