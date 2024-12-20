#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = unsafe { inner_part1(input.as_bytes()) };
    Ok(result.to_string())
}
// 252

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

unsafe fn inner_part1(input: &[u8]) -> i32 {
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

    let mut cost_grid = [i32::MAX; (DIM * DIM) as usize];
    cost_grid[START as usize] = 0;
    let mut open = [(0, 0, 0); 128];
    open[0] = (START, EAST, 0);
    let mut open_len = 1;

    while open_len > 0 {
        let (i, dir, c) = open[open_len - 1];
        open_len -= 1;
        let new_cost = c + 1;

        for d in DIRS[(dir + DIM) as usize] {
            let n = i + d;
            if grid[n as usize] == BYTE {
                continue;
            }
            if new_cost < cost_grid[n as usize] {
                cost_grid[n as usize] = new_cost;

                // `open` is already sorted, so before we add the new element we can use binary search
                // to find the insertion index, then move the items to the right over. 5x speed-up
                // over using normal sort.
                let s = open[..open_len]
                    .binary_search_by(|(_, _, c)| new_cost.cmp(c))
                    .unwrap_or_else(|e| e);
    
                // Move elements one step to the right to make a space for the new one
                let ptr = open.as_mut_ptr();
                std::ptr::copy(ptr.add(s), ptr.add(s + 1), open_len - s);

                // Insert new item
                *open.get_unchecked_mut(s) = (n, d, new_cost);
                open_len += 1;
            }
            if n == END {
                return new_cost;
            }
        }
    }
    unreachable!();
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
        assert_eq!("22", process(input)?);
        Ok(())
    }
}
