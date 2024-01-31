use sudoku_solver::types::Values;

pub const EASY: Values = [
    [0, 3, 0, 0, 0, 0, 1, 0, 0],
    [5, 0, 0, 0, 9, 6, 0, 2, 0],
    [2, 8, 0, 3, 4, 0, 0, 0, 0],
    [0, 9, 4, 0, 0, 0, 8, 0, 0],
    [0, 2, 5, 8, 7, 9, 0, 4, 3],
    [0, 7, 6, 5, 3, 0, 9, 1, 2],
    [9, 0, 0, 6, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 8, 3, 5, 0, 1],
    [0, 5, 0, 0, 0, 0, 0, 3, 6],
];

pub const EASY_SOLVED: Values = [
    [6, 3, 9, 7, 2, 8, 1, 5, 4],
    [5, 4, 7, 1, 9, 6, 3, 2, 8],
    [2, 8, 1, 3, 4, 5, 7, 6, 9],
    [3, 9, 4, 2, 6, 1, 8, 7, 5],
    [1, 2, 5, 8, 7, 9, 6, 4, 3],
    [8, 7, 6, 5, 3, 4, 9, 1, 2],
    [9, 1, 3, 6, 5, 2, 4, 8, 7],
    [7, 6, 2, 4, 8, 3, 5, 9, 1],
    [4, 5, 8, 9, 1, 7, 2, 3, 6],
];

const MEDIUM: Values = [
    [0, 2, 0, 6, 0, 8, 0, 0, 0],
    [5, 8, 0, 0, 0, 9, 7, 0, 0],
    [0, 0, 0, 0, 4, 0, 0, 0, 0],
    [3, 7, 0, 0, 0, 0, 5, 0, 0],
    [6, 0, 0, 0, 0, 0, 0, 0, 4],
    [0, 0, 8, 0, 0, 0, 0, 1, 3],
    [0, 0, 0, 0, 2, 0, 0, 0, 0],
    [0, 0, 9, 8, 0, 0, 0, 3, 6],
    [0, 0, 0, 3, 0, 6, 0, 9, 0],
];

pub const MEDIUM_SOLVED: Values = [
    [1, 2, 3, 6, 7, 8, 9, 4, 5],
    [5, 8, 4, 2, 3, 9, 7, 6, 1],
    [9, 6, 7, 1, 4, 5, 3, 2, 8],
    [3, 7, 2, 4, 6, 1, 5, 8, 9],
    [6, 9, 1, 5, 8, 3, 2, 7, 4],
    [4, 5, 8, 7, 9, 2, 6, 1, 3],
    [8, 3, 6, 9, 2, 4, 1, 5, 7],
    [2, 1, 9, 8, 5, 7, 4, 3, 6],
    [7, 4, 5, 3, 1, 6, 8, 9, 2],
];

const HARD: Values = [
    [0, 2, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 6, 0, 0, 0, 0, 3],
    [0, 7, 4, 0, 8, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 3, 0, 0, 2],
    [0, 8, 0, 0, 4, 0, 0, 1, 0],
    [6, 0, 0, 5, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 7, 8, 0],
    [5, 0, 0, 0, 0, 9, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 4, 0],
];

pub const HARD_SOLVED: Values = [
    [1, 2, 6, 4, 3, 7, 9, 5, 8],
    [8, 9, 5, 6, 2, 1, 4, 7, 3],
    [3, 7, 4, 9, 8, 5, 1, 2, 6],
    [4, 5, 7, 1, 9, 3, 8, 6, 2],
    [9, 8, 3, 2, 4, 6, 5, 1, 7],
    [6, 1, 2, 5, 7, 8, 3, 9, 4],
    [2, 6, 9, 3, 1, 4, 7, 8, 5],
    [5, 4, 8, 7, 6, 9, 2, 3, 1],
    [7, 3, 1, 8, 5, 2, 6, 4, 9],
];

