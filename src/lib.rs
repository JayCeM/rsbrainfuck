//! This project aims to implement an efficient brainfuck interpreter.
//!
//! # Usage
//!
//! The central piece if this interpreter is the `SourceCode` struct and its `run()` method.

use crate::memoryband::*;
use crate::sourcecode::*;
use crate::input::*;
use crate::output::*;
use crate::args::*;
use std::error::Error;
use std::fs;
use std::io;

pub mod memoryband;
pub mod sourcecode;
pub mod input;
pub mod output;
pub mod args;

fn run_file<M>(args: Args) -> Result<(), Box<dyn Error>> 
    where M: MemoryBand {
    let code = fs::read_to_string(args.input_path.into_os_string())?.parse::<SourceCode>()?;
    let mut stdin = InputBuffer::new();
    let mut stdout = StdOutput();
    code.run::<_, StdOutput, M>(&mut stdin, &mut stdout);
    Ok(())
}

fn run_interpreter<M>() 
    where M: MemoryBand {
    println!("Welcome to the rsbrainfuck interpreter. Type 'exit' to exit the interpreter");
    let mut band = M::new();
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
                code.run_on_band::<_,_,M>(&mut band, &mut stdin, &mut stdout);
            },
            Err(e) => eprintln!("{}", e),
        };
    }
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    if args.interactive {
        if args.infinite_memory {
            run_interpreter::<InfiniteMemoryBand>();
        } else {
            run_interpreter::<FiniteMemoryBand>();
        }
        Ok(())
    } else {
        if args.infinite_memory {
            run_file::<InfiniteMemoryBand>(args)
        } else {
            run_file::<FiniteMemoryBand>(args)
        }
    }
}

