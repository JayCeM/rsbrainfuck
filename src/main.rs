use std::env;
fn main() {
    let mut args = env::args().skip(1);
    match args.next() {
        Some(s) => {
            if let Err(e) = rsbrainfuck::run_file(s) {
                eprintln!("Fatal Error: {}", e);
            }
        }
        None => rsbrainfuck::run_interpreter(),
    };
}
