import collections
import warnings
from typing import Optional, Union

import numpy as np

from .qibo_core import __version__
from .qibo_core import gate

gates = gate.Gate


def load_result(filename: str):
    """Loads the results of a circuit execution saved to disk.

    Args:
        filename (str): Path to the file containing the results.

    Returns:
        :class:`qibo.result.QuantumState` or :class:`qibo.result.MeasurementOutcomes` or :class:`qibo.result.CircuitResult`: result of circuit execution saved to disk, depending on saved filed.
    """
    payload = np.load(filename, allow_pickle=True).item()
    return globals()[payload.pop("dtype")].from_dict(payload)


class QuantumState:
    """Data structure to represent the final state after circuit execution.

    Args:
        state (np.ndarray): Input quantum state as np.ndarray.
        backend (qibo.backends.AbstractBackend): Backend used for the calculations. If not provided the :class:`qibo.backends.GlobalBackend` is going to be used.
    """

    def __init__(self, state, backend=None):
        from . import backends

        self.backend = backends._check_backend(backend)
        self.density_matrix = len(state.shape) == 2
        self.nqubits = int(np.log2(state.shape[0]))
        self._state = state

    def symbolic(self, decimals: int = 5, cutoff: float = 1e-10, max_terms: int = 20):
        """Dirac notation representation of the state in the computational
        basis.

        Args:
            decimals (int, optional): Number of decimals for the amplitudes.
                Defaults to :math:`5`.
            cutoff (float, optional): Amplitudes with absolute value smaller than the
                cutoff are ignored from the representation. Defaults to  ``1e-10``.
            max_terms (int, optional): Maximum number of terms to print. If the state
                contains more terms they will be ignored. Defaults to :math:`20`.

        Returns:
            (str): A string representing the state in the computational basis.
        """
        if self.density_matrix:
            terms = self.backend.calculate_symbolic_density_matrix(
                self._state, self.nqubits, decimals, cutoff, max_terms
            )
        else:
            terms = self.backend.calculate_symbolic(
                self._state, self.nqubits, decimals, cutoff, max_terms
            )
        return " + ".join(terms)

    def state(self, numpy: bool = False):
        """State's tensor representation as a backend tensor.

        Args:
            numpy (bool, optional): If ``True`` the returned tensor will be a ``numpy`` array,
                otherwise it will follow the backend tensor type.
                Defaults to ``False``.

        Returns:
            The state in the computational basis.
        """
        if numpy:
            return np.array(self._state.tolist())

        return self._state

    def probabilities(self, qubits: Optional[Union[list, set]] = None):
        """Calculates measurement probabilities by tracing out qubits.

        When noisy model is applied to a circuit and `circuit.density_matrix=False`,
        this method returns the average probability resulting from
        repeated execution. This probability distribution approximates the
        exact probability distribution obtained when `circuit.density_matrix=True`.

        Args:
            qubits (list or set, optional): Set of qubits that are measured.
                If ``None``, ``qubits`` equates the total number of qubits.
                Defauts to ``None``.
        Returns:
            (np.ndarray): Probabilities over the input qubits.
        """

        if qubits is None:
            qubits = tuple(range(self.nqubits))

        if self.density_matrix:
            return self.backend.calculate_probabilities_density_matrix(
                self._state, qubits, self.nqubits
            )

        return self.backend.calculate_probabilities(self._state, qubits, self.nqubits)

    def __str__(self):
        return self.symbolic()

    def to_dict(self):
        """Returns a dictonary containinig all the information needed to
        rebuild the ``QuantumState``"""
        return {
            "state": self.state(numpy=True),
            "dtype": self.__class__.__name__,
            "qibo": __version__,
        }

    def dump(self, filename: str):
        """Writes to file the ``QuantumState`` for future reloading.

        Args:
            filename (str): Path to the file to write to.
        """
        with open(filename, "wb") as f:
            np.save(f, self.to_dict())

    @classmethod
    def from_dict(cls, payload: dict):
        """Builds a ``QuantumState`` object starting from a dictionary.

        Args:
            payload (dict): Dictionary containing all the information
                to load the ``QuantumState`` object.

        Returns:
            :class:`qibo.result.QuantumState`: Quantum state object..
        """
        from . import backends

        backend = backends.construct_backend("numpy")
        return cls(payload.get("state"), backend=backend)

    @classmethod
    def load(cls, filename: str):
        """Builds the ``QuantumState`` object stored in a file.

        Args:
            filename (str): Path to the file containing the ``QuantumState``.

        Returns:
            :class:`qibo.result.QuantumState`: Quantum state object.
        """
        payload = np.load(filename, allow_pickle=True).item()
        return cls.from_dict(payload)


def frequencies_to_binary(frequencies, nqubits):
    return collections.Counter(
        {"{:b}".format(k).zfill(nqubits): v for k, v in frequencies.items()}
    )


