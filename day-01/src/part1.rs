#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Parse into flat list of numbers
    let mut iter = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap());

    let mut left_list = vec![];
    let mut right_list = vec![];

    // Partition into left and right column
    while let Some(a) = iter.next() {
        left_list.push(a);
        right_list.push(iter.next().unwrap());
    }

    left_list.sort();
    right_list.sort();

    // Iterate pairs
    let result: u32 = left_list
        .iter()
        .zip(right_list)
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    Ok(result.to_string())
}
// 3714264

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
