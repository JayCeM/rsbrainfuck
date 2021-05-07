use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct MemoryBand {
    band: VecDeque<i64>,
    current_index: usize,
}

impl MemoryBand {
    pub fn new() -> MemoryBand {
        MemoryBand {
            band: vec![0].into_iter().collect(),
            current_index: 0,
        }
    }

    pub fn read(&self) -> i64 {
        self.band[self.current_index]
    }

    pub fn write(&mut self, int: i64) {
        self.band[self.current_index] = int;
    }

    pub fn add(&mut self, int: i64) {
        self.band[self.current_index] += int;
    }

    pub fn move_right(&mut self, moves: usize) {
        self.current_index += moves;

        if self.current_index >= self.band.len() {
            let offset = self.current_index - self.band.len() + 1;
            for _ in 0..offset {
                self.band.push_back(0);
            }
        }
    }

    pub fn move_left(&mut self, moves: usize) {
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
