#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input.as_bytes();

    // Find dimension of grid (assume same width and height) and guard starting position
    let mut it = input.iter();
    let dim = it.position(|&b| b == b'\n').unwrap() + 1;
    let mut guard_index = it.position(|&b| b == b'^').unwrap() as i32 + dim as i32;

    // Keep track of which points have been visited
    let mut visited = vec![false; dim * (dim - 1)];
    visited[guard_index as usize] = true;

    // Index offsets to move in desired direction
    let north = -(dim as i32);
    let east = 1;
    let south = dim as i32;
    let west = -1;

    // Starting direction
    let mut dir = north;

    // Returns next direction after hitting obstacle (turn right)
    let get_next_dir = |prev_dir| -> i32 {
        if prev_dir == north {
            east
        } else if prev_dir == east {
            south
        } else if prev_dir == south {
            west
        } else {
            north
        }
    };

    let mut result = 1;
    loop {
        // Peek next position to determine if there is an obstacle
        let next_index = (guard_index + dir) as usize;
        let Some(&b) = input.get(next_index) else {
            // Out of bounds
            break;
        };
        if b == b'.' || b == b'^' {
            // No obstacle, check if this point has already been visited
            if !visited[next_index] {
                result += 1;
                visited[next_index] = true;
            }
            // Progress guard
            guard_index += dir;
        } else if b == b'#' {
            // Obstacle! Turn right
            dir = get_next_dir(dir);
        } else {
            // New line character, out of bounds
            break;
        }
    }

    Ok(result.to_string())
}
// 5177

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}