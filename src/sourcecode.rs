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
        let mut is_bracket_open = false;

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
                        commands.push(Add(d - 1));
                    }
                    _ => commands.push(Add(-1)),
                },

                '.' => commands.push(Print),
                ',' => commands.push(Read),

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
}
