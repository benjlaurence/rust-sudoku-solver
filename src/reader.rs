use crate::consts::ALL_COORDS;
use crate::types::Values;
static ZERO_CHARS: &'static [char] = &['*', '.'];
pub fn parse_values(string: &str) -> Values {
    let mut values: Values = Default::default();
    let mut n: usize = 0;
    for mut v in string.chars() {
        if ZERO_CHARS.contains(&v) {
            v = '0';
        }
        if v.is_numeric() {
            let (r, c) = ALL_COORDS[n];
            values[r][c] = v.to_digit(10).unwrap() as u8;
            n += 1;
        }
    }
    return values;
}