class MeasurementOutcomes:
    """Object to store the outcomes of measurements after circuit execution.

    Args:
        measurements (:class:`qibo.gates.M`): Measurement gates.
        backend (:class:`qibo.backends.AbstractBackend`): Backend used for the calculations.
            If ``None``, then :class:`qibo.backends.GlobalBackend` is used. Defaults to ``None``.
        probabilities (np.ndarray): Use these probabilities to generate samples and frequencies.
        samples (np.darray): Use these samples to generate probabilities and frequencies.
        nshots (int): Number of shots used for samples, probabilities and frequencies generation.
    """

    def __init__(
        self,
        samples: Optional[int] = None,
        frequencies: Optional[dict] = None,
    ):
        self._samples = samples
        self._frequencies = frequencies

    @property
    def frequencies(self):
        """Frequencies of measured samples."""
        if self._frequencies is None:
            nelements = len(self.samples[0])
            results, counts = np.unique(self.samples, axis=0, return_counts=True)
            self._frequencies = {"".join(str(i) for i in r): c for r, c in zip(results.astype(int), counts)}
        return self._frequencies

    @property
    def samples(self):
        """Raw measurement samples."""
        if self._samples is None:
            # generate samples that respect the existing frequencies
            samples = []
            for bitstring, counts in self.frequencies.items():
                samples.extend(counts * [[int(b) for b in bitstring]])
            np.random.shuffle(samples)
            self._samples = np.array(samples, dtype="int32")
        return self._samples

    def to_dict(self):
        """Returns a dictonary containinig all the information needed to
        rebuild the :class:`qibo.result.MeasurementOutcomes`."""
        args = {
            "samples": self.samples,
            "frequencies": self.frequencies,
            "dtype": self.__class__.__name__,
            "qibo": __version__,
        }
        return args

    def dump(self, filename: str):
        """Writes to file the :class:`qibo.result.MeasurementOutcomes` for
        future reloading.

        Args:
            filename (str): Path to the file to write to.
        """
        with open(filename, "wb") as f:
            np.save(f, self.to_dict())

    @classmethod
    def from_dict(cls, payload: dict):
        """Builds a :class:`qibo.result.MeasurementOutcomes` object starting
        from a dictionary.

        Args:
            payload (dict): Dictionary containing all the information to load the :class:`qibo.result.MeasurementOutcomes` object.

        Returns:
            A :class:`qibo.result.MeasurementOutcomes` object.
        """
        from . import backends

        if payload["frequencies"] is not None and payload["samples"] is not None:
            warnings.warn(
                "Both `frequencies` and `samples` found, discarding the `frequencies` and building out of the `samples`."
            )
            payload.pop("frequencies")
        return cls(
            samples=payload.get("samples"),
            frequencies=payload.get("frequencies"),
        )

    @classmethod
    def load(cls, filename: str):
        """Builds the :class:`qibo.result.MeasurementOutcomes` object stored in
        a file.

        Args:
            filename (str): Path to the file containing the :class:`qibo.result.MeasurementOutcomes`.

        Returns:
            A :class:`qibo.result.MeasurementOutcomes` object.
        """
        payload = np.load(filename, allow_pickle=True).item()
        return cls.from_dict(payload)


class CircuitResult(QuantumState):
    """Object to store both the outcomes of measurements and the final state
    after circuit execution.

    Args:
        final_state (np.ndarray): Input quantum state as np.ndarray.
        measurements (qibo.gates.M): The measurement gates containing the measurements.
        backend (qibo.backends.AbstractBackend): Backend used for the calculations. If not provided the :class:`qibo.backends.GlobalBackend` is going to be used.
        probabilities (np.ndarray): Use these probabilities to generate samples and frequencies.
        samples (np.darray): Use these samples to generate probabilities and frequencies.
        nshots (int): Number of shots used for samples, probabilities and frequencies generation.
    """

    def __init__(self, state, elements, nshots=1000, outcomes=None, backend=None):
        super().__init__(state, backend)
        if len(elements) == 0:
            raise ValueError(
                "Circuit does not contain measurements. Use a `QuantumState` instead."
            )
        self.elements = elements
        self.nshots = nshots
        self.outcomes = outcomes

    @property
    def samples(self):
        """Raw measurement samples."""
        if self.outcomes is None:
            probs = self.probabilities(self.elements)
            samples = self.backend.sample_shots(probs, self.nshots)
            samples = self.backend.samples_to_binary(samples, len(self.elements))
            self.outcomes = MeasurementOutcomes(samples=samples)
        return self.outcomes.samples


    @property
    def frequencies(self):
        """Frequencies of measured samples."""
        if self.outcomes is None:
            probs = self.probabilities(self.elements)
            frequencies = self.backend.sample_frequencies(probs, self.nshots)
            frequencies = frequencies_to_binary(frequencies, len(self.elements))
            self.outcomes = MeasurementOutcomes(frequencies=frequencies)
        return self.outcomes.frequencies

    def to_dict(self):
        """Returns a dictonary containinig all the information needed to
        rebuild the ``CircuitResult``."""
        args = super().to_dict()
        args["elements"] = self.elements
        args["nshots"] = self.nshots
        if self.outcomes is not None:
            args["outcomes"] = self.outcomes.to_dict()
        args["dtype"] = self.__class__.__name__
        return args

    @classmethod
    def from_dict(cls, payload: dict):
        """Builds a ``CircuitResult`` object starting from a dictionary.

        Args:
            payload (dict): Dictionary containing all the information to load the ``CircuitResult`` object.

        Returns:
            :class:`qibo.result.CircuitResult`: circuit result object.
        """
        state_load = {"state": payload.pop("state")}
        state = QuantumState.from_dict(state_load)
        if "outcomes" in payload:
            payload["outcomes"] = MeasurementOutcomes.from_dict(**payload.pop("outcomes"))
        return cls(
            state.state(),
            **payload
        )
