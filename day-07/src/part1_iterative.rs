#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = input.lines().fold(0, |acc, line| acc + process_line(line));

    Ok(result.to_string())
}

fn process_line(line: &str) -> u64 {
    let c = line.find(':').unwrap();

    let test_value = line[..c].parse::<u64>().unwrap();
    let nums = line[c + 1..]
        .split_ascii_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    if check(test_value, &nums) {
        test_value
    } else {
        0
    }
}
// 303766880536

#[inline(always)]
fn check(test_value: u64, nums: &[u64]) -> bool {
    let mut stack = [(0_usize, 0_u64); 32];
    stack[0] = (1_usize, nums[0]);
    let mut stack_len = 1;

    while stack_len > 0 {
        let (i, sum) = stack[stack_len - 1];
        if i == nums.len() {
            if sum == test_value {
                return true;
            }
            stack_len -= 1;
            continue;
        }
        stack[stack_len - 1] = (i + 1, sum + nums[i]);
        stack[stack_len] = (i + 1, sum * nums[i]);
        stack_len += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
