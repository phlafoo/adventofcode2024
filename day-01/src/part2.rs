use fxhash::FxHashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Parse into flat list of numbers
    let mut iter = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap());

    let mut left_list = vec![];
    let mut count_map = FxHashMap::default();

    // Partition into left and right column (count map for right column)
    while let Some(a) = iter.next() {
        left_list.push(a);
        count_map
            .entry(iter.next().unwrap())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let result = left_list
        .into_iter()
        .fold(0, |acc, x| acc + x * count_map.get(&x).unwrap_or(&0));

    Ok(result.to_string())
}
// 18805872

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
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
