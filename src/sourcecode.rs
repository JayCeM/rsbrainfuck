use super::memoryband::*;
use super::input::Input;
//use char_stream::CharStream;
use std::str::FromStr;
use BfCommand::*;

/// The variants of this enum each represent a brainfuck command.
#[derive(Debug, PartialEq)]
enum BfCommand {
    Move(isize),
    Add(u8),
    Print,
    Read,
    Loop(SourceCode),
}

/// This struct is created mainly using its `FromStr` implementation, e.g. by invoking
/// `from_str(s)` or `s.parse()`. Use `code.run()` to run the SourceCode.
/// The Sourcecode can be executed using the [`SourceCode::run()`] or [`SourceCode::run_on_band()`] methods.
/// One Specialty of these functions is that they accept every Iterator over [`char`] as their
/// StdIn, use [`super::inputbuffer::InputBuffer`] for the standard StdIn-behavior.
#[derive(Debug, PartialEq)]
pub struct SourceCode(Vec<BfCommand>);

impl SourceCode {
    /// Runs the brainfuck source code on an empty memoryband.
    ///
    /// Pass any Iterator over [`char`] as stdin to the method, 
    /// use [`super::inputbuffer::InputBuffer`] for the standard StdIn-behavior.
    pub fn run<I, M>(&self, stdin: &mut I)
    where I: Input,
          M: MemoryBand {
        let mut band = M::new();
        self.run_on_band(&mut band, stdin);
    }

    /// Runs the brainfuck source code on the given `band` memoryband.
    ///
    /// Pass any Iterator over [`char`] as stdin to the method, 
    /// use [`super::inputbuffer::InputBuffer`] for the standard StdIn-behavior.
    pub fn run_on_band<I,M>(&self, band: &mut M, stdin: &mut I)
    where I: Input,
          M: MemoryBand {
        self.run_loop_band(band, stdin);
        println!("");
    }

    /// Runs the brainfuck source code on the given `band` memoryband.
    /// This method got outsourced form [run_on_band`] because taht method needs to print one
    /// newline symbol at the end of the computaton.
    /// Leaving the newline out caused problems where the stdout would be presented delayed to the
    /// user.
    fn run_loop_band<I,M>(&self, band: &mut M, stdin: &mut I)
    where I: Input,
          M: MemoryBand {
        for c in self.0.iter() {
            match c {
                Move(i) => band.move_head(*i),
                Add(i) => band.add(*i),
                Print => print!("{}", band.read() as char),
                Read => {
                        match stdin.read_char() {
                        Some(c) => band.write(c as u8),
                        None => band.write(0),
                    }
                },
                Loop(code) => {
                    while band.read() != 0 {
                        code.run_loop_band(band, stdin);
                    }
                }
            }
        }
    }
}

/// s: string slice to find the brackets in
/// start_index: index of the opening bracket
fn find_matching_closing_bracket(s: &str, start_index: usize) -> Result<usize, String> {
    let mut iter = s.char_indices().skip(start_index);
    match iter.next() {
        Some((_, '[')) => (),
        _ => return Err(format!("No opening bracket at index {}.", start_index)),
    };
    let mut count: usize = 1;
    for (i, c) in iter {
        match c {
            '[' => count += 1,
            ']' => count -= 1,
            _ => continue,
        }
        if count == 0 {
            return Ok(i);
        }
    }
    Err(format!(
        "No matching bracket was found for '[' at position {}.",
        start_index
    ))
}

/// s: string slice to extract the loop from
/// start_index: index of the opening bracket
/// output: OK: the extracted loop Sourcecode and a length of the underlying string segment
fn extract_loop_code(s: &str, start_index: usize) -> Result<(SourceCode, usize), String> {
    let mut iter = s.chars().skip(start_index);
    match iter.next() {
        Some('[') => (),
        _ => return Err(format!("No opening bracket at index {}.", start_index)),
    };
    let close_index = find_matching_closing_bracket(s, start_index)?;
    Ok((SourceCode::from_str(&s[start_index + 1..close_index])?, close_index - start_index))
}

