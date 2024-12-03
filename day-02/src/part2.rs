#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = input.lines().fold(0, |acc, report| {
        let levels = report
            .split_ascii_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        
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

fn safety_check(levels: &mut [i32]) -> bool {
    let range = match levels[0] - levels[1] {
        n if (-3..=-1).contains(&n) => -3..=-1, // Increasing
        n if (1..=3).contains(&n) => 1..=3,     // Decreasing
        _ => {
            return !safety_check(levels[1..]) || saf
        }, // Unsafe
    };

    for pair in levels.windows(2).skip(1) {
        if !range.contains(&(pair[0] - pair[1])) {
            return acc; // Unsafe
        }
    }
    acc + 1 // Safe
}

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
