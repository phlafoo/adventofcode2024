#[tracing::instrument]
pub fn process1(input: &str) -> miette::Result<String> {
    let input = input.as_bytes();

    // Find dimension of grid (assume same width and height) and guard starting position
    let mut it = input.iter();
    let dim = it.position(|&b| b == b'\n').unwrap() as i32 + 1;
    let mut guard_index = it.position(|&b| b == b'^').unwrap() as i32 + dim;

    // const NORTH: u8 = 0b00000001;
    // const EAST: u8 = 0b00000010;
    // const SOUTH: u8 = 0b00000100;
    // const WEST: u8 = 0b00001000;

    // Keep track of which points have been visited
    let mut visited = vec![0; (dim * (dim - 1)) as usize];

    // Index offsets to move in desired direction
    let north = -dim;
    let east = 1;
    let south = dim;
    let west = -1;

    let dir_to_bitmask = |dir| -> i32 { dir + dim + dim };
    let north_m = dir_to_bitmask(north);
    let east_m = dir_to_bitmask(east);
    let south_m = dir_to_bitmask(south);
    let west_m = dir_to_bitmask(west);

    // n: -11 + 11 + 1 = 1
    // e: 1 + 11 + 1 = 13

    // visited[guard_index as usize] = dir_to_bitmask(north);

    // Starting direction
    let mut dir = north;

    let mut cols = vec![vec![]; dim as usize];
    let mut rows = vec![vec![]; dim as usize];

    let get_coords =
        |index: u32| -> (u32, u32) { (index as u32 % dim as u32, index as u32 / dim as u32) };

    let get_index = |x, y| -> u32 { x + y * dim as u32 };

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
    let mut start_pos = (0, 0);

    for (y, row) in input.chunks(dim as usize).enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == b'.' {
                continue;
            } else if c == b'#' {
                cols[x].push(y as u32);
                rows[y].push(x as u32);
            } else if c == b'^' {
                start_pos = (x as u32, y as u32);
            }
        }
    }

    
    
    

    let mut result = 0;
    loop {
        visited[guard_index as usize] |= dir_to_bitmask(dir);
        if guard_index == 15 {
            println!("d: {}, v: {}", dir, visited[guard_index as usize]);
        }

        // Peek next position to determine if there is an obstacle
        let next_index = (guard_index + dir) as usize;
        let Some(&b) = input.get(next_index) else {
            // Out of bounds
            break;
        };
        if b == b'.' || b == b'^' {
            path.push((guard_index as u32, dir));
            // Progress guard

            // CHECK
            loop {
                let (x, y) = get_coords(guard_index as u32);
                let next_dir = get_next_dir(dir);
                println!("{x},{y}");

                if next_dir == north {
                    let mut row_index = cols[x as usize].binary_search(&y).unwrap_err();
                    if row_index == 0 {
                        guard_index += dir;
                        continue;
                    }
                    // if row_index == cols[x as usize].len() {
                    //     continue;
                    // }
                    row_index -= 1;

                    let check_row = cols[x as usize][row_index] + 1;
                    let check_index = get_index(x, check_row);
                    if dir_to_bitmask(north) == (visited[check_index as usize] & dir_to_bitmask(north))
                    {
                        // Loop found!
                        println!("  north - {x},{check_row}");
                        result += 1;
                    }
                } else if next_dir == east {
                    // println!("EAST! i: {i}");
                    let row_index = rows[y as usize].binary_search(&x).unwrap_err();
                    if row_index == rows[y as usize].len() {
                        guard_index += dir;
                        continue;
                    }
                    let check_col = rows[y as usize][row_index] - 1;
                    let check_index = get_index(check_col, y);
                    if dir_to_bitmask(east) == (visited[check_index as usize] & dir_to_bitmask(east)) {
                        // Loop found!
                        println!("  east  - {check_col},{y}");

                        result += 1;
                    }
                }
                if next_dir == south {
                    let row_index = cols[x as usize].binary_search(&y).unwrap_err();
                    if row_index == cols[x as usize].len() {
                        guard_index += dir;
                        continue;
                    }
                    let check_row = cols[x as usize][row_index] - 1;
                    let check_index = get_index(x, check_row);
                    if check_index >= visited.len() as u32 {
                        guard_index += dir;
                        continue;
                    }
                    if dir_to_bitmask(south) == (visited[check_index as usize] & dir_to_bitmask(south))
                    {
                        // Loop found!
                        println!("  south - {x},{check_row}");
                        result += 1;
                    }
                }
                if next_dir == west {
                    let mut row_index = rows[y as usize].binary_search(&x).unwrap_err();
                    if row_index == 0 {
                        guard_index += dir;
                        continue;
                    }
                    row_index -= 1;
                    // if row_index == rows[y as usize].len() {
                    //     continue;
                    // }
                    let check_col = rows[y as usize][row_index] + 1;
                    let check_index = get_index(check_col, y);
                    if dir_to_bitmask(west) == (visited[check_index as usize] & dir_to_bitmask(west)) {
                        // Loop found!
                        println!("  west  - {check_col},{y}");
                        result += 1;
                    }
                }
            }

            // END
            guard_index += dir;
        } else if b == b'#' {
            let (x, y) = get_coords(next_index as u32);
            if cols[x as usize].binary_search(&y).is_err() {
                cols[x as usize].push(y);
            }
            if rows[y as usize].binary_search(&x).is_err() {
                rows[y as usize].push(x);
            }

            // Obstacle! Turn right
            dir = get_next_dir(dir);
            println!("g: {}", guard_index);
            visited[guard_index as usize] |= dir_to_bitmask(dir);
        } else {
            // New line character, out of bounds
            break;
        }
    }
    println!("{:?}", path);

    println!("r: {:?}\nc: {:?}", rows, cols);

    // for (i, dir) in path {
    //     let (x, y) = get_coords(i);
    //     let next_dir = get_next_dir(dir);
    //     println!("{x},{y}");

    //     if next_dir == north {
    //         let mut row_index = cols[x as usize].binary_search(&y).unwrap_err();
    //         if row_index == 0 {
    //             continue;
    //         }
    //         // if row_index == cols[x as usize].len() {
    //         //     continue;
    //         // }
    //         row_index -= 1;

    //         let check_row = cols[x as usize][row_index] + 1;
    //         let check_index = get_index(x, check_row);
    //         if dir_to_bitmask(north) == (visited[check_index as usize] & dir_to_bitmask(north)) {
    //             // Loop found!
    //             println!("  north - {x},{check_row}");
    //             result += 1;
    //         }
    //     } else if next_dir == east {
    //         // println!("EAST! i: {i}");
    //         let row_index = rows[y as usize].binary_search(&x).unwrap_err();
    //         if row_index == rows[y as usize].len() {
    //             continue;
    //         }
    //         let check_col = rows[y as usize][row_index] - 1;
    //         let check_index = get_index(check_col, y);
    //         if dir_to_bitmask(east) == (visited[check_index as usize] & dir_to_bitmask(east)) {
    //             // Loop found!
    //             println!("  east  - {check_col},{y}");

    //             result += 1;
    //         }
    //     }
    //     if next_dir == south {
    //         let row_index = cols[x as usize].binary_search(&y).unwrap_err();
    //         if row_index == cols[x as usize].len() {
    //             continue;
    //         }
    //         let check_row = cols[x as usize][row_index] - 1;
    //         let check_index = get_index(x, check_row);
    //         if check_index >= visited.len() as u32 {
    //             continue;
    //         }
    //         if dir_to_bitmask(south) == (visited[check_index as usize] & dir_to_bitmask(south)) {
    //             // Loop found!
    //             println!("  south - {x},{check_row}");
    //             result += 1;
    //         }
    //     }
    //     if next_dir == west {
    //         let mut row_index = rows[y as usize].binary_search(&x).unwrap_err();
    //         if row_index == 0 {
    //             continue;
    //         }
    //         row_index -= 1;
    //         // if row_index == rows[y as usize].len() {
    //         //     continue;
    //         // }
    //         let check_col = rows[y as usize][row_index] + 1;
    //         let check_index = get_index(check_col, y);
    //         if dir_to_bitmask(west) == (visited[check_index as usize] & dir_to_bitmask(west)) {
    //             // Loop found!
    //             println!("  west  - {check_col},{y}");
    //             result += 1;
    //         }
    //     }

    // }

    println!(
        "we: {}, w: {}, e: {}, dim: {dim}",
        (west_m | east_m),
        west_m,
        east_m
    );

    for (i, &b) in visited.iter().enumerate() {
        if i % dim as usize == dim as usize - 1 {
            continue;
        }
        if i % dim as usize == 0 {
            println!();
        }
        let mut c = '+';
        if b == (west_m | east_m) || b == west_m || b == east_m {
            c = '-'
        } else if b == (north_m | south_m) || b == north_m || b == south_m {
            c = '|'
        } else if b == 0 {
            c = '.'
        }
        print!("{}", c);
    }
    println!();
    println!();

    // let mut result = 0;

    // let input = input.as_bytes();
    // let dim = input.iter().position(|&b| b == b'\n').unwrap() + 1;
    // let mut cols = vec![vec![]; dim];
    // let mut rows = vec![vec![]; dim];
    // let mut pos = (0, 0);

    // for (y, row) in input.chunks(dim).enumerate() {
    //     for (x, &c) in row.iter().enumerate() {
    //         if c == b'.' {
    //             continue;
    //         } else if c == b'#' {
    //             cols[x].push(y as u32);
    //             rows[y].push(x as u32);
    //         } else if c == b'^' {
    //             pos = (x as u32, y as u32);
    //         }
    //     }
    // }
    // // const NORTH: (i32, i32) = (0, -1);
    // // const SOUTH: (i32, i32) = (0, 1);
    // // const WEST: (i32, i32) = (-1, 0);
    // // const EAST: (i32, i32) = (1, 0);
    // let mut dir = Dir::North;
    // let mut dbg = vec![];

    // loop {
    //     dbg.push(pos);
    //     println!("R: {result}");
    //     match dir {
    //         Dir::North => {
    //             let i = cols[pos.0 as usize].binary_search(&pos.1).unwrap_err() - 1;
    //             println!("{i}");
    //             let y = cols[pos.0 as usize][i];
    //             result += pos.1.abs_diff(y + 1);
    //             pos = (pos.0, y + 1);
    //             dir = Dir::East
    //         }
    //         Dir::South => {
    //             let i = cols[pos.0 as usize].binary_search(&pos.1).unwrap_err();
    //             if i == cols[pos.0 as usize].len() {
    //                 result += dim as u32 - pos.1;
    //                 break;
    //             }
    //             println!("pos:{},{}  i:{i}", pos.0, pos.1);
    //             let y = cols[pos.0 as usize][i];
    //             result += pos.1.abs_diff(y - 1);

    //             pos = (pos.0, y - 1);
    //             dir = Dir::West;
    //         }
    //         Dir::West => {
    //             let i = rows[pos.1 as usize].binary_search(&pos.0).unwrap_err() - 1;
    //             let x = rows[pos.1 as usize][i];
    //             result += pos.0.abs_diff(x + 1);

    //             pos = (x + 1, pos.1);
    //             dir = Dir::North
    //         }
    //         Dir::East => {
    //             let i = rows[pos.1 as usize].binary_search(&pos.0).unwrap_err();
    //             println!("{i}");
    //             let x = rows[pos.1 as usize][i];
    //             result += pos.0.abs_diff(x - 1);

    //             pos = (x - 1, pos.1);
    //             dir = Dir::South
    //         }
    //     }
    // }
    // println!("{:?}", dbg);

    Ok(result.to_string())
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input.as_bytes();

    // Find dimension of grid (assume same width and height) and guard starting position
    let mut it = input.iter();
    let dim = it.position(|&b| b == b'\n').unwrap() as i32 + 1;
    let mut guard_index = it.position(|&b| b == b'^').unwrap() as i32 + dim;

    // const NORTH: u8 = 0b00000001;
    // const EAST: u8 = 0b00000010;
    // const SOUTH: u8 = 0b00000100;
    // const WEST: u8 = 0b00001000;

    // Keep track of which points have been visited
    let mut visited = vec![0; (dim * (dim - 1)) as usize];

    // Index offsets to move in desired direction
    let north = -dim;
    let east = 1;
    let south = dim;
    let west = -1;

    let dir_to_bitmask = |dir| -> i32 { dir + dim + dim };
    let north_m = dir_to_bitmask(north);
    let east_m = dir_to_bitmask(east);
    let south_m = dir_to_bitmask(south);
    let west_m = dir_to_bitmask(west);

    // n: -11 + 11 + 1 = 1
    // e: 1 + 11 + 1 = 13

    // visited[guard_index as usize] = dir_to_bitmask(north);

    // Starting direction
    let mut dir = north;

    let mut cols = vec![vec![]; dim as usize];
    let mut rows = vec![vec![]; dim as usize];

    let get_coords =
        |index: u32| -> (u32, u32) { (index as u32 % dim as u32, index as u32 / dim as u32) };

    let get_index = |x, y| -> u32 { x + y * dim as u32 };

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
    let mut path = vec![];

    let mut result = 0;

    let step = |g: &i32, dir: i32, obs: Option<i32>| -> StepResult {

        StepResult::Ok(north)
    };

    let mut prev_guard_index = guard_index;
    while let StepResult::Ok(next_dir) = step(&guard_index, dir, None) {
        let obs = Some(guard_index);
        loop {
            match step(&prev_guard_index, dir, obs) {
                StepResult::Ok(next_dir) => dir = next_dir,
                StepResult::End => break,
                StepResult::LoopDetected => {
                    result += 1;
                    break;
                }
            }

        }
        prev_guard_index = guard_index;
    }
    
    loop {
        visited[guard_index as usize] |= dir_to_bitmask(dir);
        if guard_index == 15 {
            println!("d: {}, v: {}", dir, visited[guard_index as usize]);
        }

        // Peek next position to determine if there is an obstacle
        let next_index = (guard_index + dir) as usize;
        let Some(&b) = input.get(next_index) else {
            // Out of bounds
            break;
        };
        if b == b'.' || b == b'^' {
            path.push((guard_index as u32, dir));
            // Progress guard

            // CHECK
            let (x, y) = get_coords(guard_index as u32);
            let next_dir = get_next_dir(dir);
            println!("{x},{y}");

            if next_dir == north {
                let mut row_index = cols[x as usize].binary_search(&y).unwrap_err();
                if row_index == 0 {
                    guard_index += dir;
                    continue;
                }
                // if row_index == cols[x as usize].len() {
                //     continue;
                // }
                row_index -= 1;

                let check_row = cols[x as usize][row_index] + 1;
                let check_index = get_index(x, check_row);
                if dir_to_bitmask(north) == (visited[check_index as usize] & dir_to_bitmask(north))
                {
                    // Loop found!
                    println!("  north - {x},{check_row}");
                    result += 1;
                }
            } else if next_dir == east {
                // println!("EAST! i: {i}");
                let row_index = rows[y as usize].binary_search(&x).unwrap_err();
                if row_index == rows[y as usize].len() {
                    guard_index += dir;
                    continue;
                }
                let check_col = rows[y as usize][row_index] - 1;
                let check_index = get_index(check_col, y);
                if dir_to_bitmask(east) == (visited[check_index as usize] & dir_to_bitmask(east)) {
                    // Loop found!
                    println!("  east  - {check_col},{y}");

                    result += 1;
                }
            }
            if next_dir == south {
                let row_index = cols[x as usize].binary_search(&y).unwrap_err();
                if row_index == cols[x as usize].len() {
                    guard_index += dir;
                    continue;
                }
                let check_row = cols[x as usize][row_index] - 1;
                let check_index = get_index(x, check_row);
                if check_index >= visited.len() as u32 {
                    guard_index += dir;
                    continue;
                }
                if dir_to_bitmask(south) == (visited[check_index as usize] & dir_to_bitmask(south))
                {
                    // Loop found!
                    println!("  south - {x},{check_row}");
                    result += 1;
                }
            }
            if next_dir == west {
                let mut row_index = rows[y as usize].binary_search(&x).unwrap_err();
                if row_index == 0 {
                    guard_index += dir;
                    continue;
                }
                row_index -= 1;
                // if row_index == rows[y as usize].len() {
                //     continue;
                // }
                let check_col = rows[y as usize][row_index] + 1;
                let check_index = get_index(check_col, y);
                if dir_to_bitmask(west) == (visited[check_index as usize] & dir_to_bitmask(west)) {
                    // Loop found!
                    println!("  west  - {check_col},{y}");
                    result += 1;
                }
            }

            // END
            guard_index += dir;
        } else if b == b'#' {
            let (x, y) = get_coords(next_index as u32);
            if cols[x as usize].binary_search(&y).is_err() {
                cols[x as usize].push(y);
            }
            if rows[y as usize].binary_search(&x).is_err() {
                rows[y as usize].push(x);
            }

            // Obstacle! Turn right
            dir = get_next_dir(dir);
            println!("g: {}", guard_index);
            visited[guard_index as usize] |= dir_to_bitmask(dir);
        } else {
            // New line character, out of bounds
            break;
        }
    }
    println!("{:?}", path);

    println!("r: {:?}\nc: {:?}", rows, cols);

    // }

    println!(
        "we: {}, w: {}, e: {}, dim: {dim}",
        (west_m | east_m),
        west_m,
        east_m
    );

    for (i, &b) in visited.iter().enumerate() {
        if i % dim as usize == dim as usize - 1 {
            continue;
        }
        if i % dim as usize == 0 {
            println!();
        }
        let mut c = '+';
        if b == (west_m | east_m) || b == west_m || b == east_m {
            c = '-'
        } else if b == (north_m | south_m) || b == north_m || b == south_m {
            c = '|'
        } else if b == 0 {
            c = '.'
        }
        print!("{}", c);
    }
    println!();
    println!();


    Ok(result.to_string())
}

// fn step(input: &[u8], guard_index: &i32, obs: Option<i32>) -> StepResult {
    
    
//     StepResult::Ok
// }

#[derive(Eq, PartialEq)]
enum StepResult {
    Ok(i32),
    End,
    LoopDetected,
}

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
/*
[(4, 6), (4, 1), (8, 1), (8, 6), (2, 6), (2, 4), (6, 4), (6, 8), (1, 8), (1, 7), (7, 7)]
0, 5, 50, 99

6 ^,

expected:
3 north
2 west
1 south

mine:
3 north
2 west
2 south
1 east

east
south
north
west
north
north
south
west

*/
