// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
extern crate qasm;
extern crate pyqir_generator;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use qasm::{process, lex, parse};
use qir_translator::qasm2qir::listener::QasmListener;
use log;

fn main() {
    log::info!("Test");
    let cwd = env::current_dir().unwrap();
    let mut source = String::new();

    let mut f = File::open("test.qasm").expect("cannot find source file 'test.qasm'");
    f.read_to_string(&mut source).expect("couldn't read file 'test.qasm'");

    let processed_source = process(&source, &cwd);
    let mut tokens = lex(&processed_source);
    let ast = parse(&mut tokens);

    let mut listener = QasmListener::new("test".to_string());

    // Simple example:
    // listener.add_quantum_register("q".to_string(), 2);
    // listener.add_classical_register("c".to_string(), 1);
    // listener.h("q0".to_string());
    // listener.cx("q0".to_string(), "q1".to_string());
    // listener.m("q1".to_string(), "c0".to_string());

    // println!("Emitted QIR:");
    // println!("{:?}", listener.get_ir_string());

    println!("Contents of ast:");
    for nodes in ast.iter() {
        for node in nodes.iter() {
            println!("> {:?}", node);
            listener.walk(node.to_owned());
        }
    }
    
    // println!("Emitted QIR:");
    // println!("{:?}", listener.get_ir_string());
    let file_name = "test.ll";
    let result = listener.write_model_to_file(file_name.to_string());
    println!("{:?}", result);
}
