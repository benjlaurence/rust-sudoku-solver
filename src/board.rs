use std::fmt;

use crate::consts::ALL_COORDS;
use crate::errors::{BrokenSudokuError, BrokenSudokuResult};
use crate::types::*;
use crate::utils::{count_trues, get_box_n, get_linked_values, UnknownCoordsIterator};

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Board {
    pub values: Values,
    pub knowns: Knowns,
    pub possibles: Possibles,
    pub rows_n_possibles: Values,
    pub columns_n_possibles: Values,
    pub boxes_n_possibles: Values,
}

impl Board {
    pub fn new(values: Values) -> BrokenSudokuResult<Self> {
        let mut board: Board = Default::default();
        board.possibles = [[[true; 9]; 9]; 9];
        board.rows_n_possibles = [[9; 9]; 9];
        board.columns_n_possibles = [[9; 9]; 9];
        board.boxes_n_possibles = [[9; 9]; 9];
        for r in 0usize..9 {
            for c in 0usize..9 {
                let v = values[r][c];
                if v != 0 {
                    board.set_value(r, c, v)?;
                }
            }
        }
        return Ok(board);
    }
    fn to_string(&self) -> String {
        let mut string = "".to_owned();

        for n in 0usize..9 {
            for m in 0usize..9 {
                string.push_str(&format!("{} ", self.values[n][m]));
                if m == 2 || m == 5 {
                    string.push_str("│ ");
                }
            }
            string.push_str("\n");
            if n == 2 || n == 5 {
                string.push_str("──────┼───────┼──────\n");
            }
        }
        return string;
    }
    pub fn set_not_possible(&mut self, r: usize, c: usize, v: usize) -> BrokenSudokuResult<()> {
        if self.possibles[r][c][v] {
            let b = get_box_n(r, c);
            self.possibles[r][c][v] = false;
            self.rows_n_possibles[r][v] -= 1;
            self.columns_n_possibles[c][v] -= 1;
            self.boxes_n_possibles[b][v] -= 1;
            if [
                self.rows_n_possibles[r][v],
                self.columns_n_possibles[c][v],
                self.boxes_n_possibles[b][v],
            ]
            .contains(&0)
            {
                return Err(BrokenSudokuError);
            }
        }
        return Ok(());
    }
    pub fn set_value(&mut self, r: usize, c: usize, v: u8) -> BrokenSudokuResult<CoordSet> {
        let v_p = (v - 1) as usize;

        if !self.possibles[r][c][v_p] || self.knowns[r][c] {
            return Err(BrokenSudokuError);
        }

        for n in 0usize..9 {
            if n != v_p {
                self.set_not_possible(r, c, n)?;
            }
        }

        self.values[r][c] = v;
        self.knowns[r][c] = true;
        let linked_values = get_linked_values(&self.knowns, r, c);

        for &(r2, c2) in &linked_values {
            self.set_not_possible(r2, c2, v_p)?;
        }
        return Ok(linked_values);
    }
    pub fn unknowns(&self) -> UnknownCoordsIterator {
        return UnknownCoordsIterator {
            index: 0,
            knowns: self.knowns,
        };
    }
    pub fn check_solved(&self) -> bool {
        for (n, m) in ALL_COORDS {
            if !self.knowns[n][m] {
                return false;
            }
        }
        return true;
    }
    pub fn get_n_possibles(&self, r: usize, c: usize) -> u8 {
        return count_trues(&self.possibles[r][c]);
    }
    pub fn get_next_possible(&self, r: usize, c: usize) -> BrokenSudokuResult<u8> {
        for v in self.get_possibles_as_indexes(r, c) {
            return Ok((v as u8) + 1);
        }
        return Err(BrokenSudokuError);
    }
    pub fn get_possibles_as_indexes(&self, r: usize, c: usize) -> Vec<usize> {
        let mut values: Vec<usize> = Vec::with_capacity(9);
        for (i, v) in self.possibles[r][c].iter().enumerate() {
            if *v {
                values.push(i);
            }
        }
        return values;
    }
}

impl fmt::Display for Board {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
