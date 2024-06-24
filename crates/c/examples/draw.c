#include <stdio.h>
#include <complex.h>
#include "qibo_core_c.h"


int main(int argc, char *argv[])
{
    qibo_core_circuit *c = qibo_core_circuit_new(5);
    qibo_core_circuit_add(c, "H", (size_t[]) {0}, 1);
    qibo_core_circuit_add(c, "X", (size_t[]) {2}, 1);
    qibo_core_circuit_add(c, "H", (size_t[]) {2}, 1);
    qibo_core_circuit_add(c, "X", (size_t[]) {3}, 1);
    qibo_core_circuit_add(c, "CNOT", (size_t[]) {0, 3}, 2);

    printf("%s\n", qibo_core_circuit_draw(c));

    return 0;
}
