use super::Board;
use std::string::ToString;

static _BORDER: &'static str = " |ABCDEFG|";

impl ToString for Board {
    fn to_string(&self) -> String {
        unimplemented!();
    }
}

impl Board {
    fn _get_row_strings(&self) -> Vec<String> {
        // let temp = (0..self.height)
        //     .map(|row| {
        //         self.get_row_indices(row)
        //         .into_iter()
        //         .map(|i| self.squares[i])
        //          // todo todo
        //     })
        unimplemented!();
    }

    fn _get_row_indices(&self, row: u32) -> Vec<usize> {
        (0..self.width)
            .map(|col| self.get_pos_index(row, col))
            .collect()
    }
}
