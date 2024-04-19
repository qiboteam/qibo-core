backends := "target/backends"

[private]
@backend-netcat:
  cp examples/backend/qibo-backend-netcat {{backends}}

[private]
@backend-simple:
  cargo build --example qibo-backend-simple
  cp ./target/debug/examples/qibo-backend-simple {{backends}}


install-backends: && backend-netcat backend-simple
  @mkdir -p target/backends

example-backend: install-backends
  cargo run --example backend
