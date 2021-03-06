// Copyright © 2021 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.
//
//! Collection of roqoqo PRAGMA operations.
//!

use ndarray::{array, Array1, Array2};
use num_complex::Complex64;
use qoqo_calculator::{Calculator, CalculatorFloat};
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::operations::{
    InvolveQubits, InvolvedQubits, Operate, OperateMultiQubit, OperatePragma, OperatePragmaNoise,
    OperateSingleQubit, RoqoqoError, Substitute,
};
use crate::Circuit;

/// This PRAGMA Operation sets the number of measurements of the circuit.
///
/// This is used for backends that allow setting the number of tries. However, setting the number of
/// measurements does not allow access to the underlying wavefunction or density matrix.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct PragmaSetNumberOfMeasurements {
    /// The number of measurements.
    number_measurements: usize,
    /// The register for the readout.
    readout: String,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaSetNumberOfMeasurements: &[&str; 3] = &[
    "Operation",
    "PragmaOperation",
    "PragmaSetNumberOfMeasurements",
];

// Implementing the InvolveQubits trait for PragmaSetNumberOfMeasurements.
impl InvolveQubits for PragmaSetNumberOfMeasurements {
    /// Lists all involved qubits (here, none).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}

/// This PRAGMA Operation sets the statevector of a quantum register.
///
/// The Circuit() module automatically initializes the qubits in the |0> state, so this PRAGMA
/// operation allows you to set the state of the qubits to a state of your choosing.
///
/// # Example
///
/// For instance, to initialize the $|\Psi^->$ Bell state, we pass the following `statevec` to
/// the PragmaSetStateVector operation.
///
/// ```
/// use ndarray::{array, Array1};
/// use num_complex::Complex64;
/// use roqoqo::operations::PragmaSetStateVector;
///
/// let statevec: Array1<Complex64> = array![
///     Complex64::new(0.0, 0.0),
///     Complex64::new(1.0 / (2.0_f64).sqrt(), 0.0),
///     Complex64::new(-1.0 / (2.0_f64).sqrt(), 0.0),
///     Complex64::new(0.0, 0.0)
/// ];
///
/// let pragma = PragmaSetStateVector::new(statevec.clone());
/// ```
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaSetStateVector {
    /// The statevector that is initialized.
    statevector: Array1<Complex64>,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaSetStateVector: &[&str; 3] =
    &["Operation", "PragmaOperation", "PragmaSetStateVector"];

// Implementing the InvolveQubits trait for PragmaSetStateVector.
impl InvolveQubits for PragmaSetStateVector {
    /// Lists all involved qubits (here, all).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }
}

/// This PRAGMA Operation sets the density matrix of a quantum register.
///
/// The Circuit() module automatically initializes the qubits in the |0> state, so this PRAGMA
/// operation allows you to set the state of the qubits by setting a density matrix of your choosing.
///
/// # Example
///
/// ```
/// use ndarray::{array, Array2};
/// use num_complex::Complex64;
/// use roqoqo::operations::PragmaSetDensityMatrix;
///
/// let matrix: Array2<Complex64> = array![
///    [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
///    [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
/// ];
///
/// let pragma = PragmaSetDensityMatrix::new(matrix.clone());
/// ```
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaSetDensityMatrix {
    /// The density matrix that is initialized.
    density_matrix: Array2<Complex64>,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaSetDensityMatrix: &[&str; 3] =
    &["Operation", "PragmaOperation", "PragmaSetDensityMatrix"];

// Implementing the InvolveQubits trait for PragmaSetDensityMatrix.
impl InvolveQubits for PragmaSetDensityMatrix {
    /// Lists all involved qubits (here, all).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }
}

/// The repeated gate PRAGMA operation.
///
/// This PRAGMA Operation repeats the next gate in the circuit the given number of times to increase the rate for error mitigation.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaRepeatGate {
    /// The number of times the following gate is repeated.
    repetition_coefficient: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaRepeatGate: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaRepeatGate"];

// Implementing the InvolveQubits trait for PragmaRepeatGate.
impl InvolveQubits for PragmaRepeatGate {
    /// Lists all involved qubits (here, all).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }
}

/// The statistical overrotation PRAGMA operation.
///
/// This PRAGMA applies a statistical overrotation to the next rotation gate in the circuit, which
/// matches the hqslang name in the `gate` parameter of PragmaOverrotation and the involved qubits in `qubits`.
///
/// The applied overrotation corresponds to adding a random number to the rotation angle.
/// The random number is drawn from a normal distribution with mean `0`
/// and standard deviation `variance` and is multiplied by the `amplitude`.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::OperatePragma,
    roqoqo_derive::OperateMultiQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "overrotate")]
pub struct PragmaOverrotation {
    /// The unique hqslang name of the gate to overrotate.
    gate_hqslang: String,
    /// The qubits of the gate to overrotate.
    qubits: Vec<usize>,
    /// The amplitude the random number is multiplied by.
    amplitude: f64,
    /// The standard deviation of the normal distribution the random number is drawn from.
    variance: f64,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaOverrotation: &[&str; 4] = &[
    "Operation",
    "MultiQubitOperation",
    "PragmaOperation",
    "PragmaOverrotation",
];

/// This PRAGMA Operation boosts noise and overrotations in the circuit.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaBoostNoise {
    /// The coefficient by which the noise is boosted, i.e. the number by which the gate time is multiplied.
    noise_coefficient: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaBoostNoise: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaBoostNoise"];

// Implementing the InvolveQubits trait for PragmaBoostNoise.
impl InvolveQubits for PragmaBoostNoise {
    /// Lists all involved qubits (here, none).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}

/// This PRAGMA Operation signals the STOP of a parallel execution block.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateMultiQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaStopParallelBlock {
    /// The qubits involved in parallel execution block.
    qubits: Vec<usize>,
    /// The time for the execution of the block in seconds.
    execution_time: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaStopParallelBlock: &[&str; 4] = &[
    "Operation",
    "MultiQubitOperation",
    "PragmaOperation",
    "PragmaStopParallelBlock",
];

/// The global phase PRAGMA operation.
///
/// This PRAGMA Operation signals that the quantum register picks up a global phase,
/// i.e. it provides information that there is a global phase to be considered.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaGlobalPhase {
    /// The picked up global phase.
    phase: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaGlobalPhase: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaGlobalPhase"];

// Implementing the InvolveQubits trait for PragmaGlobalPhase.
impl InvolveQubits for PragmaGlobalPhase {
    /// Lists all involved qubits (here, none).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}

/// This PRAGMA Operation makes the quantum hardware wait a given amount of time.
///
/// This PRAGMA Operation is used for error mitigation reasons, for instance.
/// It can be used to boost the noise on the qubits since it gets worse with time.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateMultiQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaSleep {
    /// The qubits involved in the sleep block.
    qubits: Vec<usize>,
    /// Time for the execution of the operation in seconds.
    sleep_time: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaSleep: &[&str; 4] = &[
    "Operation",
    "MultiQubitOperation",
    "PragmaOperation",
    "PragmaSleep",
];

/// This PRAGMA Operation resets the chosen qubit to the zero state.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaActiveReset {
    /// The qubit to be reset.
    qubit: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaActiveReset: &[&str; 4] = &[
    "Operation",
    "SingleQubitOperation",
    "PragmaOperation",
    "PragmaActiveReset",
];

/// This PRAGMA Operation signals the START of a decomposition block.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::OperateMultiQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaStartDecompositionBlock {
    /// The qubits involved in the decomposition block.
    qubits: Vec<usize>,
    /// The reordering dictionary of the block.
    reordering_dictionary: HashMap<usize, usize>,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaStartDecompositionBlock: &[&str; 4] = &[
    "Operation",
    "MultiQubitOperation",
    "PragmaOperation",
    "PragmaStartDecompositionBlock",
];

/// Substitute trait allowing to replace symbolic parameters and to perform qubit mappings.
impl Substitute for PragmaStartDecompositionBlock {
    /// Remaps qubits in clone of the operation.
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        let mut new_qubits: Vec<usize> = Vec::new();
        for q in &self.qubits {
            new_qubits.push(*mapping.get(q).ok_or(Err("")).map_err(
                |_x: std::result::Result<&usize, &str>| RoqoqoError::QubitMappingError {
                    qubit: *q,
                },
            )?)
        }

