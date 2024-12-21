use core::str;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = unsafe { inner(input.as_bytes()) };
    Ok(result.to_string())
}
// 636483903099279

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

unsafe fn inner(input: &[u8]) -> u64 {
    // `towels` is a tree structure.
    let mut towels = vec![Towel::new()];
    let mut k = 0;

    'outer: loop {
        // Get stripe index for the first stripe of this pattern (b => 0, g => 1, etc.)
        let mut i = 0;
        loop {
            let n = get_stripe_index(input[k]);

            // Get next stripe index
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
            k += 1;
            // If the next char is a comma or newline, we have reached the end of this pattern
            if input[k] == b',' {
                towels[i].end = true;
                break;
            }
            if input[k] == b'\n' {
                towels[i].end = true;
                break 'outer;
            }
        }
        k += 2;
    }

    k += 2;
    let mut result = 0;

    const MAX_DESIGN_LEN: usize = 60;
    const MAX_TOWEL_LEN: usize = 8;

    let mut combinations = [0; MAX_DESIGN_LEN + 1];
    combinations[0] = 1;

    // Iterate designs
    for design in input[k..input.len() - 1].split(|&b| b == b'\n') {
        for len in 1..=design.len() {
            combinations[len] = 0;
            let mut i = 0;

            // Progressively check larger portions of this design, starting with the last stripe
            for (di, &d) in design[design.len() - len..]
                .iter()
                .take(MAX_TOWEL_LEN)
                .enumerate()
            {
                match towels[i].children[get_stripe_index(d)] {
                    Some(c) => i = c,
                    None => break,
                }
                if towels[i].end {
                    combinations[len] += combinations[len - 1 - di];
                }
            }
        }
        result += combinations[design.len()];
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
        assert_eq!("16", process(input)?);
        Ok(())
    }
}