impl FromStr for SourceCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut commands = Vec::new();

        let mut iter = s.char_indices();

        // remove Move(0), Add(0) ?
        while let Some((i, c)) = iter.next() {
            match c {
                '>' => match commands.last() {
                    Some(&Move(d)) => {
                        commands.pop();
                        commands.push(Move(d + 1));
                    }
                    _ => commands.push(Move(1)),
                },
                '<' => match commands.last() {
                    Some(&Move(d)) => {
                        commands.pop();
                        commands.push(Move(d - 1));
                    }
                    _ => commands.push(Move(-1)),
                },

                '+' => match commands.last() {
                    Some(&Add(d)) => {
                        commands.pop();
                        commands.push(Add(d + 1));
                    }
                    _ => commands.push(Add(1)),
                },
                '-' => match commands.last() {
                    Some(&Add(d)) => {
                        commands.pop();
                        commands.push(Add(d.overflowing_sub(1).0));
                    }
                    _ => commands.push(Add(u8::MAX)), // u8::MAX <=> -1
                },

                '.' => commands.push(Print),
                ',' => commands.push(Read),

                '[' => {
                    let (loop_code, len) = extract_loop_code(s, i)?;
                    for _ in 0..len {
                        iter.next();
                    }
                    commands.push(Loop(loop_code));
                }
                ']' => {
                    return Err(format!(
                        "No matching bracket was found for ']' at position {}.",
                        i
                    ))
                }

                _ => continue,
            }
        }

        Ok(SourceCode(commands))
    }
}

#[cfg(test)]
mod test {
    const NEG1: u8 = u8::MAX;
    use super::*;
    #[test]
    fn test_from_str() {
        let source = "+-->><<<+++---";
        let code = source.parse::<SourceCode>();

        let expected = Ok(SourceCode(vec![Add(NEG1), Move(-1), Add(0)]));

        assert_eq!(code, expected);
    }

    #[test]
    fn test_from_str_err() {
        let code = "<<+++\nD\n+++".parse::<SourceCode>();

        let expected = "<<++++++".parse::<SourceCode>();

        assert_eq!(code, expected);
    }

    #[test]
    fn test_find_matching_closing_bracket() {
        let text = "0[23]5[[8]][";
        assert_eq!(find_matching_closing_bracket(text, 0), Err(String::from("No opening bracket at index 0.")));
        assert_eq!(find_matching_closing_bracket(text, 1), Ok(4));
        assert_eq!(find_matching_closing_bracket(text, 6), Ok(10));
        assert_eq!(find_matching_closing_bracket(text, 7), Ok(9));
        assert_eq!(find_matching_closing_bracket(text, 11), Err(String::from("No matching bracket was found for '[' at position 11.")));
    }
    
    #[test]
    fn test_ectract_loop_code() {
        let code = ".[.[+ ] ].".parse::<SourceCode>();

        let expected: Result<SourceCode, String> = Ok(SourceCode(
                vec![
                Print,
                Loop(SourceCode(
                        vec![
                        Print,
                        Loop(SourceCode(
                                vec![
                                Add(1)]))
                        ])),
                Print]));

        assert_eq!(code, expected);
    }

    #[test]
    fn test_loop_no_opening_bracket() {
        let code = "<<+++].".parse::<SourceCode>();

        let expected = Err(String::from(
            "No matching bracket was found for ']' at position 5.",
        ));

        assert_eq!(code, expected);
    }
    #[test]
    fn test_loop_no_closing_bracket() {
        let code = "<<+++[.".parse::<SourceCode>();

        let expected = Err(String::from(
            "No matching bracket was found for '[' at position 5.",
        ));

        assert_eq!(code, expected);
    }
    #[test]
    fn test_from_str_loop() {
        let code = "+[--]+".parse::<SourceCode>();

        let expected = Ok(SourceCode(vec![
            Add(1),
            Loop(SourceCode(vec![Add(NEG1-1)])),
            Add(1),
        ]));

        assert_eq!(code, expected);
    }

    #[test]
    fn test_from_str_many_loops() {
        let code1 = "+[--]++[--]+".parse::<SourceCode>();

        let expected1 = Ok(SourceCode(vec![
            Add(1),
            Loop(SourceCode(vec![Add(NEG1-1)])),
            Add(2),
            Loop(SourceCode(vec![Add(NEG1-1)])),
            Add(1),
        ]));

        let code2 = "+[-[.-]+].".parse::<SourceCode>();

        let expected2 = Ok(SourceCode(vec![
                                      Add(1),
                                      Loop(SourceCode(vec![
                                                      Add(NEG1),
                                                      Loop(SourceCode(vec![
                                                                      Print,
                                                                      Add(NEG1),
                                                      ])),
                                                      Add(1)
                                      ])),
                                      Print]));


        assert_eq!(code1, expected1);
        assert_eq!(code2, expected2);
    }

    #[test]
    fn test_from_str_empty_loop() {
        let code = "[]".parse::<SourceCode>();

        let expected = Ok(SourceCode(vec![Loop(SourceCode(vec![]))]));

        assert_eq!(code, expected);
    }
}
