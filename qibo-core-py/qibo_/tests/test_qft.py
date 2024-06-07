import numpy as np
from qibo_.models.qft import qft
from qibo.models.qft import QFT
from qibo_core import NumpyBackend
from qibo.backends import NumpyBackend as NumpyBackend_


def test_qft():
    c = qft(5)
    cq = QFT(5)

    assert str(c) == cq.draw().replace("─", "-")


def test_qft_execution():
    c = qft(5)
    cq = QFT(5)

    result = NumpyBackend().execute_circuit(c)
    target_result = NumpyBackend_().execute_circuit(cq)
    np.testing.assert_allclose(result.state(), target_result.state())