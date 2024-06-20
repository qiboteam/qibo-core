import os

import numpy as np

from .numpy import NumpyBackend


class MetaBackend:
    """Meta-backend class which takes care of loading the qibo backends."""

    @staticmethod
    def load(backend: str, **kwargs):
        """Loads the native qibo backend."""
        return NumpyBackend()

    def list_available(self) -> dict:
        """Lists all the available native qibo backends."""
        return {"numpy": True}


class GlobalBackend(NumpyBackend):
    """The global backend will be used as default by ``circuit.execute()``."""

    _instance = None
    _dtypes = {"double": "complex128", "single": "complex64"}
    _default_order = [
        {"backend": "qibojit", "platform": "cupy"},
        {"backend": "qibojit", "platform": "numba"},
        {"backend": "tensorflow"},
        {"backend": "numpy"},
        {"backend": "pytorch"},
    ]

    def __new__(cls):
        if cls._instance is not None:
            return cls._instance

        backend = os.environ.get("QIBO_BACKEND")
        if backend:  # pragma: no cover
            # Create backend specified by user
            platform = os.environ.get("QIBO_PLATFORM")
            cls._instance = construct_backend(backend, platform=platform)
        else:
            # Create backend according to default order
            for kwargs in cls._default_order:
                try:
                    cls._instance = construct_backend(**kwargs)
                    break
                except (ModuleNotFoundError, ImportError):
                    pass

        if cls._instance is None:  # pragma: no cover
            raise RuntimeError("No backends available.")

        return cls._instance

    @classmethod
    def set_backend(cls, backend, **kwargs):  # pragma: no cover
        if (
            cls._instance is None
            or cls._instance.name != backend
            or cls._instance.platform != kwargs.get("platform")
        ):
            cls._instance = construct_backend(backend, **kwargs)


def get_backend():
    return str(GlobalBackend())


def set_backend(backend, **kwargs):
    GlobalBackend.set_backend(backend, **kwargs)


def get_device():
    return GlobalBackend().device


def set_device(device):
    parts = device[1:].split(":")
    if device[0] != "/" or len(parts) < 2 or len(parts) > 3:
        raise ValueError(
            "Device name should follow the pattern: /{device type}:{device number}.",
        )
    backend = GlobalBackend()
    backend.set_device(device)


def get_threads():
    return GlobalBackend().nthreads


def set_threads(nthreads):
    if not isinstance(nthreads, int):
        raise TypeError("Number of threads must be integer.")
    if nthreads < 1:
        raise ValueError("Number of threads must be positive.")
    GlobalBackend().set_threads(nthreads)


def _check_backend(backend):
    if backend is None:
        return GlobalBackend()

    return backend


def list_available_backends() -> dict:
    """Lists all the backends that are available."""
    return MetaBackend().list_available()


def construct_backend(backend, **kwargs):
    """Construct a generic native or non-native qibo backend.

    Args:
        backend (str): Name of the backend to load.
        kwargs (dict): Additional arguments for constructing the backend.
    Returns:
        qibo.backends.abstract.Backend: The loaded backend.
    """
    return MetaBackend.load(backend, **kwargs)


def _check_backend_and_local_state(seed, backend):
    if (
        seed is not None
        and not isinstance(seed, int)
        and not isinstance(seed, np.random.Generator)
    ):
        raise TypeError("seed must be either type int or numpy.random.Generator.")

    backend = _check_backend(backend)

    if seed is None or isinstance(seed, int):
        if backend.__class__.__name__ in [
            "CupyBackend",
            "CuQuantumBackend",
        ]:  # pragma: no cover
            local_state = backend.np.random.default_rng(seed)
        else:
            local_state = np.random.default_rng(seed)
    else:
        local_state = seed

    return backend, local_state
