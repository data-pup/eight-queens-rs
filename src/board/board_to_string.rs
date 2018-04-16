use std::string::ToString;
use super::Board;

impl ToString for Board {
    fn to_string(&self) -> String {
        unimplemented!();
    }
}

#[cfg(test)]
mod board_to_string_tests {
    use super::Board;

    #[test]
    fn it_works() {
        assert_eq!(1, 2, "To do...");
    }
}
