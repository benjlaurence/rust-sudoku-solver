use std::fmt;

pub type BrokenSudokuResult<T> = std::result::Result<T, BrokenSudokuError>;
#[derive(Debug, Clone, PartialEq)]
pub struct BrokenSudokuError;

impl fmt::Display for BrokenSudokuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sudoku is broken")
    }
}
