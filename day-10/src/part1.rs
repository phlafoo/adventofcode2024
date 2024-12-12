#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let dim = input.find('\n').unwrap() + 1;
    let input = input.as_bytes();

    let mut visited = vec![false; dim * (dim - 1)];

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
        visited.fill(false);

        // Work thru all paths starting from this 9
        while let Some(s) = stack.pop() {
            visited[s] = true;
            for nx in neighbors {
                let n = s as i32 + nx;
                // Bounds check
                if n < 0 {
                    continue;
                }
                let n = n as usize;
                if n >= input.len() || visited[n] {
                    continue;
                }
                if input[n] == input[s] - 1 {
                    if input[n] == b'0' {
                        visited[n] = true;
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
// 611

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
        assert_eq!("36", process(input)?);
        Ok(())
    }
}
