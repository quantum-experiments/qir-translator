// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
// This code was based on pyqir_generator::python::PyQIR
use pyqir_generator::emit::{get_ir_string, write_model_to_file};
use pyqir_generator::interop::{
    ClassicalRegister, Controlled, Instruction, Measured, QuantumRegister, Rotated, SemanticModel,
    Single,
};
use qasm::{Argument, AstNode};
use log;

use crate::qasm2qir::arguments::{Register, Pair};

pub struct QasmListener {
    pub(super) model: SemanticModel,
}

impl QasmListener {
    pub fn new(name: String) -> Self {
        QasmListener {
            model: SemanticModel::new(name),
        }
    }

    pub fn get_ir_string(&mut self) -> Result<String, String> {
        return get_ir_string(&self.model);
    }

    pub fn write_model_to_file(&mut self, file_name: String ) -> Result<(), String> {
        return write_model_to_file(&self.model, &file_name)
    }

    pub fn cx(&mut self, control: String, target: String) {
        log::info!("cx {} => {}", control, target);
        let controlled = Controlled::new(control, target);
        let inst = Instruction::Cx(controlled);
        self.model.add_inst(inst);
    }

    pub fn cz(&mut self, control: String, target: String) {
        log::info!("cz {} => {}", control, target);
        let controlled = Controlled::new(control, target);
        let inst = Instruction::Cz(controlled);
        self.model.add_inst(inst);
    }

    pub fn h(&mut self, qubit: String) {
        log::info!("h => {}", qubit);
        let single = Single::new(qubit);
        let inst = Instruction::H(single);
        self.model.add_inst(inst);
    }

    pub fn m(&mut self, qubit: String, target: String) {
        log::info!("m {}[{}]", qubit, target);
        let inst = Measured::new(qubit, target);
        let inst = Instruction::M(inst);
        self.model.add_inst(inst);
    }

    pub fn reset(&mut self, qubit: String) {
        log::info!("reset => {}", qubit);
        let single = Single::new(qubit);
        let inst = Instruction::Reset(single);
        self.model.add_inst(inst);
    }

    pub fn rx(&mut self, theta: f64, qubit: String) {
        log::info!("rx {} => {}", qubit, theta);
        let rotated = Rotated::new(theta, qubit);
        let inst = Instruction::Rx(rotated);
        self.model.add_inst(inst);
    }

    pub fn ry(&mut self, theta: f64, qubit: String) {
        log::info!("ry {} => {}", qubit, theta);
        let rotated = Rotated::new(theta, qubit);
        let inst = Instruction::Ry(rotated);
        self.model.add_inst(inst);
    }

    pub fn rz(&mut self, theta: f64, qubit: String) {
        log::info!("rz {} => {}", qubit, theta);
        let rotated = Rotated::new(theta, qubit);
        let inst = Instruction::Rz(rotated);
        self.model.add_inst(inst);
    }

    pub fn s(&mut self, qubit: String) {
        log::info!("s => {}", qubit);
        let single = Single::new(qubit);
        let inst = Instruction::S(single);
        self.model.add_inst(inst);
    }

    pub fn s_adj(&mut self, qubit: String) {
        log::info!("s_adj => {}", qubit);
        let single = Single::new(qubit);
        let inst = Instruction::SAdj(single);
        self.model.add_inst(inst);
    }

    pub fn t(&mut self, qubit: String) {
        log::info!("t => {}", qubit);
        let single = Single::new(qubit);
        let inst = Instruction::T(single);
        self.model.add_inst(inst);
    }

    pub fn t_adj(&mut self, qubit: String) {
        log::info!("t_adj => {}", qubit);
        let single = Single::new(qubit);
        let inst = Instruction::TAdj(single);
        self.model.add_inst(inst);
    }

    pub fn x(&mut self, qubit: String) {
        log::info!("x => {}", qubit);
        let single = Single::new(qubit);
        let inst = Instruction::X(single);
        self.model.add_inst(inst);
    }

    pub fn y(&mut self, qubit: String) {
        log::info!("y => {}", qubit);
        let single = Single::new(qubit);
        let inst = Instruction::Y(single);
        self.model.add_inst(inst);
    }

    pub fn dump_machine(&mut self) {
        log::info!("dump_machine");
        let inst = Instruction::DumpMachine;
        self.model.add_inst(inst);
    }

    pub fn z(&mut self, qubit: String) {
        log::info!("z => {}", qubit);
        let single = Single::new(qubit);
        let inst = Instruction::Z(single);
        self.model.add_inst(inst);
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn add_quantum_register(&mut self, name: String, size: u64) {
        let ns = name.as_str();
        for index in 0..size {
            let register_name = format!("{}[{}]", ns, index);
            log::info!("Adding {}", register_name);
            let reg = QuantumRegister {
                name: String::from(ns),
                index,
            };
            self.model.add_reg(&reg.as_register());
        }
    }

    pub fn add_classical_register(&mut self, name: String, size: u64) {
        let ns = name.clone();
        let reg = ClassicalRegister { name, size };
        log::info!("Adding {}({})", ns, size);
        self.model.add_reg(&reg.as_register());
    }

    pub fn measure(&mut self, qubit: Argument, register: Argument) {
        let qubit: Register = (&qubit).try_into().unwrap();
        let register: Register = (&register).try_into().unwrap();
        self.m(
            qubit.as_qir_name(),
            register.as_qir_name()
        );
    }

    pub fn apply_gate(&mut self, name: String, qubits: Vec<Argument>) {
        println!("{:?}", name);
    
        if name == "h" {
            let qubit: Register = (&qubits).try_into().unwrap();
            self.h(qubit.as_qir_name());
        } else if name == "CX" {
            let Pair(control, target) =
                (&qubits).try_into()
                .unwrap();
            self.cx(
                control.as_qir_name(),
                target.as_qir_name()
            );
        }
    }

    pub fn walk(&mut self, node: AstNode) {
        match node {
            AstNode::QReg(name, size) => self.add_quantum_register(name, size.try_into().unwrap()),
            AstNode::CReg(name, size) => self.add_classical_register(name, size.try_into().unwrap()),
            AstNode::ApplyGate(name, qubits, _registers) => self.apply_gate(name, qubits),
            AstNode::Measure(qubit, register) => self.measure(qubit, register),
            _other => {}
        }
    }
}
