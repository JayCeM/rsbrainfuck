use std::env;
fn main() {
    println!("Hello, world!");
    let mut arr = [0, 1, 3];
    arr[1] = 5;
    println!("{:?}", arr);
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
