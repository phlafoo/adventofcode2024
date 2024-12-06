#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Assumptions:
    //  - Every number has 2 digits.
    //  - The position of each number in the corrected update is fully determined by the rules,
    //    i.e. every pair of numbers has an associated rule.
    let mut result = 0;

    let mut lines = input.lines();
    let mut rules = vec![Vec::new(); 100];

    // Get all the rules
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let a = line[..2].parse::<usize>().unwrap();
        let b = line[3..5].parse::<usize>().unwrap();
        rules[b].push(a);
    }
    // Map a number to its count (# of times it appears in all the rules for this update)
    let mut counts: [usize; 100] = [0; 100];

    'outer: for line in lines {
        let mut update = vec![];

        // Collect numbers in this update
        let mut i = 0;
        while i < line.len() {
            update.push(line[i..i + 2].parse::<usize>().unwrap());
            i += 3
        }

        counts.fill(0);
        let last_index = update.len() - 1;

        // Count how many times each number appears in all the rules that apply to this update
        // (A rule only "applies" to this update if the second number in the rule is in this update)
        for &p in update.iter() {
            for rule in &rules[p] {
                counts[*rule] += 1;
            }
        }
        // `last_index` minus the count for that number is equal to the expected index
        for (i, p) in update.iter().enumerate() {
            if i != last_index - counts[*p] {
                continue 'outer;
            }
        }
        result += update[last_index / 2];
    }

    Ok(result.to_string())
}
// 6384

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
        assert_eq!("143", process(input)?);
        Ok(())
    }
}