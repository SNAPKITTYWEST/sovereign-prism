//! # sovereign-prism
//!
//! Linear type resource rules for quantum circuits.

pub mod core {
//! # utqc-core
//!
//! Circuit IR — Gate, Qubit, Circuit, Measurement.
//! Non-recursive. Every circuit compiles to a flat list of operations.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors in circuit construction or execution.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CircuitError {
    /// Qubit index out of bounds.
    #[error("qubit index {0} out of bounds (circuit has {1} qubits)")]
    QubitOutOfBounds(usize, usize),

    /// Duplicate measurement on the same qubit.
    #[error("duplicate measurement on qubit {0}")]
    DuplicateMeasurement(usize),

    /// Empty circuit.
    #[error("circuit is empty")]
    EmptyCircuit,
}

/// A single qubit identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Qubit(pub usize);

/// Single-qubit gate types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SingleGate {
    /// Pauli-X (NOT).
    PauliX,
    /// Pauli-Y.
    PauliY,
    /// Pauli-Z.
    PauliZ,
    /// Hadamard.
    Hadamard,
    /// T-gate (π/8 phase).
    TGate,
    /// S-gate (π/4 phase).
    SGate,
}

/// Two-qubit gate types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DoubleGate {
    /// Controlled-NOT.
    CNOT,
    /// Controlled-Z.
    CZ,
    /// SWAP.
    SWAP,
}

/// A gate operation in the circuit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Gate {
    /// Single-qubit gate.
    Single {
        /// Gate type.
        gate: SingleGate,
        /// Target qubit.
        target: Qubit,
    },
    /// Two-qubit gate.
    Double {
        /// Gate type.
        gate: DoubleGate,
        /// Control qubit.
        control: Qubit,
        /// Target qubit.
        target: Qubit,
    },
    /// Rotation gate (parameterized).
    Rotation {
        /// Target qubit.
        target: Qubit,
        /// Angle in radians.
        angle: f64,
    },
}

/// A measurement record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Measurement {
    /// Qubit being measured.
    pub qubit: Qubit,
    /// Classical bit index to store result.
    pub classical_bit: usize,
}

/// A quantum circuit — non-recursive flat IR.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Circuit {
    /// Number of qubits in the circuit.
    pub num_qubits: usize,
    /// Number of classical bits.
    pub num_classical_bits: usize,
    /// Ordered list of gate operations.
    pub gates: Vec<Gate>,
    /// Measurements to perform at the end.
    pub measurements: Vec<Measurement>,
}

impl Circuit {
    /// Create a new empty circuit.
    pub fn new(num_qubits: usize, num_classical_bits: usize) -> Self {
        Self {
            num_qubits,
            num_classical_bits,
            gates: Vec::new(),
            measurements: Vec::new(),
        }
    }

    /// Add a gate to the circuit.
    pub fn add_gate(&mut self, gate: Gate) -> Result<(), CircuitError> {
        match &gate {
            Gate::Single { target, .. } => {
                if target.0 >= self.num_qubits {
                    return Err(CircuitError::QubitOutOfBounds(target.0, self.num_qubits));
                }
            }
            Gate::Double { control, target, .. } => {
                if control.0 >= self.num_qubits {
                    return Err(CircuitError::QubitOutOfBounds(control.0, self.num_qubits));
                }
                if target.0 >= self.num_qubits {
                    return Err(CircuitError::QubitOutOfBounds(target.0, self.num_qubits));
                }
            }
            Gate::Rotation { target, .. } => {
                if target.0 >= self.num_qubits {
                    return Err(CircuitError::QubitOutOfBounds(target.0, self.num_qubits));
                }
            }
        }
        self.gates.push(gate);
        Ok(())
    }

    /// Add a measurement.
    pub fn add_measurement(&mut self, qubit: Qubit, classical_bit: usize) -> Result<(), CircuitError> {
        if qubit.0 >= self.num_qubits {
            return Err(CircuitError::QubitOutOfBounds(qubit.0, self.num_qubits));
        }
        if self.measurements.iter().any(|m| m.qubit == qubit) {
            return Err(CircuitError::DuplicateMeasurement(qubit.0));
        }
        self.measurements.push(Measurement { qubit, classical_bit });
        Ok(())
    }

    /// Number of gates in the circuit.
    pub fn depth(&self) -> usize {
        self.gates.len()
    }

    /// Validate the circuit.
    pub fn validate(&self) -> Result<(), CircuitError> {
        if self.gates.is_empty() && self.measurements.is_empty() {
            return Err(CircuitError::EmptyCircuit);
        }
        Ok(())
    }
}

/// The non-recursive pass trait.
pub trait Pass {
    /// Input type for this pass.
    type Input;
    /// Output type for this pass.
    type Output;

