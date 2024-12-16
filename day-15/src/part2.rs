#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = unsafe { inner(input.as_bytes()) };
    Ok(result.to_string())
}
// 1492011

const ROBOT: u8 = b'@';
const WALL: u8 = b'#';
const BOX: u8 = b'O';
const AIR: u8 = b'.';

const BOX_LEFT: u8 = b'[';
const BOX_RIGHT: u8 = b']';

// Hardcoded dimensions
const HEIGHT: usize = 50;
const WIDTH: usize = HEIGHT * 2;

unsafe fn inner(input: &[u8]) -> usize {
    // let height = input.iter().position(|b| b == &b'\n').unwrap();
    // let width = height * 2;

    let mut rob = 0;

    let north = -(WIDTH as i32);
    let south = WIDTH as i32;
    let west = -1;
    let east = 1;

    // Faster direction lookup
    let mut move_map = [0; 167];
    move_map[b'<' as usize] = west;
    move_map[b'>' as usize] = east;
    move_map[b'^' as usize] = north;
    move_map[b'v' as usize] = south;

    // Mutable version of traversable area
    let mut grid = [0; HEIGHT * WIDTH];

    // Build grid from input
    let mut gi = 0;
    let mut i = 0;
    let i_max = (HEIGHT + 1) * HEIGHT;
    loop {
        match input[i] {
            AIR => {
                grid[gi] = AIR;
                grid[gi + 1] = AIR;
                gi += 2;
            }
            BOX => {
                grid[gi] = BOX_LEFT;
                grid[gi + 1] = BOX_RIGHT;
                gi += 2;
            }
            WALL => {
                grid[gi] = WALL;
                grid[gi + 1] = WALL;
                gi += 2;
            }
            ROBOT => {
                rob = gi as i32;
                grid[gi] = ROBOT;
                grid[gi + 1] = AIR;
                gi += 2;
            }
            _ => (),
        }
        i += 1;
        if i == i_max {
            break;
        }
    }

    let moves = &input[HEIGHT * (HEIGHT + 1) + 1..];

    // Process moves
    for &m in moves.iter().filter(|&m| *m != b'\n') {
        let dir = *move_map.get_unchecked(m as usize);

        // Index of next robot position
        let mut i = rob + dir;

        // Horizontal moves are handled same as part 1
        if dir.abs() == 1 {
            // Step in direction until air is reached, then loop backwards moving the boxes and robot
            'outer: loop {
                match grid[i as usize] {
                    AIR => loop {
                        *grid.get_unchecked_mut(i as usize) =
                            *grid.get_unchecked((i - dir) as usize);
                        if *grid.get_unchecked(i as usize) == ROBOT {
                            *grid.get_unchecked_mut((i - dir) as usize) = AIR;
                            rob = i;
                            break 'outer;
                        }
                        i -= dir;
                    },
                    BOX_LEFT | BOX_RIGHT => i += dir,
                    WALL => break,
                    t => unreachable!(
                        "TILE: {}\n\nGRID:\n{}",
                        t as char,
                        get_grid_string(&grid, WIDTH)
                    ),
                }
            }
            continue;
        }
        // Vertical is more complicated due to potential partial overlap of boxes
        if match *grid.get_unchecked(i as usize) {
            AIR => {
                grid.swap(i as usize, rob as usize);
                true
            }
            WALL => false,
            BOX_LEFT => move_boxes(&mut grid, i, i, dir),
            BOX_RIGHT => move_boxes(&mut grid, i - 1, i, dir),
            t => unreachable!(
                "ROB: {rob}, i:{i}, TILE: {}\n\nGRID:\n{}",
                t as char,
                get_grid_string(&grid, WIDTH)
            ),
        } {
            // Success! Update robot position
            rob = i;
        }
    }

    grid.iter()
        .enumerate()
        .filter(|(_, &t)| t == BOX_LEFT)
        .fold(0, |acc, (i, _)| {
            let y_score = 100 * (i / WIDTH);
            let x_score = i % WIDTH;
            acc + y_score + x_score
        })
}

/// Returns true if boxes were moved.
#[inline(always)]
unsafe fn move_boxes(grid: &mut [u8], left: i32, i: i32, dir: i32) -> bool {
    // Store indices of left part of boxes to be moved
    let mut swaps = [0; 32];
    let mut s_len = 0;

    if rec(grid, &mut swaps, &mut s_len, left, dir) {
        // Check was successful. Now the boxes get moved.
        for &s in swaps.iter().take(s_len) {
            grid.swap(s as usize, (s - dir) as usize);
            grid.swap((s + 1) as usize, (s + 1 - dir) as usize);
        }
        // Move robot
        grid.swap(i as usize, (i - dir) as usize);
        return true;
    }
    false
}

/// Recursive call that returns true if a move can be made.
#[inline(always)]
unsafe fn rec(grid: &mut [u8], swaps: &mut [i32], s_len: &mut usize, left: i32, dir: i32) -> bool {
    // Step in direction
    let left = left + dir;

    // It's possible we have already checked this location for this move
    if swaps[..*s_len].contains(&left) {
        return true;
    }
    // Make appropriate call based on what is in the path of movement
    if match (
        *grid.get_unchecked(left as usize),
        *grid.get_unchecked((left + 1) as usize),
    ) {
        (AIR, AIR) => true,
        (WALL, _) | (_, WALL) => false,
        (BOX_LEFT, _) => rec(grid, swaps, s_len, left, dir),
        (AIR, BOX_LEFT) => rec(grid, swaps, s_len, left + 1, dir),
        (BOX_RIGHT, AIR) => rec(grid, swaps, s_len, left - 1, dir),
        (BOX_RIGHT, BOX_LEFT) => {
            rec(grid, swaps, s_len, left - 1, dir) && rec(grid, swaps, s_len, left + 1, dir)
        }
        (a, b) => unreachable!("Unexpected pattern: \"{}{}\"", a as char, b as char),
    } {
        // Track the left part of the box so we can move it later
        *swaps.get_unchecked_mut(*s_len) = left;
        *s_len += 1;
        return true;
    }
    // Reached a wall! `swaps` might have some indices, but if any call returns false then the move fails.
    false
}

fn get_grid_string(grid: &[u8], width: usize) -> String {
    grid.chunks(width).fold("".to_string(), |acc, row| {
        format!(
            "{}\n{}",
            acc,
            row.iter()
                .fold("".to_string(), |acc, &t| format!("{}{}", acc, t as char))
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // Update dimensions for testing
        //   Height: 10
        let input = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
        assert_eq!("9021", process(input)?);
//         let input = "\
// ########
// #......#
// ##@.O..#
// #...O..#
// #.#.O..#
// #...O..#
// #......#
// ########

// ^^
// ";
        //v>>>><vvvv>>>>^^<<>>>
        //>^^^<<<<<vvv<vv<vv>^^vv>>^^^^
        // assert_eq!("2028", process(input)?);
        //         let input = "\
        // ########
        // #..O.O.#
        // ##@.O..#
        // #...O..#
        // #.#.O..#
        // #...O..#
        // #......#
        // ########

        // <^^>>>vv<v>>v<<
        // ";
        //         assert_eq!("2028", process(input)?);
        Ok(())
    }
}

/*
################
##..@.[]..[]..##
####....[]....##
##......[]....##
##..##..[]....##
##......[]....##
##............##
################


####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##


*/
