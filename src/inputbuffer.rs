use std::io::*;
//use std::str::Chars;
use std::collections::VecDeque;

pub struct InputBuffer {
    buffer: VecDeque<char>,
    stdin: Stdin,
}

impl InputBuffer {
    pub fn new() -> InputBuffer {
        let buffer = VecDeque::new();
        InputBuffer {
            buffer,
            stdin: stdin(),
        }
    }

}

impl Iterator for InputBuffer {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.buffer.pop_front() {
            Some(c) => Some(c),
            None => {
                let mut buf_str = String::new();
                println!("\nType in your input:");
                if let Err(_) =  self.stdin.read_line(&mut buf_str) {
                    return None;
                }
                self.buffer = buf_str.chars().collect();
                self.buffer.pop_front()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_next() {
        let mut buf = InputBuffer {
            buffer: vec!['a', 'b'].into_iter().collect(),
            stdin: stdin(),
        };
        assert_eq!(buf.next(), Some('a'));
        assert_eq!(buf.next(), Some('b'));
    }
}
