use std::collections::HashSet;

pub type Coord = (usize, usize);

pub type CoordSet = HashSet<Coord>;

pub type Knowns = [[bool; 9]; 9];
pub type Possibles = [[[bool; 9]; 9]; 9];
pub type Values = [[u8; 9]; 9];
