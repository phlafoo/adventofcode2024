#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // We assume all numbers have 2 digits
    let mut result = 0;

    let mut lines = input.lines();
    let mut rules = vec![Vec::new(); 100];

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let a = line[..2].parse::<usize>().unwrap();
        let b = line[3..5].parse::<usize>().unwrap();
        rules[b].push(a);
    }

    'outer: for line in lines {
        let mut forbidden_lists: Vec<&Vec<_>> = vec![];
        let mut i = 0;
        while i < line.len() {
            let n = line[i..i + 2].parse::<usize>().unwrap();

            for &list in &forbidden_lists {
                if list.contains(&n) {
                    // Incorrect update
                    continue 'outer;
                }
            }
            forbidden_lists.push(&rules[n]);
            i += 3
        }
        // Correct update, add middle number
        let index = (i / 2) - 1;
        result += line[index..index + 2].parse::<usize>().unwrap();
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
