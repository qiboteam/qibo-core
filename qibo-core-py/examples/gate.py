from qibo_core import Gate

X = lambda: Gate("X")
Y = lambda: Gate("Y")

print([X() if i % 2 == 0 else Y() for i in range(7)])
