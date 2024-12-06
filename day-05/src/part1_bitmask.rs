#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Assumptions:
    //  - Every number has 2 digits (10-99).
    //  - The position of each number in the corrected update is fully determined by the rules,
    //    i.e. every pair of numbers has an associated rule.
    let mut result = 0;

    // Indexed by the "before" numbers, each element is bitmask representing all the numbers that must go after.
    let mut rules = vec![0_u128; 100];
    let mut lines = input.lines().map(str::as_bytes);

    // Get all the rules
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let a = parse_u32(line);
        let b = parse_u32(&line[3..5]);
        rules[a as usize] |= 1 << b; // Set corresponding bit
    }

    // Iterate updates
    'outer: for line in lines {
        // Collect page numbers in this update
        let update = line.chunks(3).map(parse_u32).collect::<Vec<_>>();

        // The expected index of each number can be calculated by counting how many times the number
        // appears in all the rules associated with this update. An incorrect update will have at
        // least 2 numbers in the wrong position, so the first number in the update can be skipped.
        for (i, p0) in update.iter().enumerate().skip(1) {
            let mut count = 0;
            for p1 in update.iter() {
                count += ((rules[*p1 as usize] >> p0) & 1) as u32;
            }
            if i as u32 != count {
                // Expected index does not match actual index, so this update is incorrect
                continue 'outer;
            }
        }
        // Add middle value to the result since the update was correct
        result += update[update.len() / 2];
    }

    Ok(result.to_string())
}
// 6384

/// Parses the first 2 bytes of a byte slice into a u32. Panics if < 2 bytes in slice.
#[inline(always)]
fn parse_u32(bytes: &[u8]) -> u32 {
    ((bytes[0] - b'0') * 10 + bytes[1] - b'0') as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
