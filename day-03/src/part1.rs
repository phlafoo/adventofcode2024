#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;
    let mut i = 0;
    while i < input.len() - 7 {
        let Some((val0, val1)) = get_values(input, i) else {
            // I tried incrementing i according to the index of the mismatch but it slightly decreased performance.
            i += 1;
            continue;
        };
        result += val0 * val1;
        i += 9;
    }
    Ok(result.to_string())
}
// 189527826

fn get_values(input: &str, i: usize) -> Option<(i32, i32)> {
    if input[i..i + 4] != *"mul(" {
        return None;
    }
    // Find comma position
    let comma_offset = input[i + 5..].find(",")?;
    let comma_index = comma_offset + i + 5;

    // Get first value
    let val0 = input[i + 4..comma_index].parse::<i32>().ok()?;

    // Find right parenthesis position
    let right_paren_offset = input[comma_index + 2..].find(")")?;
    let right_paren_index = right_paren_offset + comma_index + 2;

    // Get second value
    let val1 = input[comma_index + 1..right_paren_index].parse::<i32>().ok()?;
    Some((val0, val1))
}

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
