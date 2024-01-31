use crate::types::Coord;

pub const ALL_COORDS: [(usize, usize); 81] = get_all_coords();
pub const BOXES: [[Coord; 9]; 9] = get_boxes();

const fn get_all_coords() -> [(usize, usize); 81] {
    let mut coords: [(usize, usize); 81] = [(0, 0); 81];
    let mut n = 0;
    while n < 9 {
        let mut m = 0;
        while m < 9 {
            coords[n * 9 + m] = (n, m) as (usize, usize);
            m += 1;
        }
        n += 1;
    }
    return coords;
}

const fn get_boxes() -> [[Coord; 9]; 9] {
    let mut boxes = [[(0, 0); 9]; 9];
    let mut b: usize = 0;
    while b < 9 {
        let mut n: usize = 0;
        while n < 9 {
            boxes[b][n] = (b / 3 * 3 + n / 3, b % 3 * 3 + n % 3);
            n += 1;
        }
        b += 1;
    }
    return boxes;
}
