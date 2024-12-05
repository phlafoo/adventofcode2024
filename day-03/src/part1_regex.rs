use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Regex is very slow
    
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result = re
        .captures_iter(input)
        .map(|c| c.extract().1)
        .fold(0, |acc, g: [&str; 2]| acc + g[0].parse::<i32>().unwrap() * g[1].parse::<i32>().unwrap());

    Ok(result.to_string())
}
// 189527826

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        // 2*4, 5*5, 11*8, 8*5
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