const UNSOLVABLE: Values = [
    [8, 2, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 6, 0, 0, 0, 0, 3],
    [0, 7, 4, 0, 8, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 3, 0, 0, 2],
    [0, 8, 0, 0, 4, 0, 0, 1, 0],
    [6, 0, 0, 5, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 7, 8, 0],
    [5, 0, 0, 0, 0, 9, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 4, 0],
];

const BROKEN: Values = [
    [1, 3, 0, 0, 0, 0, 1, 0, 0],
    [5, 0, 0, 0, 9, 6, 0, 2, 0],
    [2, 8, 0, 3, 4, 0, 0, 0, 0],
    [0, 9, 4, 0, 0, 0, 8, 0, 0],
    [0, 2, 5, 8, 7, 9, 0, 4, 3],
    [0, 7, 6, 5, 3, 0, 9, 1, 2],
    [9, 0, 0, 6, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 8, 3, 5, 0, 1],
    [0, 5, 0, 0, 0, 0, 0, 3, 6],
];

const STR_CSV_LINE: &str = "0, 3, 0, 0, 0, 0, 1, 0, 0, 5, 0, 0, 0, 9, 6, 0, 2, 0, 2, 8, 0, 3, 4, 0, 0, 0, 0, 0, 9, 4, 0, 0, 0, 8, 0, 0, 0, 2, 5, 8, 7, 9, 0, 4, 3, 0, 7, 6, 5, 3, 0, 9, 1, 2, 9, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 3, 5, 0, 1, 0, 5, 0, 0, 0, 0, 0, 3, 6";
const STR_NUMERIC_ONLY: &str =
    "030000100500096020280340000094000800025879043076530912900600000000083501050000036";
const STR_NUMERIC_W_POINT: &str =
    ".3....1..5...96.2.28.34.....94...8...25879.43.7653.9129..6.........835.1.5.....36";

#[cfg(test)]
mod tests {
    use sudoku_solver::{
        board::Board, errors::BrokenSudokuError, reader::parse_values, solver::solve,
    };

    use crate::{
        BROKEN, EASY, EASY_SOLVED, HARD, HARD_SOLVED, MEDIUM, MEDIUM_SOLVED, STR_CSV_LINE,
        STR_NUMERIC_ONLY, STR_NUMERIC_W_POINT, UNSOLVABLE,
    };

    #[test]
    fn easy() {
        let mut board = Board::new(EASY).unwrap();
        println!("Start easy:\n{board}");
        let solved_board = solve(&mut board).unwrap();
        println!("Solved easy:\n{solved_board}\n--------------------------------");
        assert_eq!(solved_board.values, EASY_SOLVED);
    }

    #[test]
    fn medium() {
        let mut board = Board::new(MEDIUM).unwrap();
        println!("Start medium:\n{board}");
        let solved_board = solve(&mut board).unwrap();
        println!("Solved medium:\n{solved_board}\n--------------------------------");
        assert_eq!(solved_board.values, MEDIUM_SOLVED);
    }
    #[test]
    fn hard() {
        let mut board = Board::new(HARD).unwrap();
        println!("Start hard:\n{board}");
        let solved_board = solve(&mut board).unwrap();
        println!("Solved hard:\n{solved_board}\n--------------------------------");
        assert_eq!(solved_board.values, HARD_SOLVED);
    }
    #[test]
    fn unsolvable() {
        let mut board = Board::new(UNSOLVABLE).unwrap();
        let result = solve(&mut board);
        assert!(result.is_err());
        assert_eq!(result, Err(BrokenSudokuError))
    }
    #[test]
    fn broked() {
        let result = Board::new(BROKEN);
        assert!(result.is_err());
        assert_eq!(result, Err(BrokenSudokuError))
    }
    #[test]
    fn reader_csv_line() {
        let values = parse_values(STR_CSV_LINE);
        assert_eq!(values, EASY);
    }
    #[test]
    fn reader_numeric_only() {
        let values = parse_values(STR_NUMERIC_ONLY);
        assert_eq!(values, EASY);
    }
    #[test]
    fn reader_numeric_w_point() {
        let values = parse_values(STR_NUMERIC_W_POINT);
        assert_eq!(values, EASY);
    }
}
