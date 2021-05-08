use std::str::FromStr;
use BfCommand::*;

#[derive(Debug, PartialEq)]
pub struct SourceCode(Vec<BfCommand>);

#[derive(Debug, PartialEq)]
pub enum BfCommand {
    Move(isize),
    Add(i64),
    Print,
    Read,
    Loop(SourceCode),
}

impl FromStr for SourceCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //let mut is_bracket_open = false;

        let mut commands = Vec::new();

        let mut iter = s.char_indices();

        // start_index: index of the opening bracket
        let find_matching_closing_bracket = |start_index| {
            let mut count: usize = 1;
            let mut iter = s.char_indices().skip(start_index + 1);
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
        };

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
                        commands.push(Add(d - 1));
                    }
                    _ => commands.push(Add(-1)),
                },

                '.' => commands.push(Print),
                ',' => commands.push(Read),

                '[' => {
                    let i_close = find_matching_closing_bracket(i)?;
                    println!("loop: from {} to {}", i + 1, i_close);
                    let LoopCode = Self::from_str(&s[i + 1..i_close])?;
                    for _ in 0..LoopCode.0.len() + 2 {
                        iter.next();
                    }
                    commands.push(Loop(LoopCode));
                }
                ']' => {
                    return Err(format!(
                        "No matching bracket was found for ']' at position {}.",
                        i
                    ))
                }

                _ => {
                    // omitt instead of Err ?
                    return Err(format!(
                        "character '{}' at index {} is not a valid brainfuck command",
                        c, i
                    ));
                }
            }
        }

        Ok(SourceCode(commands))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_from_str() {
        let source = "+-->><<<+++---";
        let code = source.parse::<SourceCode>();

        let expected = Ok(SourceCode(vec![Add(-1), Move(-1), Add(0)]));

        assert_eq!(code, expected);
    }

    #[test]
    fn test_from_str_err() {
        let source = "<<+++D+++";
        let code = source.parse::<SourceCode>();

        let expected = Err(String::from(
            "character 'D' at index 5 is not a valid brainfuck command",
        ));

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
            Loop(SourceCode(vec![Add(-2)])),
            Add(1),
        ]));

        assert_eq!(code, expected);
    }

    #[test]
    fn test_from_str_empty_loop() {
        let code = "[]".parse::<SourceCode>();

        let expected = Ok(SourceCode(vec![Loop(SourceCode(vec![]))]));

        assert_eq!(code, expected);
    }
}
