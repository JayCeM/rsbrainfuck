//! This project aims to implement an efficient brainfuck interpreter.
//!
//! # Usage
//!
//! The central piece if this interpreter is the `SourceCode` struct and its `run()` method.

use crate::memoryband::*;
use crate::sourcecode::*;
use crate::input::*;
use crate::output::*;
use std::error::Error;
use std::fs;
use std::io;

pub mod memoryband;
pub mod sourcecode;
pub mod input;
pub mod output;

pub fn run_file(s: String) -> Result<(), Box<dyn Error>> {
    let code = fs::read_to_string(s)?.parse::<SourceCode>()?;
    let mut stdin = InputBuffer::new();
    let mut stdout = StdOutput();
    code.run::<_, StdOutput, InfiniteMemoryBand>(&mut stdin, &mut stdout);
    Ok(())
}

pub fn run_interpreter() {
    println!("Welcome to the rsbrainfuck interpreter. Type 'exit' to exit the interpreter");
    let mut band = InfiniteMemoryBand::new();
    let mut stdin = InputBuffer::new();
    let mut stdout = StdOutput();
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
                code.run_on_band(&mut band, &mut stdin, &mut stdout);
            },
            Err(e) => eprintln!("{}", e),
        };
    }
}
