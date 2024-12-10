#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let dim = input.find('\n').unwrap() + 1;
    let input = input.as_bytes();

    let north = -(dim as i32);
    let south = dim as i32;
    let west = -1;
    let east = 1;

    let neighbors = [north, south, west, east];

    let mut stack = vec![];
    let mut result = 0;

    for (i, &b) in input.iter().enumerate() {
        // Only want to find 9s
        if b != b'9' {
            continue;
        }
        stack.push(i);

        // Work thru all paths starting from this 9
        while let Some(s) = stack.pop() {
            for nx in neighbors {
                let n = s as i32 + nx;
                // Bounds check
                if n < 0 || n as usize >= input.len() {
                    continue;
                }
                let n = n as usize;
                if input[n] == input[s] - 1 {
                    if input[n] == b'0' {
                        result += 1;
                        continue;
                    }
                    stack.push(n);
                }
            }
        }
    }

    Ok(result.to_string())
}
// 1380

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("81", process(input)?);
        Ok(())
    }
}
