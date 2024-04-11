from qibo_core import circuit, gate

c = circuit.Circuit(5)
x = gate.X()
c.add(x, [2])
c.add(gate.RX(3.0), [2])

print(c)
