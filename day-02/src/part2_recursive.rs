#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = input.lines().fold(0, |acc, report| {
        let mut report = report
            .split_ascii_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if safety_check(&mut report, None, 0) {
            acc + 1 // Safe
        } else {
            acc // Unsafe
        }
    });

    Ok(result.to_string())
}
// 290

/// Recursive function to check safety of report.
fn safety_check(report: &mut Vec<i32>, removal_index: Option<usize>, depth: u8) -> bool {
    // Base case (only want to remove 1 level from the report)
    if depth > 1 {
        return false; // Unsafe
    }

    // Some reference shuffling to reduce clones
    let mut report_removed;
    let report = if let Some(index) = removal_index {
        report_removed = report.clone();
        report_removed.remove(index);
        &mut report_removed
    } else {
        report
    };
    // The first pair of report determines if increasing or decreasing (assumes report has at least 2 levels)
    let range = match report[0] - report[1] {
        n if (-3..=-1).contains(&n) => -3..=-1, // Increasing
        n if (1..=3).contains(&n) => 1..=3,     // Decreasing
        _ => {
            // Unsafe, try again with first or second level removed
            return safety_check(report, Some(0), depth + 1)
                || safety_check(report, Some(1), depth + 1);
        }
    };

    let len = report.len();
    for i in 1..len - 1 {
        if !range.contains(&(report[i] - report[i + 1])) {
            // Unsafe, try again with either level removed (or first level removed if on 2nd pair)
            return safety_check(report, Some(i), depth + 1)
                || safety_check(report, Some(i + 1), depth + 1)
                || {
                    if i == 1 {
                        safety_check(report, Some(i - 1), depth + 1)
                    } else {
                        false // Definitely unsafe
                    }
                };
        }
    }
    true // Safe
}
// 290

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
