from qibo_core import backend, circuit, gate

# To run this, you should install qibo-backend-simple in your PATH
# cf. ../../examples/backend/qibo-backend-simple.rs,
# or just run `just install-backends`, and add `target/backends` to your PATH
backend = backend.Client.spawn("simple")

X = gate.Gate.X
Y = gate.Gate.Y
H = gate.Gate.H
CNOT = gate.Gate.CNOT

c = circuit.Circuit(5)
c.add(X(), [2])
c.add(Y(), [2])
c.add(H(), [2])
c.add(CNOT(), [3, 1])
print(f"circuit:\n{c}")
res = backend.execute(c)
print(f"response: {res}")
