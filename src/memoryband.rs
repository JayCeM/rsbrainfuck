use std::collections::VecDeque;

/// A memoryband is a sequential band of memorycells.
/// Each memory cell holds a value of type [`u8`].
/// The memoryband has one reading head that can be moved left or right. It can read and write to
/// the memory cell below it.
/// Each memory cell is initialized to `0`.
pub trait MemoryBand {
    fn new() -> Self;
    fn read(&self) -> u8;
    fn write(&mut self, int: u8);
    fn add(&mut self, int: u8);
    fn move_head(&mut self, moves: isize);
}


#[derive(Debug, PartialEq, Eq)]
/// This struct implements [`MemoryBand`] with an unlimited amount of memory cells to the left and
/// right. 
pub struct InfiniteMemoryBand {
    band: VecDeque<u8>,
    current_index: usize,
}

impl InfiniteMemoryBand {
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
}


impl MemoryBand for InfiniteMemoryBand {
    /// Creates a new Memoryband instance
    fn new() -> InfiniteMemoryBand {
        InfiniteMemoryBand {
            band: vec![0].into_iter().collect(),
            current_index: 0,
        }
    }

    /// Outputs the value that is currently readable
    fn read(&self) -> u8 {
        self.band[self.current_index]
    }

    /// Writes `int` to the current cell
    fn write(&mut self, int: u8) {
        self.band[self.current_index] = int;
    }

    /// Adds `int` to the current cell
    fn add(&mut self, int: u8) {
        self.band[self.current_index] = self.band[self.current_index].overflowing_add(int).0;
    }

    /// Moves the reading head left by `moves` amount.
    /// Positive values correspond to moving right, negative values to moving left.
    fn move_head(&mut self, moves: isize) {
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
    const NEG1: u8 = u8::MAX;
    use super::*;
    #[test]
    fn test_new() {
        let band = InfiniteMemoryBand::new();
        let expected = InfiniteMemoryBand {
            band: vec![0].into_iter().collect(),
            current_index: 0,
        };
        assert_eq!(band, expected);
    }

    #[test]
    fn test_read() {
        let band = InfiniteMemoryBand {
            band: vec![1, 2].into_iter().collect(),
            current_index: 1,
        };
        assert_eq!(2, band.read());
    }

    #[test]
    fn test_write() {
        let mut band = InfiniteMemoryBand {
            band: vec![1, 2].into_iter().collect(),
            current_index: 1,
        };
        band.write(NEG1-4); // -5

        let expected = InfiniteMemoryBand {
            band: vec![1, NEG1-4].into_iter().collect(),
            current_index: 1,
        };

        assert_eq!(band, expected);
    }

    #[test]
    fn test_add() {
        let mut band = InfiniteMemoryBand::new();
        band.add(NEG1);

        let expected = InfiniteMemoryBand {
            band: vec![NEG1].into_iter().collect(),
            current_index: 0,
        };

        assert_eq!(band, expected);
    }

    #[test]
    fn test_move_right() {
        let mut band = InfiniteMemoryBand::new();
        band.write(4);
        band.move_right(2);

        let expected = InfiniteMemoryBand {
            band: vec![4, 0, 0].into_iter().collect(),
            current_index: 2,
        };
        assert_eq!(band, expected);
    }

    #[test]
    fn test_move_left() {
        let mut band = InfiniteMemoryBand::new();
        band.write(4);
        band.move_right(1);
        band.move_left(3);

        let expected = InfiniteMemoryBand {
            band: vec![0, 0, 4, 0].into_iter().collect(),
            current_index: 0,
        };
        assert_eq!(band, expected);
    }
}
