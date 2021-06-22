use rsbrainfuck::args::*;
//use std::env;
use std::error::Error;
use structopt::StructOpt;
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();
    rsbrainfuck::run(args)
}

