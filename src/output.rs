pub trait Output {
    fn write_char(&mut self, c: char);
}

pub struct StdOutput();

impl Output for StdOutput {
    fn write_char(&mut self, c: char) {
        print!("{}", c);
    }
}

impl Output for Vec<char> {
    fn write_char(&mut self, c: char) {
        self.push(c);
    }
}
