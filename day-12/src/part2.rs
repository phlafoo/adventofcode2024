#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let dim = input.find('\n').unwrap() + 1;
    let input = input.as_bytes();

    let mut visited = vec![false; dim * (dim - 1)];

    let north = -(dim as i32);
    let south = dim as i32;
    let west = -1;
    let east = 1;

    let dirs = [north, east, south, west];
    let clockwise = [east, south, west, north];

    let max_index = input.len() as i32 - 1;
    let mut result = 0;
    let mut i = 0;

    loop {
        if visited[i] {
            i += 1;
            continue;
        }
        if input[i] == b'\n' {
            if i == input.len() - 1 {
                break;
            }
            i += 1;
            continue;
        }
        visited[i] = true;
        let plant = input[i];

        // Valid plant that we have not visited, traverse region

        let mut stack = [0; 128];
        stack[0] = i as i32;
        let mut stack_len = 1;

        let mut area = 1;
        let mut sides = 0; // Number of sides is equal to number of corners

        while stack_len > 0 {
            stack_len -= 1;
            let ii = stack[stack_len];
            for (dir_index, &dir) in dirs.iter().enumerate() {
                // Get neighbor index
                let n = ii + dir;

                // Bounds check
                if n < 0 || n > max_index {
                    // Check if on corner
                    let cw = ii + clockwise[dir_index];
                    if cw < 0 || cw > max_index || input[cw as usize] != plant {
                        sides += 1;
                    }
                    continue;
                }
                if input[n as usize] == plant {
                    if !visited[n as usize] {
                        // Same plant type, mark as visited and increase area
                        area += 1;
                        visited[n as usize] = true;
                        stack[stack_len] = n;
                        stack_len += 1;
                    }
                } else {
                    // Different plant type, check if on corner
                    let cw = ii + clockwise[dir_index];
                    let diag = cw + dir;
                    if cw < 0
                        || cw > max_index
                        || input[cw as usize] != plant
                        || (diag >= 0 && input[diag as usize] == plant)
                    {
                        sides += 1;
                    }
                }
            }
        }
        i += 1;
        result += area * sides;
    }

    Ok(result.to_string())
}
// 899196

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
        assert_eq!("1206", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process1() -> miette::Result<()> {
        let input = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
        assert_eq!("436", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input = "\
AAAA
BBCD
BBCC
EEEC
";
        assert_eq!("80", process(input)?);
        Ok(())
    }
}