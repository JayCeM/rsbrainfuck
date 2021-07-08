# About this project

This is a rust implementation of a brainfuck interpreter. The program performs some optimisations to the read code before execution,
tho there is lots of room for further improvements.

# Installation

1. clone this repository: `git clone <URL OF THIS REPO>`
2. build the project using [cargo](https://www.rust-lang.org/tools/install): `cargo build --release`

# Modes of operation

see `rsbrainfuck --help` for further information

## interactive

In interactive environment, you can execute single lines of brainfuck code one after another.

## non-interactive

In non-interactive mode, the interpreter expects you to provide a valid path to a file containing your brainfuck code.
The file will be read and the code will be executed. After that, the interpreter terminates.

# Information on the underlying model

Internally, the interpreter operates on a memoryband of 8bit cells, so the brainfuck program can operate on 8bit unsigned integer values per memory cell.
Integer under-/overflows are inentionally not caught.

Per default the interpreter uses a memoryband of width 30,000 cells and starts at index 15,000.
Using the flag `-m` you can tell the interpreter to use a dynamically sized memoryband, giving virtually infinite memoryband size,
tho this comes at a performance hit.

# License

This project is licensed under GPL v3. For further information see the [LICENSE](LICENSE) file.
