from qibo_core import circuit, gate

X = gate.Gate.X
Y = gate.Gate.Y
CNOT = gate.Gate.CNOT
RX = gate.Gate.RX

c = circuit.Circuit(5)
c.add(X(), [2])
c.add(RX(3.0), [2])
c.add(CNOT(), [0, 2])
c.add(X(), [1])
c.add(X(), [3])
c.add(Y(), [3])
c.add(X(), [2])
c.add(Y(), [4])
c.add(CNOT(), [1, 4])
c.add(X(), [0])

print(c)
