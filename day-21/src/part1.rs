use crate::common::*;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = inner(input.as_bytes());
    Ok(result.to_string())
}
// 94284

pub const DEPTH: usize = 2;

fn inner(input: &[u8]) -> i64 {
    // The next optimization would be to use a simple lookup table for all possible numpad movements.

    let mut result = 0;

    // Each of the 15 possible patterns can be memoized at each depth of recursion
    let mut memo = [[0; DEPTH - 1]; 15];

    // Iterate codes
    for line in input[..input.len() - 1].split(|&b| b == b'\n') {
        // Value of this code (leading zeros ignored)
        let mut value = 0;

        // Starting position (A)
        let p0 = Vec2::new(2, 0);

        // Get position of first code digit
        let p1 = get_coords(line[0]);

        // Calculate number of steps to get from 'A' to the first code digit
        let mut total = get_shortest_seq_length(p0, p1, &mut memo);

        // Calculate step counts for subsequent movements
        for p in line.windows(2) {
            value = value * 10 + (p[0] - b'0') as i64;

            let p0 = get_coords(p[0]);
            let p1 = get_coords(p[1]);
            total += get_shortest_seq_length(p0, p1, &mut memo);
        }
        // Calculate complexity score
        result += total * value
    }

    result
}

/// Recursive function to calculate step count
fn rec(i: usize, depth: u32, memo: &mut [[i64; DEPTH - 1]; 15]) -> i64 {
    if depth == DEPTH as u32 - 1 {
        // Simply return number of steps for this subsequence if we are at max depth
        return STEP_COUNT[i];
    }
    // Check if we have already calculated the step count for this subsequence at this depth
    let m = memo[i][depth as usize];
    if m != 0 {
        return m;
    }
    let mut total = 0;
    for s in LUT[i] {
        if s == NONE {
            break;
        }
        total += rec(s, depth + 1, memo);
    }
    memo[i][depth as usize] = total;
    total
}

fn get_shortest_seq_length(p0: Vec2, p1: Vec2, memo: &mut [[i64; DEPTH - 1]; 15]) -> i64 {
    // Move priority: < ^ v >

    if p0 == p1 {
        return 1;
    }
    // Delta
    let d = p1 - p0;

    // Flip priority if the optimal sequence would pass over empty button (bottom left)
    let flip_priority = (p1.x == 0 && p0.y == 0) || (p0.x == 0 && p1.y == 0);

    // The calculations further down assume that 'A' is pressed one time, so this equation adds the
    // remaining 'A' button presses if there are any.
    let mut total = d.x.abs().max(1) + d.y.abs().max(1) - 2;

    // Get sequence of moves for the robot that controls the first robot at the numpad (first robot
    // with a d-pad)
    let sequence = get_move_sequence(d, flip_priority);

    // Execute first move
    total += rec(sequence.first, 0, memo);

    // Execute second move if it exists
    if let Some(second_move) = sequence.second {
        total += rec(second_move, 0, memo);
    }

    // Return to A
    total += rec(sequence.return_to_a, 0, memo);

    total
}
