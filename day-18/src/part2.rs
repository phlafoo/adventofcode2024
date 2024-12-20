#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = unsafe { inner_part2(input.as_bytes()) };
    Ok(result)
}
// 5,60
// input2: 20,64

// const DIM: i32 = 9;
// const BYTE_TOTAL: i32 = 12;
const DIM: i32 = 73;
const BYTE_TOTAL: i32 = 1024;

const AIR: u8 = 0;
const BYTE: u8 = 1;

const NORTH: i32 = -DIM;
const SOUTH: i32 = DIM;
const WEST: i32 = -1;
const EAST: i32 = 1;

const NEIGHBORS: [i32; 8] = [
    NORTH,
    NORTH + EAST,
    EAST,
    SOUTH + EAST,
    SOUTH,
    SOUTH + WEST,
    WEST,
    NORTH + WEST,
];

/// Using an array for direction lookup is slightly faster (assuming DIM is not too large)
const DIRS: [[i32; 3]; DIM as usize * 2 + 1] = get_dir_array();

/// Lookup for list of directions to check based on previous direction
const fn get_dir_array() -> [[i32; 3]; (DIM * 2 + 1) as usize] {
    let mut dirs = [[0; 3]; (DIM * 2 + 1) as usize];
    dirs[(NORTH + DIM) as usize] = [NORTH, EAST, WEST];
    dirs[(SOUTH + DIM) as usize] = [SOUTH, EAST, WEST];
    dirs[(EAST + DIM) as usize] = [EAST, NORTH, SOUTH];
    dirs[(WEST + DIM) as usize] = [WEST, NORTH, SOUTH];
    dirs
}

/// Adds padding to remove bounds checking
const fn get_grid() -> [u8; (DIM * DIM) as usize] {
    let mut grid = [AIR; (DIM * DIM) as usize];
    let mut i = 0;
    loop {
        if i == DIM {
            break;
        }
        grid[i as usize] = BYTE;
        grid[(i + DIM * (DIM - 1)) as usize] = BYTE;
        grid[(i * DIM) as usize] = BYTE;
        grid[(i * DIM + DIM - 1) as usize] = BYTE;
        i += 1;
    }
    grid
}

#[inline(always)]
fn is_passable(mask: i32) -> bool {
    // For certain combinations of neighbors around a new "byte" we don't have to recheck if there
    // is still a path.
    !(((0b0100_0100 & mask == 0) && (0b1000_0011 & mask != 0) && (0b0011_1000 & mask != 0))
        || ((0b0001_0001 & mask == 0) && (0b1110_0000 & mask != 0) && (0b0000_1110 & mask != 0))
        || ((0b0000_0101 & mask == 0) && (0b0000_0010 & mask != 0) && (0b1111_1000 & mask != 0))
        || ((0b0001_0100 & mask == 0) && (0b0000_1000 & mask != 0) && (0b1110_0011 & mask != 0))
        || ((0b0101_0000 & mask == 0) && (0b0010_0000 & mask != 0) && (0b1000_1111 & mask != 0))
        || ((0b0100_0001 & mask == 0) && (0b1000_0000 & mask != 0) && (0b0011_1110 & mask != 0)))
}

unsafe fn inner_part2(input: &[u8]) -> String {
    let mut grid: [u8; (DIM * DIM) as usize] = get_grid();

    let mut ptr = input.as_ptr();
    let mut byte_count = 0;

    // Parse bytes and fill grid
    while byte_count < BYTE_TOTAL {
        byte_count += 1;
        let x = match *ptr.add(1) {
            b',' => *ptr - b'0',
            _ => {
                let x = (*ptr - b'0') * 10 + *ptr.add(1) - b'0';
                ptr = ptr.add(1);
                x
            }
        };
        ptr = ptr.add(2);
        let y = match *ptr.add(1) {
            b'\n' => *ptr - b'0',
            _ => {
                let x = (*ptr - b'0') * 10 + *ptr.add(1) - b'0';
                ptr = ptr.add(1);
                x
            }
        };
        ptr = ptr.add(2);
        let i = (y as i32 + 1) * DIM + x as i32 + 1;
        grid[i as usize] = BYTE;
    }
    const START: i32 = DIM + 1;
    const END: i32 = DIM * (DIM - 1) - 2;

    'outer: loop {
        // Parse coords of new "byte"
        let x = match *ptr.add(1) {
            b',' => *ptr - b'0',
            _ => {
                let x = (*ptr - b'0') * 10 + *ptr.add(1) - b'0';
                ptr = ptr.add(1);
                x
            }
        };
        ptr = ptr.add(2);
        let y = match *ptr.add(1) {
            b'\n' => *ptr - b'0',
            _ => {
                let x = (*ptr - b'0') * 10 + *ptr.add(1) - b'0';
                ptr = ptr.add(1);
                x
            }
        };
        ptr = ptr.add(2);
        let i = (y as i32 + 1) * DIM + x as i32 + 1;

        grid[i as usize] = BYTE;

        // Check neighbors to skip pathfinding if possible
        let mut mask = 0;
        for (di, d) in NEIGHBORS.iter().enumerate() {
            let n = i + d;
            if grid[n as usize] == BYTE {
                mask |= 1 << di;
            }
        }
        if is_passable(mask) {
            continue;
        }

        let mut visited = [false; (DIM * DIM) as usize];

        let mut open = [(0, 0); 1024];
        open[0] = (END, WEST);
        let mut open_len = 1;

        while open_len > 0 {
            let (i, dir) = open[open_len - 1];
            open_len -= 1;
            for d in DIRS[(dir + DIM) as usize] {
                let n = i + d;
                if grid[n as usize] == BYTE || visited[n as usize] {
                    continue;
                }
                if n == START {
                    // Path found, check next byte
                    continue 'outer;
                }
                visited[n as usize] = true;
                open[open_len] = (n, d);
                open_len += 1;
            }
        }
        return format!("{x},{y}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";
        assert_eq!("6,1", process(input)?);
        Ok(())
    }
}
