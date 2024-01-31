use crate::board::Board;
use crate::errors::{BrokenSudokuError, BrokenSudokuResult};
use crate::types::Coord;
use crate::utils::{count_trues, get_box_n, get_only_value};

fn infer_values(
    board: &mut Board,
    values_to_infer: impl IntoIterator<Item = Coord>,
) -> BrokenSudokuResult<bool> {
    let mut found = false;
    for (r, c) in values_to_infer {
        if !board.knowns[r][c] {
            let trues = count_trues(&board.possibles[r][c]);
            if trues == 0 {
                return Err(BrokenSudokuError);
            }
            if trues == 1 {
                let val = get_only_value(&board.possibles[r][c]);
                let linked_coords = board.set_value(r, c, val)?;
                infer_values(board, linked_coords)?;
                found = true;
            } else {
                let b = get_box_n(r, c);
                for v_p in board.get_possibles_as_indexes(r, c) {
                    if [
                        board.rows_n_possibles[r][v_p],
                        board.columns_n_possibles[c][v_p],
                        board.boxes_n_possibles[b][v_p],
                    ]
                    .contains(&1)
                    {
                        let linked_coords = board.set_value(r, c, (v_p as u8) + 1)?;
                        infer_values(board, linked_coords)?;
                        found = true;
                        break;
                    }
                }
            }
        }
    }

    return Ok(found);
}

fn get_next_value_to_try(board: &Board) -> BrokenSudokuResult<(usize, usize, u8)> {
    let mut min_r: usize = 0;
    let mut min_c: usize = 0;
    let mut min_unknown_n: u8 = 9;
    for (r, c) in board.unknowns() {
        let n_possibles = board.get_n_possibles(r, c);
        if n_possibles == 2 {
            return Ok((r, c, board.get_next_possible(r, c)?));
        } else {
            if n_possibles < min_unknown_n {
                min_r = r;
                min_c = c;
                min_unknown_n = n_possibles;
            }
        }
    }
    return Ok((min_r, min_c, board.get_next_possible(min_r, min_c)?));
}

fn infer_to_completion(board: &mut Board) -> BrokenSudokuResult<()> {
    let mut solving = true;
    while solving {
        let unknowns = board.unknowns();
        solving = infer_values(board, unknowns)?;
    }
    Ok(())
}

pub fn solve(board: &mut Board) -> BrokenSudokuResult<Board> {
    infer_to_completion(board)?;
    while !board.check_solved() {
        let (r, c, v) = get_next_value_to_try(board)?;
        let mut test_board = board.clone();
        test_board.set_value(r, c, v)?;
        let solve_result = solve(&mut test_board);
        if solve_result.is_ok() {
            return Ok(solve_result.unwrap());
        }
        board.set_not_possible(r, c, (v - 1) as usize)?;
        infer_to_completion(board)?;
    }

    return Ok(*board);
}
