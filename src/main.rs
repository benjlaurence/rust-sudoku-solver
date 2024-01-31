fn main() {
    use sudoku_solver::board::Board;
    use sudoku_solver::reader::parse_values;
    use sudoku_solver::solver::solve;
    println!("Enter Sudoku with 0, . or * where unknown values are: ");

    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let values = parse_values(string.as_str());
    let mut board = Board::new(values).unwrap();
    println!("{board}");
    let solved = solve(&mut board).unwrap();
    println!("{solved}");
}
