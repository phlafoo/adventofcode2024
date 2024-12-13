#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Didn't spend any time on optimizing today
    
    let dim = input.find('\n').unwrap() + 1;
    let input = input.as_bytes();
    
    let mut visited = vec![false; dim * (dim - 1)];

    let north = -(dim as i32);
    let south = dim as i32;
    let west = -1;
    let east = 1;

    let dirs = [north, east, south, west];

    // If we just moved north we don't have to check south...
    let dirs_opp = [south, west, north, east];

    let max_index = input.len() as i32 - 1;
    let mut result = 0;

    for (i, &plant) in input.iter().enumerate() {
        if visited[i] {
            continue;
        }
        if plant == b'\n' {
            continue;
        }
        visited[i] = true;

        // Valid plant that we have not visited, traverse region

        // stack elements -> (index, direction that can be skipped when checking neighbors)
        let mut stack = [(0, 0); 128];
        stack[0] = (i as i32, 0);
        let mut stack_len = 1;
        
        let mut area = 1;
        let mut peri = 0;

        while stack_len > 0 {
            stack_len -= 1;
            let (ii, dir_skip) = stack[stack_len];
            for (dir_index, &dir) in dirs.iter().enumerate().filter(|(_, &d)| d != dir_skip) {
                // Get neighbor index
                let n = ii + dir;

                // Bounds check
                if n < 0 || n > max_index {
                    peri += 1;
                    continue;
                }

                if input[n as usize] == plant {
                    if !visited[n as usize] {
                        // Same plant type, mark as visited and increase area
                        area += 1;
                        visited[n as usize] = true;
                        stack[stack_len] = (n, dirs_opp[dir_index]);
                        stack_len += 1;
                    }
                } else {
                    // Different plant type
                    peri += 1;
                }
            }
        }
        result += area * peri;
    }

    Ok(result.to_string())
}
// 1485656

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
        assert_eq!("1930", process(input)?);
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
        assert_eq!("772", process(input)?);
        Ok(())
    }
}
