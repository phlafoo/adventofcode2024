#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = unsafe { inner(input.as_bytes()) };
    Ok(result.to_string())
}
// 285

const fn get_stripe_map() -> [usize; 120] {
    let mut map = [0; 120];
    map[b'b' as usize] = 0;
    map[b'g' as usize] = 1;
    map[b'r' as usize] = 2;
    map[b'u' as usize] = 3;
    map[b'w' as usize] = 4;
    map
}

const STRIPE_MAP: [usize; 120] = get_stripe_map();

#[derive(Debug, Clone)]
struct Towel {
    /// Is true if there is a stripe pattern that ends at this node
    end: bool,
    /// Indices in the tree structure for subsequent stripes accross all towel options
    children: [Option<usize>; 5],
}

impl Towel {
    fn new() -> Self {
        Towel {
            end: false,
            children: [None; 5],
        }
    }
}

/// Maps bgruw to 01234 respectively
#[inline(always)]
fn get_stripe_index(s: u8) -> usize {
    STRIPE_MAP[s as usize]
}

unsafe fn inner(input: &[u8]) -> i32 {
    // Get start and end ptr
    let std::ops::Range {
        start: mut ptr,
        end,
    } = input.as_ptr_range();

    // `towels` is a tree structure. The first 5 nodes are the root nodes representing the 5 stripe colors
    let mut towels = vec![Towel::new(); 5];

    'outer: loop {
        // Get stripe index for the first stripe of this pattern (b => 0, g => 1, etc.)
        let mut i = get_stripe_index(*ptr);
        loop {
            // If the next char is a comma or newline, we have reached the end of this pattern
            ptr = ptr.add(1);
            if *ptr == b',' {
                towels[i].end = true;
                break;
            }
            if *ptr == b'\n' {
                towels[i].end = true;
                break 'outer;
            }

            // Get next stripe index
            let n = get_stripe_index(*ptr);
            let towels_len = towels.len();

            // If this stripe is already a child of the previous stripe we just need to update `i`
            match towels[i].children[n] {
                Some(c) => i = c,
                None => {
                    // Otherwise we add it as a child
                    towels[i].children[n] = Some(towels_len);
                    i = towels_len;
                    towels.push(Towel::new());
                }
            }
        }
        ptr = ptr.add(2);
    }

    ptr = ptr.add(2);
    let mut result = 0;

    'outer: loop {
        // Get stripe index of first char in this design
        let mut i = get_stripe_index(*ptr);

        // (index into towels, pointer to input)
        let mut stack = [(0, std::ptr::null()); 64];
        let mut stack_len = 0;

        loop {
            ptr = ptr.add(1);
            if *ptr == b'\n' {
                // If next char is newline and the current stripe is an end of a pattern then we can
                // break (success)
                if towels[i].end {
                    result += 1;
                    break;
                }
                // Otherwise we continue looking
                match stack_len {
                    // Unless there are no more patterns to check, then we failed to match
                    0 => break,
                    _ => {
                        stack_len -= 1;
                        (i, ptr) = stack[stack_len];
                        continue;
                    }
                }
            }
            // Get next stripe index
            let n = get_stripe_index(*ptr);
            if towels[i].end {
                // If this is the end of pattern, we might have to come back to it later
                stack[stack_len] = (n, ptr);
                stack_len += 1;
            }
            match towels[i].children[n] {
                Some(c) => {
                    // If this stripe is a child of the previous stripe then we simply continue with this pattern
                    i = c;
                }
                None => {
                    // Otherwise there was a mismatch, so we continue with the last valid pattern from the stack
                    match stack_len {
                        // Empty stack means we exhausted all options
                        0 => break,
                        _ => {
                            stack_len -= 1;
                            (i, ptr) = stack[stack_len];
                        }
                    }
                }
            }
        }
        // Skip to next design
        while *ptr != b'\n' {
            ptr = ptr.add(1);
        }
        ptr = ptr.add(1);
        if ptr == end {
            break 'outer;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
