import numpy as np
from qibo_core import circuit, gate, NumpyBackend
import qibo
from qibo.gates import X as X_, Y as Y_, RX as RX_, CNOT as CNOT_
from qibo.backends import NumpyBackend as NumpyBackend_

X = gate.Gate.X
Y = gate.Gate.Y
CNOT = gate.Gate.CNOT
RX = gate.Gate.RX

def qibo_circuit():
    cq = qibo.Circuit(5)
    cq.add(X_(2))
    cq.add(RX_(2, 3.0))
    cq.add(CNOT_(0, 2))
    cq.add(X_(1))
    cq.add(X_(3))
    cq.add(Y_(3))
    cq.add(X_(2))
    cq.add(Y_(4))
    cq.add(CNOT_(4, 1))
    cq.add(X_(0))
    return cq


def qibo_core_circuit():
    c = circuit.Circuit(5)
    c.add(X(), [2])
    c.add(RX(3.0), [2])
    c.add(CNOT(), [2, 0])
    c.add(X(), [1])
    c.add(X(), [3])
    c.add(Y(), [3])
    c.add(X(), [2])
    c.add(Y(), [4])
    c.add(CNOT(), [1, 4])
    c.add(X(), [0])
    return c


def test_draw():
    c = qibo_core_circuit()
    cq = qibo_circuit()
    assert str(c) == cq.draw().replace("─", "-")


def test_execute():
    c = qibo_core_circuit()
    cq = qibo_circuit()
    result = NumpyBackend().execute_circuit(c)
    target_result = NumpyBackend_().execute_circuit(cq)
    np.testing.assert_allclose(result.state(), target_result.state())