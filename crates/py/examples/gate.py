from qibo_core import gate

# waiting for PyO3 v0.22
# https://github.com/PyO3/pyo3/issues/3900
# https://github.com/PyO3/pyo3/issues/759
X = gate.Gate.X
RX = gate.Gate.RX

print([X() if i % 2 == 0 else RX(i) for i in range(7)])
