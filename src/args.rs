use std::path::PathBuf;
use structopt::StructOpt;
#[derive(StructOpt)]
pub struct Args {
    /// use an virtually infinite memory band, comes at a performance loss
    #[structopt(short = "m", long)]
    pub infinite_memory: bool,

    /// enter interactive environment
    #[structopt(short, long)]
    pub interactive: bool,

    /// path to brainfuck source code file
    #[structopt(parse(from_os_str), default_value(""))]
    pub input_path: PathBuf,
}
