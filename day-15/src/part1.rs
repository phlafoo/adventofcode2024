#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = unsafe { inner(input.as_bytes()) };
    Ok(result.to_string())
}
// 1486930

unsafe fn inner(input: &[u8]) -> usize {
    const ROBOT: u8 = b'@';
    const WALL: u8 = b'#';
    const BOX: u8 = b'O';
    const AIR: u8 = b'.';

    const DIM: usize = 50 + 1;
    
    // let dim = input.iter().position(|b| b == &b'\n').unwrap() + 1;

    // Robot index
    let mut rob = input.iter().position(|b| b == &ROBOT).unwrap() as i32;

    let north = -(DIM as i32);
    let south = DIM as i32;
    let west = -1;
    let east = 1;

    // Faster direction lookup
    let mut move_map = [0; 167];
    move_map[b'<' as usize] = west;
    move_map[b'>' as usize] = east;
    move_map[b'^' as usize] = north;
    move_map[b'v' as usize] = south;

    let mut grid = input[..DIM * (DIM - 1)].to_vec();
    let moves = &input[DIM * (DIM - 1) + 1..];

    for &m in moves.iter().filter(|&m| *m != b'\n') {
        let dir = move_map.get_unchecked(m as usize);
        let mut i = rob + dir;

        // Step in direction until air is reached, then loop backwards moving the boxes and robot
        'outer: loop {
            match grid[i as usize] {
                AIR => loop {
                    *grid.get_unchecked_mut(i as usize) = *grid.get_unchecked((i - dir) as usize);
                    if *grid.get_unchecked(i as usize) == ROBOT {
                        *grid.get_unchecked_mut((i - dir) as usize) = AIR;
                        rob = i;
                        break 'outer;
                    }
                    i -= dir;
                },
                BOX => i += dir,
                WALL => break,
                t => unreachable!("TILE: {}\n\nGRID:\n{}", t as char, get_grid_string(&grid)),
            }
        }
    }

    grid.iter()
        .enumerate()
        .filter(|(_, &t)| t == BOX)
        .fold(0, |acc, (i, _)| {
            let y_score = 100 * (i / DIM);
            let x_score = i % DIM;
            acc + y_score + x_score
        })
}

fn get_grid_string(grid: &[u8]) -> String {
    grid.iter()
        .fold("".to_string(), |acc, &t| format!("{}{}", acc, t as char))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
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
        assert_eq!("10092", process(input)?);
        let input = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";
        assert_eq!("2028", process(input)?);
        Ok(())
    }
}