        let mut mutable_reordering: HashMap<usize, usize> = HashMap::new();
        for (old_qubit, new_qubit) in self.reordering_dictionary.clone() {
            let old_remapped = *mapping.get(&old_qubit).ok_or(Err("")).map_err(
                |_x: std::result::Result<&usize, &str>| RoqoqoError::QubitMappingError {
                    qubit: old_qubit,
                },
            )?;
            let new_remapped = *mapping.get(&new_qubit).ok_or(Err("")).map_err(
                |_x: std::result::Result<&usize, &str>| RoqoqoError::QubitMappingError {
                    qubit: new_qubit,
                },
            )?;
            mutable_reordering.insert(old_remapped, new_remapped);
        }

        Ok(PragmaStartDecompositionBlock::new(
            new_qubits,
            mutable_reordering,
        ))
    }

    /// Substitutes symbolic parameters in clone of the operation.
    fn substitute_parameters(&self, _calculator: &mut Calculator) -> Result<Self, RoqoqoError> {
        Ok(self.clone())
    }
}

/// This PRAGMA Operation signals the STOP of a decomposition block.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateMultiQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaStopDecompositionBlock {
    /// The qubits involved in the decomposition block.
    qubits: Vec<usize>,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaStopDecompositionBlock: &[&str; 4] = &[
    "Operation",
    "MultiQubitOperation",
    "PragmaOperation",
    "PragmaStopDecompositionBlock",
];

/// The damping PRAGMA noise Operation.
///
/// This PRAGMA Operation applies a pure damping error corresponding to zero temperature environments.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaDamping {
    /// The qubit on which to apply the damping.
    qubit: usize,
    /// The time (in seconds) the gate takes to be applied to the qubit on the (simulated) hardware
    gate_time: CalculatorFloat,
    /// The error rate of the damping (in 1/second).
    rate: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaDamping: &[&str; 5] = &[
    "Operation",
    "SingleQubitOperation",
    "PragmaOperation",
    "PragmaNoiseOperation",
    "PragmaDamping",
];

/// OperatePragmaNoise trait creating necessary functions for a PRAGMA noise Operation.
impl OperatePragmaNoise for PragmaDamping {
    /// Returns the superoperator matrix of the operation.
    fn superoperator(&self) -> Result<Array2<f64>, RoqoqoError> {
        let gate_time: f64 = f64::try_from(self.gate_time.clone())?;
        let rate: f64 = f64::try_from(self.rate.clone())?;

        let pre_exp: f64 = -1.0 * gate_time * rate;
        let prob: f64 = 1.0 - pre_exp.exp();
        let sqrt: f64 = (1.0 - prob).sqrt();

        Ok(array![
            [1.0, 0.0, 0.0, prob],
            [0.0, sqrt, 0.0, 0.0],
            [0.0, 0.0, sqrt, 0.0],
            [0.0, 0.0, 0.0, 1.0 - prob],
        ])
    }

