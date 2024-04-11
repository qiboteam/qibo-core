from qibo_core import circuit, gate

c = circuit.Circuit(5)
c.add(gate.X(), [2])

print(c)
