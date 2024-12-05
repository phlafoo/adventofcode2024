use fxhash::FxHashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let width = (input.find('\n').unwrap() + 1) as i32;
    let input = input.as_bytes();

    // Map index of 'A' to its count (1 or 2, 2 means X was found)
    let mut map = FxHashMap::default();

    // Called after an 'X' is found to check for "MAS" in a single direction determined by (x, y).
    let mut check_mas = |mut i, (x, y): (i32, i32)| {
        let delta = x + y * width;
        for char in "AS".bytes() {
            i += delta;
            if input[i as usize] != char {
                return
            }
        }
        map.entry(i - delta).and_modify(|e| *e += 1).or_insert(1);
    };
    
    // Bounds for 'X' in "XMAS". Assumes input has > 3 rows.
    let min_y = width * 2;
    let max_y = input.len() as i32 - width * 2;
    
    // For each 'M', check diag directions for "AS".
    for (i, _) in input.iter().enumerate().filter(|(_, &b)| b == b'M') {
        let i = i as i32;

        if i >= min_y {
            check_mas(i, (1, -1)); // NE
            check_mas(i, (-1, -1)); // NW
        }
        if i <= max_y {
            check_mas(i, (1, 1)); // SE
            check_mas(i, (-1, 1)); // SW
        }
    }
    let result = map.iter().fold(0, |acc, (_, count)| acc + (*count == 2) as i32);

    Ok(result.to_string())
}
// 1925

#[cfg(test)]
mod tests {
    use super::*;
    /*
    E  3
    W  2
    N  2
    S  1
    NE 4
    NW 4
    SE 1
    SW 1
     */

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