    /// Returns the probability of the noise gate affecting the qubit, based on its `gate_time` and `rate`.
    fn probability(&self) -> CalculatorFloat {
        let prob: CalculatorFloat =
            ((self.gate_time.clone() * self.rate.clone() * (-2.0)).exp() * (-1.0) + 1.0) * 0.5;
        prob
    }

    /// Returns the gate to the power of `power`.
    fn powercf(&self, power: CalculatorFloat) -> Self {
        let mut new = self.clone();
        new.gate_time = power * self.gate_time.clone();
        new
    }
}

/// The depolarising PRAGMA noise Operation.
///
/// This PRAGMA Operation applies a depolarising error corresponding to infinite temperature environments.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaDepolarising {
    /// The qubit on which to apply the depolarising.
    qubit: usize,
    /// The time (in seconds) the gate takes to be applied to the qubit on the (simulated) hardware
    gate_time: CalculatorFloat,
    /// The error rate of the depolarisation (in 1/second).
    rate: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaDepolarising: &[&str; 5] = &[
    "Operation",
    "SingleQubitOperation",
    "PragmaOperation",
    "PragmaNoiseOperation",
    "PragmaDepolarising",
];

/// OperatePragmaNoise trait creating necessary functions for a PRAGMA noise Operation.
impl OperatePragmaNoise for PragmaDepolarising {
    /// Returns the superoperator matrix of the operation.
    fn superoperator(&self) -> Result<Array2<f64>, RoqoqoError> {
        let gate_time: f64 = f64::try_from(self.gate_time.clone())?;
        let rate: f64 = f64::try_from(self.rate.clone())?;

        let pre_exp: f64 = -1.0 * gate_time * rate;
        let prob: f64 = (3.0 / 4.0) * (1.0 - pre_exp.exp());
        let proba1: f64 = 1.0 - (2.0 / 3.0) * prob;
        let proba2: f64 = 1.0 - (4.0 / 3.0) * prob;
        let proba3: f64 = (2.0 / 3.0) * prob;

        Ok(array![
            [proba1, 0.0, 0.0, proba3],
            [0.0, proba2, 0.0, 0.0],
            [0.0, 0.0, proba2, 0.0],
            [proba3, 0.0, 0.0, proba1],
        ])
    }

