#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = input.lines().fold(0, |acc, report| {
        let report = report
            .split_ascii_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        // The goal is to never clone the report by only using slices.
        get_error_index(&report, 1, (0, 1))
            .and_then(|i| {
                if i == report.len() - 2 {
                    None
                } else if i >= 2 {
                    get_error_index(&report[i - 2..], 2, (0, 1))
                        .and_then(|_| get_error_index(&report[i - 1..], 2, (0, 1)))
                } else if i == 1 {
                    get_error_index(&report, 1, (0, 2))
                        .and_then(|_| get_error_index(&report, 2, (0, 1)))
                        .and_then(|_| get_error_index(&report, 1, (1, 2)))
                } else {
                    get_error_index(&report, 1, (1, 2)).and_then(|_| get_error_index(&report, 1, (0, 2)))
                }
            })
            .map_or(acc + 1, |_| acc) // No error index means success => increment counter
    });

    Ok(result.to_string())
}
// 290

/// Each pair of adjacent number is checked and the index of the first number is returned on failure.
fn get_error_index(report: &[i32], step: usize, (l0, l1): (usize, usize)) -> Option<usize> {
    // l0 and l1 are the first 2 indices to check in the report.
    // `step` determines if the third number (index 2) will be skipped.
    // step = 1 ==> don't skip index 2
    // step = 2 ==> skip index 2

    // The first pair of report determines if increasing or decreasing (assumes report has at least 2 levels)
    let range = match report[l0] - report[l1] {
        n if (-3..=-1).contains(&n) => -3..=-1, // Increasing
        n if (1..=3).contains(&n) => 1..=3,     // Decreasing
        _ => {
            return Some(l0);
        }
    };
    // Now we check the next pair (note that step is 1 or 2)
    if !range.contains(&(report[l1] - report[l1 + step])) {
        return Some(l1);
    }
    // And then we check the rest of the pairs
    let len = report.len();
    (l1 + step..len - 1).find(|&i| !range.contains(&(report[i] - report[i + 1])))
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
