#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = input.lines().fold(0, |acc, report| {
        let levels = report
            .split_ascii_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        
        // Assume reports have at least 2 levels
        let range = match levels[0] - levels[1] {
            n if (-3..=-1).contains(&n) => -3..=-1, // Increasing
            n if (1..=3).contains(&n) => 1..=3,     // Decreasing
            _ => return acc, // Unsafe
        };

        for pair in levels.windows(2).skip(1) {
            if !range.contains(&(pair[0] - pair[1])) {
                return acc; // Unsafe
            }
        }
        acc + 1 // Safe
    });

    Ok(result.to_string())
}
// 218

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
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
