#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input.as_bytes();

    // True if the file has been moved
    let mut moved = vec![false; input.len()];

    let mut left_id = 0;
    let mut pos = 0; // Index of the rearranged list of IDs
    let mut left = 0; // Left index

    let calculate_step = |id, count, pos| -> usize {
        (id * count * (count + 2 * pos - 1)) / 2
    };
    
    // Indexed by file size. Indicates where in the input the leftmost file was for a given size.
    let mut cache = [input.len() - 1; 10];
    let update_cache = |cache: &mut [usize; 10], i, val| {
        for c in cache.iter_mut().take(i + 1) {
            *c = (*c).min(val);
        }
    };
    let mut result = 0;

    while left != input.len() - 1 {
        let left_size = parse_u8(input[left]);

        // Add to result if this file hasn't been moved
        if !moved[left] {
            result += calculate_step(left_id, left_size, pos);
        }
        left_id += 1;
        left += 1;
        pos += left_size;

        let mut hole = parse_u8(input[left]);
        let mut right = cache[hole]; // Start at leftmost position of last known same sized file
        let mut right_id = right / 2 + 1;

        // Fill hole
        while right > left {
            right_id -= 1;

            // Skip if moved already
            if moved[right] {
                right -= 2;
                continue;
            }
            // Get size of file
            let right_size = parse_u8(input[right]);

            // If it fits it sits
            if right_size <= hole {
                update_cache(&mut cache, hole, right);
                
                result += calculate_step(right_id, right_size, pos);
                pos += right_size;

                // Hole gets smaller
                hole -= right_size;
                moved[right] = true;

                if hole == 0 {
                    // Hole filled
                    break;
                }
            }
            // Hole not filled, keep searching
            right -= 2;
        }
        // If hole still wasn't filled, then there are no more files <= the hole size
        if hole != 0 {
            update_cache(&mut cache, hole, 0);
            pos += hole;
        }
        left += 1;
    }

    Ok(result.to_string())
}
// 6382582136592

#[inline(always)]
fn parse_u8(b: u8) -> usize {
    (b - b'0') as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
/*

00...111...2...333.44.5555.6666.777.888899
0099.111...2...333.44.5555.6666.777.8888..
0099.1117772...333.44.5555.6666.....8888..
0099.111777244.333....5555.6666.....8888..
00992111777.44.333....5555.6666.....8888..

..992111777.44.333....5555.6666.....8888

9*2+9*3+2*4+1*5+1*6+1*7+7*8+7*9+7*10+4*12+4*13+3*15+3*16+3*17+5*22+5*23+5*24+5*25 +6*27+6*28+6*29+6*30 + 8*36 + 8*37 + 8*38 + 8*39

*/
