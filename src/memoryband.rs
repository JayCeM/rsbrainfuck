use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
/// This struct implements a memoryband that has an unlimited amount of memory cells to the left and
/// right. Each memory cell holds a value of type `u8`.
/// The memoryband has one reading head that can be moved left or right. It can read and write to
/// the memory cell below it.
/// Each memory cell is initialized to `0`.
pub struct MemoryBand {
    band: VecDeque<u8>,
    current_index: usize,
}

impl MemoryBand {
    /// Creates a new Memoryband instance
    pub fn new() -> MemoryBand {
        MemoryBand {
            band: vec![0].into_iter().collect(),
            current_index: 0,
        }
    }

    /// Outputs the value that is currently readable
    pub fn read(&self) -> u8 {
        self.band[self.current_index]
    }

    /// Writes `int` to the current cell
    pub fn write(&mut self, int: u8) {
        self.band[self.current_index] = int;
    }

    /// Adds `int` to the current cell
    pub fn add(&mut self, int: u8) {
        self.band[self.current_index] = self.band[self.current_index].overflowing_add(int).0;
    }

    /// Moves the reading head right by `moves` amount
    fn move_right(&mut self, moves: usize) {
        self.current_index += moves;

        if self.current_index >= self.band.len() {
            let offset = self.current_index - self.band.len() + 1;
            for _ in 0..offset {
                self.band.push_back(0);
            }
        }
    }

    /// Moves the reading head left by `moves` amount
    fn move_left(&mut self, moves: usize) {
        if self.current_index == 0 {
            for _ in 0..moves {
                self.band.push_front(0);
            }
            return;
        } else if moves > self.current_index {
            let moves_later = moves - self.current_index;
            self.current_index = 0;
            return self.move_left(moves_later);
        }
        self.current_index -= moves;
    }

    /// Moves the reading head left by `moves` amount.
    /// Positive values correspond to moving right, negative values to moving left.
    pub fn move_head(&mut self, moves: isize) {
        use std::cmp::Ordering::*;
        match moves.cmp(&0) {
            Less => self.move_left(-moves as usize),
            Equal => (),
            Greater => self.move_right(moves as usize),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new() {
        let band = MemoryBand::new();
        let expected = MemoryBand {
            band: vec![0].into_iter().collect(),
            current_index: 0,
        };
        assert_eq!(band, expected);
    }

    #[test]
    fn test_read() {
        let band = MemoryBand {
            band: vec![1, 2].into_iter().collect(),
            current_index: 1,
        };
        assert_eq!(2, band.read());
    }

    #[test]
    fn test_write() {
        let mut band = MemoryBand {
            band: vec![1, 2].into_iter().collect(),
            current_index: 1,
        };
        band.write(-5);

        let expected = MemoryBand {
            band: vec![1, -5].into_iter().collect(),
            current_index: 1,
        };

        assert_eq!(band, expected);
    }

    #[test]
    fn test_add() {
        let mut band = MemoryBand::new();
        band.add(-1);

        let expected = MemoryBand {
            band: vec![-1].into_iter().collect(),
            current_index: 0,
        };

        assert_eq!(band, expected);
    }

    #[test]
    fn test_move_right() {
        let mut band = MemoryBand::new();
        band.write(4);
        band.move_right(2);

        let expected = MemoryBand {
            band: vec![4, 0, 0].into_iter().collect(),
            current_index: 2,
        };
        assert_eq!(band, expected);
    }

    #[test]
    fn test_move_left() {
        let mut band = MemoryBand::new();
        band.write(4);
        band.move_right(1);
        band.move_left(3);

        let expected = MemoryBand {
            band: vec![0, 0, 4, 0].into_iter().collect(),
            current_index: 0,
        };
        assert_eq!(band, expected);
    }
}
