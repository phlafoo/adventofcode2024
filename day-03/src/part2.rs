#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;

    let mut do_mul = true;
    let mut i = 0;
    
    while i < input.len() - 7 {
        if do_mul {
            let Some((val0, val1)) = get_values(input, i) else {
                if input[i..i + 7] == *"don't()" {
                    do_mul = false;
                    i += 7;
                } else {
                    i += 1;
                }
                continue;
            };
            result += val0 * val1;
            i += 9;
        } else if input[i..i + 4] == *"do()" {
            do_mul = true;
            i += 4;
        } else {
            i += 1;
        }
    }
    Ok(result.to_string())
}
// 63013756

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
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        // 2*4 + 8*5
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
