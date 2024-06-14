# Backend example

The easiest way to run this example is to install [`just`](https://just.systems/) and
run:

```sh
export PATH=<repo>/target/backends
just example-backend
```

## Manual run

The only requirement is to first compile the `qibo-backend-echo` (whose source is
present in this folder), and place the executable in your `$PATH`.

Then, you can simply run the example as usual, with `cargo run --example backend`.
