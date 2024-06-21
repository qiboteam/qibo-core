#include <stdio.h>
#include <complex.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>
#include "qibo_core_c.h"
#include "microsim.h"


typedef struct {
    complex double h[4];
    complex double x[4];
    complex double y[4];
    complex double z[4];
} Matrices;


Matrices create() {
    double const h = 1.0 / sqrt(2);
    Matrices const m = {
        {h, h, h, -h},
        {0, 1, 1, 0},
        {0, -I, I, 0},
        {1, 0, 0, -1}
    };
    return m;
}

int compare(const void *a, const void *b) {
    return (*(int*)a - *(int*)b);
}

size_t control_index(size_t const g, size_t const *qubits, size_t const nqubits) {
    size_t i = g;
    for (size_t j = 0; j < nqubits; j++) {
        size_t const n = qubits[j];
        size_t const k = 1 << n;
        i = ((i >> n) << (n + 1)) + (i & (k - 1)) + k;
    }
    return i;
}

void apply_controlled_gate(
    complex double *state, 
    complex double const *gate, 
    size_t const *qubits,
    size_t const ncontrols,
    size_t const nqubits
) {
    size_t target = qubits[0];
    
    size_t sorted_qubits[ncontrols + 1];
    memcpy(sorted_qubits, qubits, (ncontrols + 1) * sizeof(size_t));
    for (size_t i = 0; i < ncontrols + 1; i++) {
        sorted_qubits[i] = nqubits - sorted_qubits[i] - 1;
    }
    qsort(sorted_qubits, ncontrols + 1, sizeof(size_t), compare);

    size_t const nstates = 1 << (nqubits - ncontrols - 1);
    size_t const tk = 1 << (nqubits - target - 1);
    // TODO: This can be parallelized for large number of qubits
    for (size_t g = 0; g < nstates; g++) {
        size_t const i2 = control_index(g, sorted_qubits, ncontrols + 1);
        size_t const i1 = i2 - tk;
        complex double const state1 = state[i1];
        complex double const state2 = state[i2];
        state[i1] = gate[0] * state1 + gate[1] * state2;
        state[i2] = gate[2] * state1 + gate[3] * state2;
    }
}

void execute_circuit(qibo_core_circuit *circuit, complex double *state) {
	Matrices gates = create();

	size_t const n_elements = qibo_core_circuit_n_elements(circuit);
	size_t const n_gates = qibo_core_circuit_n_gates(circuit);

	for (size_t gid = 0; gid < n_gates; gid++) {
		char const *gate = qibo_core_circuit_gate(circuit, gid);
		complex double *matrix;
        // get matrix corresponding to the gate
		if (strcmp(gate, "H") == 0) {
			matrix = gates.h;
		} else if (strcmp(gate, "Y") == 0) {
			matrix = gates.y;
		} else if (strcmp(gate, "Z") == 0) {
			matrix = gates.z;
		} else {
			matrix = gates.x;
		}
		
        // calculate number of control qubits
        // maybe can be improved if we expose gate n_elements to C API
        // but this will change anyway when we implement ``controlled_by``
		size_t n_controls = 0;
		if (strcmp(gate, "CNOT") == 0) {
			n_controls = 1;
		}

		size_t const *elements;
		size_t n_gate_elements;
		qibo_core_circuit_elements(circuit, gid, &elements, &n_gate_elements);

		apply_controlled_gate(state, matrix, elements, n_controls, n_elements);
		
		printf("\n%ld %ld %s ", n_gate_elements, n_controls, gate);
		for (size_t i = 0; i < n_gate_elements; i++) {
			printf("%ld ", elements[i]);
		}
		qibo_core_circuit_free_elements(elements, n_gate_elements);
	}
	printf("\n");
}
