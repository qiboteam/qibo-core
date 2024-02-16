import importlib.metadata as im

__version__ = im.version(__package__)

from . import gates, result
from qibo.backends import *
from qibo.config import *
from qibo.models.circuit import Circuit
