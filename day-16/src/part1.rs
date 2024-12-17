#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = unsafe { inner(input.as_bytes()) };

    Ok(result.to_string())
}
// 92432

const DIM: i32 = 142;
// const DIM: i32 = 18;
// const DIM: i32 = 16;

const NORTH: i32 = -DIM;
const SOUTH: i32 = DIM;
const WEST: i32 = -1;
const EAST: i32 = 1;

const WALL: u8 = b'#';

unsafe fn inner(input: &[u8]) -> u32 {
    // let dim = input.iter().position(|&b| b == b'\n').unwrap() + 1;

    // Using an array for direction lookup is slightly faster (assuming DIM is not too large)
    const DIRS: [[i32; 3]; DIM as usize * 2 + 1] = get_dir_array();

    // Start and end index
    const START: i32 = (DIM - 1) * (DIM - 2) - 1;
    const END: i32 = (DIM - 1) * 2 - 1;

    // Store cost at each position for Dijkstra's algo
    let mut cost_grid = [u32::MAX; (DIM * (DIM - 1)) as usize];
    cost_grid[START as usize] = 0;

    // Hopefully 512 is enough
    let mut open = [(0, 0, 0); 512];
    open[0] = (START, EAST, 0);
    let mut open_len = 1;

    while open_len > 0 {
        // Pop from open (should be lowest cost)
        let (i, dir, c) = *open.get_unchecked(open_len - 1);

        // Check neighbors. The first neighbor continues in the same direction with step cost of 1,
        // the last 2 neighbors are making a 90 degree turn, so they have cost 1001.
        let mut step_cost = 1;
        for d in *DIRS.get_unchecked((dir + DIM) as usize) {
            // Neighbor index
            let n = i + d;

            if *input.get_unchecked(n as usize) == WALL {
                step_cost = 1001;
                continue;
            }
            // Calculate new total cost
            let new_cost = c + step_cost;

            if n == END {
                // Since `open` is always sorted, we know we have the minimum solution
                return new_cost;
            }

            // If this tile has been visited and has a lower cost AND the next tile in the same
            // direction also has a lower cost then it can be skipped. The second check is because
            // the 1000 cost penalty is only incurred *after* the turn is made, not *on* the tile
            // where the turn actually happened.
            if new_cost >= *cost_grid.get_unchecked(n as usize)
                && new_cost + 1 >= *cost_grid.get_unchecked((n + d) as usize)
            {
                step_cost = 1001;
                continue;
            }
            // Update cost at this tile
            *cost_grid.get_unchecked_mut(n as usize) = new_cost;

            // `i` is the index where the next node will be inserted in `open`. If we are not turning
            // then we always add the node at the end of open, to be popped immidiately. This only works
            // because the cost of turning is 1000.
            let mut i = open_len - 1;

            if step_cost == 1001 {
                // We are turning so we have to add the new node to open and then sort the list.
                // Since we immediately pop nodes that were not on a turn, the `open` list is always
                // sorted at this point in the code. This means we can use binary search to find the
                // next insertion index, and then use std::ptr::copy to move the elements over for 
                // efficient insertion. This bring our time down to 0.5ms down from >3ms with normal
                // sorting.
                i = open[..open_len - 1]
                    .binary_search_by(|(_, _, c)| new_cost.cmp(c))
                    .unwrap_or_else(|e| e);

                // Move elements one step to the right to make a space for the new one
                let ptr = open.as_mut_ptr();
                std::ptr::copy(ptr.add(i), ptr.add(i + 1), open_len - i - 1);
            }
            // Finally insert the new node
            *open.get_unchecked_mut(i) = (n, d, new_cost);

            open_len += 1;
            step_cost = 1001;
        }
        // Decrement due to the pop at the top of the while loop
        open_len -= 1;
    }
    // End was not found
    unreachable!();
}
/*
no sort, steps: 7208385
sort, steps: 90070

92 turns

m: 261, ts: 9228, t: 1263113, avg:136.87831

w/ stack
m: 81, ts: 2809, t: 106055, avg:37.75543

*/

const fn get_dir_array() -> [[i32; 3]; (DIM * 2 + 1) as usize] {
    let mut dirs = [[0; 3]; (DIM * 2 + 1) as usize];
    dirs[(NORTH + DIM) as usize] = [NORTH, EAST, WEST];
    dirs[(SOUTH + DIM) as usize] = [SOUTH, EAST, WEST];
    dirs[(EAST + DIM) as usize] = [EAST, NORTH, SOUTH];
    dirs[(WEST + DIM) as usize] = [WEST, NORTH, SOUTH];
    dirs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_process() -> miette::Result<()> {
        // dim = 16
        let input = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
        assert_eq!("7036", process(input)?);
        Ok(())
    }

    // #[ignore]
    #[test]
    fn test_process1() -> miette::Result<()> {
        // dim = 18
        let input = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";
        assert_eq!("11048", process(input)?);
        Ok(())
    }

    #[ignore]
    #[test]
    fn test_process2() -> miette::Result<()> {
        // dim = 16
        let input = "\
###############
#...#........E#
#.###.#######.#
#.#.#.#.....#.#
#.#.#.#.....#.#
#.#.#.#.....#.#
#.#.#.#.....#.#
#.#.#.#.....#.#
#.#.#.#...###.#
#.#.#.#...#...#
#.#.#.#####.###
#.#...#...#...#
#.#####.#.###.#
#S......#.....#
###############
";
        // 31 vs 43
        // 9  vs 6
        // 12 + 1000 + 12 = 1024
        assert_eq!("1024", process(input)?);
        Ok(())
    }
}
