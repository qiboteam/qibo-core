backends := "target/backends"

[private]
@backend-netcat:
  cp examples/qibo-backend-netcat {{backends}}

@backend-simple:
  cargo build --example qibo-backend-simple
  cp ./target/debug/examples/qibo-backend-simple {{backends}}


install-backends: && backend-netcat backend-simple
  @mkdir -p target/backends
