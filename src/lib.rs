//! This project aims to implement an efficient brainfuck interpreter.
//!
//! # Usage
//!
//! The central piece if this interpreter is the `SourceCode` struct and its `run()` method.

use crate::memoryband::*;
use crate::sourcecode::*;
use std::error::Error;
use std::fs;
use std::io;

pub mod memoryband;
pub mod sourcecode;

pub fn run_file(s: String) -> Result<(), Box<dyn Error>> {
    let code = fs::read_to_string(s)?.parse::<SourceCode>()?;
    code.run();
    Ok(())
}

pub fn run_interpreter() {
    println!("Welcome to the rsbrainfuck interpreter. Type 'exit' to exit the interpreter");
    let mut band = MemoryBand::new();
    loop {
        let mut string = String::new();
        if let Err(e) = io::stdin().read_line(&mut string) {
            eprintln!("{}", e);
            continue;
        }
        if string.starts_with("exit") {
            println!("Exiting...");
            break;
        }
        
        match string.parse::<SourceCode>() {
            Ok(code) => {
                print!("[out]: ");
                code.run_on_band(&mut band);
            },
            Err(e) => eprintln!("{}", e),
        };
    }
}