    /// Name of this pass.
    fn name(&self) -> &'static str;

    /// Execute the pass.
    fn run(&self, input: Self::Input) -> Result<Self::Output, CircuitError>;
}

}

//! # utqc-linear
//!
//! Linear type resource rules for quantum circuits.
//! enforces: lin consumed once, aff at most once, un unlimited.

use crate::core::Circuit;
use thiserror::Error;

/// Linear type resource error.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum LinearError {
    /// Qubit already consumed (linear violation).
    #[error("qubit {0} already consumed (linear violation)")]
    QubitAlreadyConsumed(usize),

    /// Qubit not available.
    #[error("qubit {0} not available in scope")]
    QubitNotAvailable(usize),

    /// Resource leak (affine type not consumed).
    #[error("resource leak: qubit {0} (affine) not consumed")]
    ResourceLeak(usize),
}

/// Resource usage kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    /// Consumed exactly once.
    Linear,
    /// Consumed at most once.
    Affine,
    /// Unrestricted reuse.
    Unrestricted,
}

/// Track resource usage per qubit.
#[derive(Debug, Clone)]
pub struct ResourceTracker {
    /// Map from qubit index to (kind, usage_count).
    resources: Vec<(ResourceKind, usize)>,
}

impl ResourceTracker {
    /// Create a new tracker for the given number of qubits.
    pub fn new(num_qubits: usize) -> Self {
        Self {
            resources: vec![(ResourceKind::Linear, 0); num_qubits],
        }
    }

    /// Mark a qubit as used.
    pub fn use_qubit(&mut self, qubit: usize) -> Result<(), LinearError> {
        if qubit >= self.resources.len() {
            return Err(LinearError::QubitNotAvailable(qubit));
        }
        let (kind, count) = &mut self.resources[qubit];
        match kind {
            ResourceKind::Linear => {
                if *count > 0 {
                    return Err(LinearError::QubitAlreadyConsumed(qubit));
                }
                *count += 1;
            }
            ResourceKind::Affine => {
                if *count > 0 {
                    return Err(LinearError::QubitAlreadyConsumed(qubit));
                }
                *count += 1;
            }
            ResourceKind::Unrestricted => {
                *count += 1;
            }
        }
        Ok(())
    }

    /// Verify no affine resources leaked.
    pub fn verify_no_leaks(&self) -> Result<(), LinearError> {
        for (i, (kind, count)) in self.resources.iter().enumerate() {
            if *kind == ResourceKind::Affine && *count == 0 {
                return Err(LinearError::ResourceLeak(i));
            }
        }
        Ok(())
    }

    /// Check linearity of a circuit.
    /// Control qubits are read-only (not consumed). Only target qubits are consumed.
    pub fn check_circuit(circuit: &Circuit) -> Result<Self, LinearError> {
        let mut tracker = Self::new(circuit.num_qubits);
        for gate in &circuit.gates {
            match gate {
                utqc_core::Gate::Single { target, .. } => {
                    tracker.use_qubit(target.0)?;
                }
                utqc_core::Gate::Double { target, .. } => {
                    // Control qubit is read-only — not consumed
                    tracker.use_qubit(target.0)?;
                }
                utqc_core::Gate::Rotation { target, .. } => {
                    tracker.use_qubit(target.0)?;
                }
            }
        }
        Ok(tracker)
    }
}

/// The linear resource check pass.
pub struct LinearCheck;

impl utqc_core::Pass for LinearCheck {
    type Input = Circuit;
    type Output = Circuit;

    fn name(&self) -> &'static str {
        "linear-resource-check"
    }

    fn run(&self, input: Circuit) -> Result<Circuit, utqc_core::CircuitError> {
        ResourceTracker::check_circuit(&input).map_err(|e| {
            utqc_core::CircuitError::QubitOutOfBounds(0, 0) // Reuse error type
        })?;
        Ok(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Circuit, Gate, Qubit, SingleGate};

    #[test]
    fn test_linear_ok() {
        let mut circuit = Circuit::new(2, 2);
        circuit.add_gate(Gate::Single { gate: SingleGate::Hadamard, target: Qubit(0) }).unwrap();
        circuit.add_gate(Gate::Double { gate: utqc_core::DoubleGate::CNOT, control: Qubit(0), target: Qubit(1) }).unwrap();
        assert!(ResourceTracker::check_circuit(&circuit).is_ok());
    }

    #[test]
    fn test_linear_violation() {
        let mut circuit = Circuit::new(1, 1);
        circuit.add_gate(Gate::Single { gate: SingleGate::Hadamard, target: Qubit(0) }).unwrap();
        circuit.add_gate(Gate::Single { gate: SingleGate::PauliX, target: Qubit(0) }).unwrap();
        assert!(ResourceTracker::check_circuit(&circuit).is_err());
    }
}

