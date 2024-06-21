export PKG_CONFIG_PATH=/home/stavros/qiboteam/qibo-core/crates/c/prefix/lib/x86_64-linux-gnu/pkgconfig
export LD_LIBRARY_PATH=/home/stavros/qiboteam/qibo-core/crates/c/prefix/lib/x86_64-linux-gnu

cc circuit.c -o circuit -I/home/stavros/qiboteam/qibo-core/crates/c/prefix/include/qibo_core_c -L/home/stavros/qiboteam/qibo-core/crates/c/prefix/lib/x86_64-linux-gnu -lqibo_core_c