    /// Returns the probability of the noise gate affecting the qubit, based on its `gate_time` and `rate`.
    fn probability(&self) -> CalculatorFloat {
        let prob: CalculatorFloat =
            ((self.gate_time.clone() * self.rate.clone() * (-1.0)).exp() * (-1.0) + 1.0) * 0.75;
        prob
    }

    /// Returns the gate to the power of `power`.
    fn powercf(&self, power: CalculatorFloat) -> Self {
        let mut new = self.clone();
        new.gate_time = power * self.gate_time.clone();
        new
    }
}

/// The dephasing PRAGMA noise Operation.
///
/// This PRAGMA Operation applies a pure dephasing error.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaDephasing {
    /// The qubit on which to apply the dephasing.
    qubit: usize,
    /// The time (in seconds) the gate takes to be applied to the qubit on the (simulated) hardware
    gate_time: CalculatorFloat,
    /// The error rate of the dephasing (in 1/second).
    rate: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaDephasing: &[&str; 5] = &[
    "Operation",
    "SingleQubitOperation",
    "PragmaOperation",
    "PragmaNoiseOperation",
    "PragmaDephasing",
];

/// OperatePragmaNoise trait creating necessary functions for a PRAGMA noise Operation.
impl OperatePragmaNoise for PragmaDephasing {
    /// Returns the superoperator matrix of the operation.
    fn superoperator(&self) -> Result<Array2<f64>, RoqoqoError> {
        let gate_time: f64 = f64::try_from(self.gate_time.clone())?;
        let rate: f64 = f64::try_from(self.rate.clone())?;

        let pre_exp: f64 = -2.0 * gate_time * rate;
        let prob: f64 = (1.0 / 2.0) * (1.0 - pre_exp.exp());

        Ok(array![
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0 - 2.0 * prob, 0.0, 0.0],
            [0.0, 0.0, 1.0 - 2.0 * prob, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Returns the probability of the noise gate affecting the qubit, based on its `gate_time` and `rate`.
    fn probability(&self) -> CalculatorFloat {
        let prob: CalculatorFloat =
            ((self.gate_time.clone() * self.rate.clone() * (-2.0)).exp() * (-1.0) + 1.0) * 0.5;
        prob
    }

    /// Returns the gate to the power of `power`.
    fn powercf(&self, power: CalculatorFloat) -> Self {
        let mut new = self.clone();
        new.gate_time = power * self.gate_time.clone();
        new
    }
}

/// The random noise PRAGMA operation.
///
/// This PRAGMA Operation applies a stochastically unravelled combination of dephasing and depolarising.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaRandomNoise {
    /// The qubit the PRAGMA Operation is applied to.
    qubit: usize,
    /// The time (in seconds) the gate takes to be applied to the qubit on the (simulated) hardware
    gate_time: CalculatorFloat,
    /// The error rate of the depolarisation (in 1/second).
    depolarising_rate: CalculatorFloat,
    /// The error rate of the dephasing (in 1/second).
    dephasing_rate: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaRandomNoise: &[&str; 5] = &[
    "Operation",
    "SingleQubitOperation",
    "PragmaOperation",
    "PragmaNoiseOperation",
    "PragmaRandomNoise",
];

/// OperatePragmaNoise trait creating necessary functions for a PRAGMA noise Operation.
impl OperatePragmaNoise for PragmaRandomNoise {
    /// Returns the superoperator matrix of the operation. For the RandomNoise pragma, the superoperator
    /// is the effective superoperator after averaging over many trajectories: the dephasing superoperator.
    fn superoperator(&self) -> Result<Array2<f64>, RoqoqoError> {
        let gate_time: f64 = f64::try_from(self.gate_time.clone())?;
        let rate: f64 = f64::try_from(self.dephasing_rate.clone())?;

        let pre_exp: f64 = -2.0 * gate_time * rate;
        let prob: f64 = (1.0 / 2.0) * (1.0 - pre_exp.exp());

        Ok(array![
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0 - 2.0 * prob, 0.0, 0.0],
            [0.0, 0.0, 1.0 - 2.0 * prob, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Returns the probability of the noise gate affecting the qubit, based on its `gate_time`, `depolarising_rate` and `dephasing_rate`.
    fn probability(&self) -> CalculatorFloat {
        let rates = [
            self.depolarising_rate.clone() / 4.0,
            self.depolarising_rate.clone() / 4.0,
            (self.depolarising_rate.clone() / 4.0) + self.dephasing_rate.clone(),
        ];
        (rates[0].clone() + &rates[1] + &rates[2]) * &self.gate_time
    }

    /// Returns the gate to the power of `power`.
    fn powercf(&self, power: CalculatorFloat) -> Self {
        let mut new = self.clone();
        new.gate_time = power * self.gate_time.clone();
        new
    }
}

/// The general noise PRAGMA operation.
///
/// This PRAGMA Operation applies a noise term according to the given operators.
/// The operators are represented by a 3x3 matrix:
/// $$ M = \begin{pmatrix}
/// a & b & c \\\\
/// d & e & f \\\\
/// g & h & j \\\\
/// \end{pmatrix} $$
/// where the coefficients correspond to the following summands
/// expanded from the first term of the non-coherent part of the Lindblad equation:
/// $$
/// a \cdot \sigma_x \rho \sigma_x + b \cdot \sigma_x \rho \sigma_y + c \cdot \sigma_x \rho \sigma_z + d \cdot \sigma_x \rho \sigma_y + e \cdot \sigma_y \rho \sigma_y + f \cdot \sigma_y \rho \sigma_z + g \cdot \sigma_x \rho \sigma_z + h \cdot \sigma_y \rho \sigma_z + j \cdot \sigma_z \rho \sigma_z.
/// $$
///
/// # Example
///
/// ```
/// use ndarray::{array, Array2};
/// use num_complex::Complex64;
/// use roqoqo::operations::PragmaGeneralNoise;
/// use qoqo_calculator::CalculatorFloat;
///
/// let operators: Array2<Complex64> = array![
///    [
///         Complex64::new(1.0, 0.0),
///         Complex64::new(0.0, 0.0),
///         Complex64::new(0.0, 0.0)
///     ],
///     [
///         Complex64::new(0.0, 0.0),
///         Complex64::new(1.0, 0.0),
///         Complex64::new(0.0, 0.0)
///     ],
///     [
///         Complex64::new(0.0, 0.0),
///         Complex64::new(0.0, 0.0),
///         Complex64::new(1.0, 0.0)
///     ],
/// ];
/// let pragma = PragmaGeneralNoise::new(
///     0,
///     CalculatorFloat::from(0.005),
///     CalculatorFloat::from(0.02),
///     operators.clone(),
/// );
/// ```
/// That will result into $\sigma_x \rho \sigma_x + \sigma_y \rho \sigma_y + \sigma_z \rho \sigma_z$.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaGeneralNoise {
    /// The qubit the PRAGMA Operation is applied to.
    qubit: usize,
    /// The time (in seconds) the gate takes to be applied to the qubit on the (simulated) hardware
    gate_time: CalculatorFloat,
    /// The error rate of the noise (in 1/second).
    rate: CalculatorFloat,
    /// The operators representing the general noise (a 3x3 matrix).
    operators: Array2<Complex64>,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaGeneralNoise: &[&str; 4] = &[
    "Operation",
    "SingleQubitOperation",
    "PragmaOperation",
    "PragmaGeneralNoise",
];

/// The conditional PRAGMA operation.
///
/// This PRAGMA executes a circuit when the condition bit/bool stored in a [crate::registers::BitRegister] is true.
///
#[derive(Debug, Clone, PartialEq, roqoqo_derive::Operate, roqoqo_derive::OperatePragma)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaConditional {
    /// The name of the [crate::registers::BitRegister] containting the condition bool value.
    condition_register: String,
    /// The index in the [crate::registers::BitRegister] containting the condition bool value.
    condition_index: usize,
    /// The circuit executed if the condition is met.
    circuit: Circuit,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaConditional: &[&str; 4] = &[
    "Operation",
    "SingleQubitOperation",
    "PragmaOperation",
    "PragmaConditional",
];

// Implementing the InvolveQubits trait for PragmaConditional.
impl InvolveQubits for PragmaConditional {
    /// Lists all involved qubits.
    fn involved_qubits(&self) -> InvolvedQubits {
        self.circuit.involved_qubits()
    }
}

/// Substitute trait allowing to replace symbolic parameters and to perform qubit mappings.
impl Substitute for PragmaConditional {
    /// Remaps qubits in clone of the operation.
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        let new_circuit = self.circuit.remap_qubits(mapping).unwrap();
        Ok(PragmaConditional::new(
            self.condition_register.clone(),
            self.condition_index,
            new_circuit,
        ))
    }

    /// Substitutes symbolic parameters in clone of the operation.
    fn substitute_parameters(&self, calculator: &mut Calculator) -> Result<Self, RoqoqoError> {
        let new_circuit = self.circuit.substitute_parameters(calculator).unwrap();
        Ok(PragmaConditional::new(
            self.condition_register.clone(),
            self.condition_index,
            new_circuit,
        ))
    }
}
