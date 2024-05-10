import math

import qibo_core

gates = qibo_core.gate.Gate
Circuit = qibo_core.circuit.Circuit


def qft(nqubits, with_swaps=True) -> Circuit:
    circuit = Circuit(nqubits)
    for i1 in range(nqubits):
        circuit.add(gates.H(), [i1])
        for i2 in range(i1 + 1, nqubits):
            theta = math.pi / 2 ** (i2 - i1)
            circuit.add(gates.CU1(theta), [i1, i2])

    if with_swaps:
        for i in range(nqubits // 2):
            circuit.add(gates.SWAP(), [i, nqubits - i - 1])

    return circuit
