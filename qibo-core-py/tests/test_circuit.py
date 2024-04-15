from qibo_core import circuit, gate
import qibo
from qibo.gates import X as X_, Y as Y_, RX as RX_, CNOT as CNOT_

X = gate.Gate.X
Y = gate.Gate.Y
CNOT = gate.Gate.CNOT
RX = gate.Gate.RX


def test_draw():
    c = circuit.Circuit(5)
    c.add(X(), [2])
    c.add(RX(3.0), [2])
    c.add(CNOT(), [2, 0])
    c.add(X(), [1])
    c.add(X(), [3])
    c.add(Y(), [3])
    c.add(X(), [2])
    c.add(Y(), [4])
    c.add(CNOT(), [4, 1])
    c.add(X(), [0])

    cq = qibo.Circuit(5)
    cq.add(X_(2))
    cq.add(RX_(2, 3.0))
    cq.add(CNOT_(0, 2))
    cq.add(X_(1))
    cq.add(X_(3))
    cq.add(Y_(3))
    cq.add(X_(2))
    cq.add(Y_(4))
    cq.add(CNOT_(1, 4))
    cq.add(X_(0))

    assert str(c) == cq.draw().replace("─", "-")
