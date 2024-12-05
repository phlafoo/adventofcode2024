#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let width = (input.find('\n').unwrap() + 1) as i32;
    let input = input.as_bytes();

    // Called after an 'X' is found to check for "MAS" in a single direction determined by (x, y).
    let check_mas = |mut i, (x, y): (i32, i32)| -> i32 {
        let delta = x + y * width;
        for char in "MAS".bytes() {
            i += delta;
            if input[i as usize] != char {
                return 0;
            }
        }
        1
    };
    
    // Bounds for 'X' in "XMAS". Assumes input has > 3 rows.
    let max_x = input.len() as i32 - 4;
    let min_x = 3;
    let min_y = width * 3;
    let max_y = input.len() as i32 - width * 3;
    
    let mut result = 0;

    // For each 'X', check all 8 directions for "MAS".
    for (i, _) in input.iter().enumerate().filter(|(_, &b)| b == b'X') {
        let i = i as i32;

        if i <= max_x {
            result += check_mas(i, (1, 0)); // E
        }
        if i >= min_x {
            result += check_mas(i, (-1, 0)); // W
        }
        if i >= min_y {
            result += check_mas(i, (0, -1)); // N
            result += check_mas(i, (1, -1)); // NE
            result += check_mas(i, (-1, -1)); // NW
        }
        if i <= max_y {
            result += check_mas(i, (0, 1)); // S
            result += check_mas(i, (1, 1)); // SE
            result += check_mas(i, (-1, 1)); // SW
        }
    }

    Ok(result.to_string())
}
// 2483

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
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
