from qibo_.models.qft import qft
from qibo.models.qft import QFT


def test_qft():
    c = qft(5)
    cq = QFT(5)

    assert str(c) == cq.draw().replace("─", "-")
