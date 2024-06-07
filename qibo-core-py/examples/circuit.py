from qibo_core import Circuit, gates


c = Circuit(5)
c.add(gates.X(), [2])
c.add(gates.RX(3.0), [2])
c.add(gates.CNOT(), [0, 2])
c.add(gates.X(), [1])
c.add(gates.X(), [3])
c.add(gates.Y(), [3])
c.add(gates.X(), [2])
c.add(gates.Y(), [4])
c.add(gates.CNOT(), [4, 1])
c.add(gates.X(), [0])

print(c)
print()
for gate, targets in zip(*c.queue):
    print(gate, targets)
