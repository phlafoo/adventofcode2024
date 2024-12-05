#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let width = (input.find('\n').unwrap() + 1) as i32;
    let input = input.as_bytes();

    // Called after an 'A' is found to check for "M_S" in a single orientation determined by (x, y).
    let check_mas = |i, (x, y): (i32, i32)| -> bool {
        let delta = x + y * width;
        if input[(i - delta) as usize] != b'M' || input[(i + delta) as usize] != b'S' {
            return false;
        }
        true
    };
    
    // Bounds for 'A' in "MAS". Assumes input has > 2 rows.
    let min_index = width + 1;
    let max_index = input.len() as i32 - width - 1;
    let mut result = 0;
    
    // For each 'A', check diag directions for "M_S".
    for (i, _) in input.iter().enumerate().filter(|(_, &b)| b == b'A') {
        let i = i as i32;

        if i >= min_index && i <= max_index {
            let ne = check_mas(i, (1, -1)); // NE
            let nw = check_mas(i, (-1, -1)); // NW
            // Checking SE is redundant if NW is true, but adding the check is slower. Likewise for SW/NE
            let se = check_mas(i, (1, 1)); // SE
            let sw = check_mas(i, (-1, 1)); // SW

            // Check for X
            result += ((sw || ne) && (se || nw)) as i32;
        }
    }

    Ok(result.to_string())
}
// 1925

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
