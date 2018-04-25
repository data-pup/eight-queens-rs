use super::Board;
use std::iter::FromIterator;
use std::string::ToString;

static EMPTY_CHAR: char = ' ';
static QUEEN_CHAR: char = 'Q';

impl ToString for Board {
    fn to_string(&self) -> String {
        let board_chars = self.initialize_char_matrix();
        let row_strings = Board::form_row_strings(board_chars);
        let bordered_strings = self.add_borders(row_strings);
        bordered_strings.join("\n")
    }
}

impl Board {
    fn initialize_char_matrix(&self) -> Vec<Vec<char>> {
        let (width, height) = self.dims();
        let mut chars = vec![vec![EMPTY_CHAR; width as usize]; height as usize];
        for (x, y) in self.get_queen_positions().into_iter() {
            chars[y as usize][x as usize] = QUEEN_CHAR;
        }
        chars
    }

    fn form_row_strings(chars: Vec<Vec<char>>) -> Vec<String> {
        chars
            .into_iter()
            .map(|row_vec| String::from_iter(row_vec.iter()))
            .collect::<Vec<String>>()
    }

    fn add_borders(&self, rows: Vec<String>) -> Vec<String> {
        let cap = (self.height() + 2) as usize;
        let mut bordered_vec = Vec::with_capacity(cap);
        let row_iter = rows.into_iter()
            .rev()
            .map(|row| format!("|{}|", row))
            .into_iter();

        bordered_vec.push(self.form_horizontal_border());
        bordered_vec.extend(row_iter);
        bordered_vec.push(self.form_horizontal_border());

        bordered_vec
    }

    fn form_horizontal_border(&self) -> String {
        let border_width = (self.width() + 2) as usize;
        let char_iter = vec!['-'; border_width].into_iter();
        String::from_iter(char_iter)
    }
}

#[cfg(test)]
mod board_to_string_tests {
    use std::string::ToString;
    use Board;

    #[test]
    fn default_board_to_string_works() {
        let b = Board::new();
        let res = b.to_string();
        let expected = vec![
            "----------",
            "|        |",
            "|        |",
            "|        |",
            "|        |",
            "|        |",
            "|        |",
            "|        |",
            "|        |",
            "----------",
        ].join("\n");
        assert_eq!(res, expected);
    }

    #[test]
    fn board_with_3_queens_converts_correctly() {
        let b = [(0, 0), (0, 7), (6, 0), (6, 6)]
            .iter()
            .cloned()
            .collect::<Board>();
        let res = b.to_string();
        let expected = vec![
            "----------",
            "|Q       |",
            "|      Q |",
            "|        |",
            "|        |",
            "|        |",
            "|        |",
            "|        |",
            "|Q     Q |",
            "----------",
        ].join("\n");
        assert_eq!(res, expected);
    }
}
