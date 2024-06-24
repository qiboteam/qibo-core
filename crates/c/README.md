# C API

Currently, there is no distribution of pre-compiled binaries for the C API.

However, you can install it from source following the instructions reported hereafter.

## How to use it

To compile and use the C library it is advised to install the following tools:

- the standard `Makefile`
- the `pkg-config` utility
- the [`cargo-c`](https://github.com/lu-zero/cargo-c) extension for Cargo

in addition to what is already required to compile the main `qibo-core` library.

### Instructions

Compile and install the library itself with:

```sh
cargo cinstall --prefix $(realpath prefix)`
```

(in this guide we are installing the library in a folder named `prefix` in the `$PWD`,
but you can change this value with any other valid path).

Now move into the `examples/` folder in here.

To compile an example, just run:

```sh
# from inside `examples/`
make my-example
```

and then run the compiled example with the following:

```sh
# from inside `examples/`
LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(realpath ../prefix/lib/) ./my-example
# and on MacOS
LD_DYLD_PATH=$LD_DYLD_PATH:$(realpath ../prefix/lib/) ./my-example
```

#### `pkg-config`

To use a custom prefix, change your `PKG_CONFIG_PATH` variable, appending the your
prefix location:

```sh
# from inside `examples/`
PKG_CONFIG_PATH=$PKG_CONFIG_PATH:$(realpath ../prefix/lib/pkgconfig) make my-example
```

and remove the line exporting this variable in the `Makefile`.

#### Justfile

For people making use of [`just`](https://just.systems/), a `Justfile` is provided. Examples could be run using:

```sh
just example my-example
```

## What's next

We are working to provide reliable internal dependency, and a suitable package for
third-party to depend on.

However, for the time being, just ignore files related to other build systems (e.g.
Meson, Buck2, ...), as they are fully experimental.
