from qibo_core import Circuit, gates, NumpyBackend


c = Circuit(5)
c.add(gates.X(), [2])
c.add(gates.RX(3.0), [2])
c.add(gates.X(), [0, 2])
c.add(gates.X(), [1])
c.add(gates.X(), [3])
c.add(gates.Y(), [3])
c.add(gates.X(), [2])
c.add(gates.Y(), [4])
c.add(gates.X(), [4, 1])
c.add(gates.X(), [0])

print(c)

backend = NumpyBackend()
result = backend.execute_circuit(c)
print()
print(result)
