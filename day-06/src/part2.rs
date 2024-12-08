#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input.as_bytes();

    // Find dimension of grid (assume same width and height)
    let dim = input.iter().position(|&b| b == b'\n').unwrap() + 1;

    // Keep track of which points have been visited
    let mut visited = vec![false; dim * (dim - 1)];

    // Keep track of visited points when checking loops
    let mut visited_loop = visited.clone();

    // Index offsets to move in desired direction
    let north = -(dim as i32);
    let east = 1;
    let south = dim as i32;
    let west = -1;

    // `cols` is indexed by x-coord, `rows` in indexed by y-coord. They store positions of obstacles.
    // Used for binary search during loop checks
    let mut cols = vec![vec![]; dim];
    let mut rows = vec![vec![]; dim];

    // Get (x, y) coords from index
    let get_coords = |index: usize| -> (usize, usize) { (index % dim, index / dim) };

    // Get index from (x, y) coords
    let get_index = |(x, y)| -> usize { x + y * dim };

    // Returns next direction after hitting obstacle (turn right)
    let get_next_dir = |prev_dir| -> i32 {
        if prev_dir == north {
            east
        } else if prev_dir == east {
            south
        } else if prev_dir == south {
            west
        } else {
            // west
            north
        }
    };

    // Starting direction
    let mut dir = north;
    let mut start_pos = (0, 0);

    // Save obstacle positions and get start position
    for (y, row) in input.chunks(dim).enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == b'.' {
                continue;
            } else if c == b'#' {
                cols[x].push(y as u8);
                rows[y].push(x as u8);
            } else if c == b'^' {
                start_pos = (x, y);
            }
        }
    }

    let mut guard_index = get_index(start_pos) as i32;
    let mut result = 0;

    loop {
        // Peek next position to determine if there is an obstacle
        let next_index = (guard_index + dir) as usize;

        let Some(&next_byte) = input.get(next_index) else {
            // Out of bounds
            break;
        };
        if next_byte == b'^' {
            // Don't want to place an obstacle on the starting point, just go to next point
            guard_index += dir;
        } else if next_byte == b'.' {
            // No obstacle, check if this point has already been visited
            if !visited[next_index] {
                visited[next_index] = true;

                // Not yet visited, place an obstacle and check for loops

                // (x, y) is where this search will start
                let (mut x, mut y) = get_coords(guard_index as usize);

                // Obstacle position
                let obs = get_coords(next_index);

                // Add obstacle
                let col = cols[obs.0].binary_search(&(obs.1 as u8)).unwrap_err();
                cols[obs.0].insert(col, obs.1 as u8);
                let row = rows[obs.1].binary_search(&(obs.0 as u8)).unwrap_err();
                rows[obs.1].insert(row, obs.0 as u8);

                // Starting direction (after hitting our placed obstacle)
                let mut loop_dir = get_next_dir(dir);
                let mut prev_pos;

                // If we touch 2 points back-to-back that we already visited, it means we made a loop
                let mut visited_count = 0;
                visited_loop.fill(false);
                loop {
                    // It is possible to change directions twice while staying on the same tile,
                    // so we need to prevent false positive loop detection with `prev_pos`
                    prev_pos = (x, y);

                    // We are not traversing tile by tile, but rather obstacle by obstacle using
                    // binary search on our `cols` and `rows` grids that store obstacle positions.
                    if loop_dir == north {
                        let row_index = cols[x].binary_search(&(y as u8)).unwrap_err();
                        if row_index == 0 {
                            break; // Out of bounds
                        }
                        y = (cols[x][row_index - 1] + 1) as usize;
                    } else if loop_dir == east {
                        let row_index = rows[y].binary_search(&(x as u8)).unwrap_err();
                        if row_index == rows[y].len() {
                            break; // Out of bounds
                        }
                        x = (rows[y][row_index] - 1) as usize;
                    } else if loop_dir == south {
                        let row_index = cols[x].binary_search(&(y as u8)).unwrap_err();
                        if row_index == cols[x].len() {
                            break; // Out of bounds
                        }
                        y = (cols[x][row_index] - 1) as usize;
                    } else {
                        let row_index = rows[y].binary_search(&(x as u8)).unwrap_err();
                        if row_index == 0 {
                            break; // Out of bounds
                        }
                        x = (rows[y][row_index - 1] + 1) as usize;
                    }
                    // If we didn't turn twice on the same tile...
                    if (x, y) != prev_pos {
                        let index = get_index((x, y));
                        if visited_loop[index] {
                            // Already visited, if this happens twice in a row then we made a loop
                            visited_count += 1;
                            if visited_count == 2 {
                                result += 1; // LOOP detected
                                break;
                            }
                        } else {
                            // Not yet visited
                            visited_loop[index] = true;
                            visited_count = 0;
                        }
                    }
                    // Change direction
                    loop_dir = get_next_dir(loop_dir);
                }
                // Remove the temporary obstacle
                cols[obs.0].remove(col);
                rows[obs.1].remove(row);
            }
            
            // Done checking for loops. Progress guard position
            guard_index += dir;
        } else if next_byte == b'#' {
            // Obstacle! Turn right
            dir = get_next_dir(dir);
        } else {
            // New line character, out of bounds
            break;
        }
    }

    Ok(result.to_string())
}
// 1686

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
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
