#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Empty = 0,
    Queen = 1,
}

pub struct Board {
    width: u32,
    height: u32,
    squares: Vec<Square>,
}

impl Board {
    pub fn new() -> Board {
        let width = 8;
        let height = 8;

        let squares = (0..width * height)
            .map(|_| Square::Empty)
            .collect();

        Board {
            width,
            height,
            squares,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn queen_count(&self) -> u32 {
        self.squares.iter().filter(|s| **s == Square::Queen).count() as u32
    }

    fn get_pos_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
