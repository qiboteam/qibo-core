#include <stdio.h>
#include <complex.h>
#include "qibo_core_c.h"
#include "microsim.h"


void print_state(complex double *state, const size_t size) {
    //const size_t size = sizeof(state) / sizeof(state[0]);
    for (size_t i=0; i < size; i++) {
        printf("%ld: %.4f, %.4f\n", i, creal(state[i]), cimag(state[i]));
    }
}


int main(int, char *[])
{
    qibo_core_circuit *c = qibo_core_circuit_new(5);
    qibo_core_circuit_add(c, "H", (size_t[]) {0}, 1);
    qibo_core_circuit_add(c, "X", (size_t[]) {2}, 1);
    qibo_core_circuit_add(c, "H", (size_t[]) {2}, 1);
    qibo_core_circuit_add(c, "X", (size_t[]) {3}, 1);
    qibo_core_circuit_add(c, "CNOT", (size_t[]) {0, 3}, 2);

    printf("%s\n\n", qibo_core_circuit_draw(c));

    // Initialize 5-qubit state vector
    complex double state[32] = { 0 };
    state[0] = 1;

    // Execute circuit
    execute_circuit(c, state);

    // Print state vector
    print_state(state, 32);

    return 0;
}
