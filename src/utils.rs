use std::collections::HashSet;

use crate::consts::{ALL_COORDS, BOXES};
use crate::types::{Coord, CoordSet, Knowns};
pub const fn get_box_n(r: usize, c: usize) -> usize {
    return ((r / 3) * 3 + c / 3) as usize;
}

pub fn count_trues(bools: &[bool; 9]) -> u8 {
    return (bools.into_iter().filter(|&b| *b).count()) as u8;
}

pub fn get_only_value(bools: &[bool; 9]) -> u8 {
    for n in 0usize..9 {
        if bools[n] {
            return (n + 1) as u8;
        }
    }
    return 0;
}

pub fn get_linked_values(knowns: &Knowns, r: usize, c: usize) -> CoordSet {
    let b = get_box_n(r, c);
    let mut set: CoordSet = HashSet::with_capacity(20);
    for n in 0usize..9 {
        for (r2, c2) in [(r, n), (n, c), BOXES[b][n]] {
            if !&knowns[r2][c2] && (r2, c2) != (r, c) {
                set.insert((r2, c2));
            }
        }
    }
    return set;
}

pub struct UnknownCoordsIterator {
    pub index: usize,
    pub knowns: Knowns,
}

impl Iterator for UnknownCoordsIterator {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        for i in self.index..81 {
            let (r, s) = ALL_COORDS[i];
            if !self.knowns[r][s] {
                self.index = i + 1;
                return Some((r, s));
            }
        }
        return None;
    }
}
