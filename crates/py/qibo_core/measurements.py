import collections


def frequencies_to_binary(frequencies, nqubits):
    return collections.Counter(
        {"{:b}".format(k).zfill(nqubits): v for k, v in frequencies.items()}
    )


class MeasurementResult:
    """Data structure for holding measurement outcomes.

    :class:`qibo.measurements.MeasurementResult` objects can be obtained
    when adding measurement gates to a circuit.

    Args:
        gate (:class:`qibo.gates.M`): Measurement gate associated with
            this result object.
        nshots (int): Number of measurement shots.
        backend (:class:`qibo.backends.abstract.AbstractBackend`): Backend
            to use for calculations.
    """

    def __init__(self, gate, nshots=0, backend=None):
        self.measurement_gate = gate
        self.backend = backend
        self.nshots = nshots
        self.circuit = None

        self._samples = None
        self._frequencies = None
        self._bitflip_p0 = None
        self._bitflip_p1 = None
        self._symbols = None

    def __repr__(self):
        qubits = self.measurement_gate.qubits
        nshots = self.nshots
        return f"MeasurementResult(qubits={qubits}, nshots={nshots})"

    def add_shot(self, probs):
        qubits = sorted(self.measurement_gate.target_qubits)
        shot = self.backend.sample_shots(probs, 1)
        bshot = self.backend.samples_to_binary(shot, len(qubits))
        if self._samples:
            self._samples.append(bshot[0])
        else:
            self._samples = [bshot[0]]
        self.nshots += 1
        return shot

    def add_shot_from_sample(self, sample):
        if self._samples:
            self._samples.append(sample)
        else:
            self._samples = [sample]
        self.nshots += 1

    def has_samples(self):
        return self._samples is not None

    def register_samples(self, samples, backend=None):
        """Register samples array to the ``MeasurementResult`` object."""
        self._samples = samples
        self.nshots = len(samples)

    def register_frequencies(self, frequencies, backend=None):
        """Register frequencies to the ``MeasurementResult`` object."""
        self._frequencies = frequencies
        self.nshots = sum(frequencies.values())

    def reset(self):
        """Remove all registered samples and frequencies."""
        self._samples = None
        self._frequencies = None

    def samples(self, binary=True, registers=False):
        """Returns raw measurement samples.

        Args:
            binary (bool): Return samples in binary or decimal form.
            registers (bool): Group samples according to registers.

        Returns:
            If `binary` is `True`
                samples are returned in binary form as a tensor
                of shape `(nshots, n_measured_qubits)`.
            If `binary` is `False`
                samples are returned in decimal form as a tensor
                of shape `(nshots,)`.
        """
        if self._samples is None:
            if self.circuit is None:
                raise RuntimeError(
                    "Cannot calculate samples if circuit is not provided."
                )
            # calculate samples for the whole circuit so that
            # individual register samples are registered here
            self.circuit.final_state.samples()
        if binary:
            return self.backend.cast(self._samples, dtype="int32")
        else:
            qubits = self.measurement_gate.target_qubits
            return self.backend.samples_to_decimal(self._samples, len(qubits))

    def frequencies(self, binary=True, registers=False):
        """Returns the frequencies of measured samples.

        Args:
            binary (bool): Return frequency keys in binary or decimal form.
            registers (bool): Group frequencies according to registers.

        Returns:
            A `collections.Counter` where the keys are the observed values
            and the values the corresponding frequencies, that is the number
            of times each measured value/bitstring appears.

            If `binary` is `True`
                the keys of the `Counter` are in binary form, as strings of
                0s and 1s.
            If `binary` is `False`
                the keys of the `Counter` are integers.
        """
        if self._frequencies is None:
            self._frequencies = self.backend.calculate_frequencies(
                self.samples(binary=False)
            )
        if binary:
            qubits = self.measurement_gate.target_qubits
            return frequencies_to_binary(self._frequencies, len(qubits))
        else:
            return self._frequencies

    def apply_bitflips(self, p0, p1=None):  # pragma: no cover
        return apply_bitflips(self, p0, p1)
