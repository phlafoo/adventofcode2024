use crate::common::process_usize;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .fold(0, |acc, n| acc + process_usize(n, 25));
    Ok(result.to_string())
}
// 185894

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
