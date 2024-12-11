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

    if check_line(test_value, &nums) {
        test_value
    } else {
        0
    }
}

/// Check every permutation
fn check_line(test_value: u64, nums: &[u64]) -> bool {
    let mut b = 0_u32;

    while b < u32::pow(2, nums.len() as u32 - 1) {
        let mut s = nums[0];
        for (i, num) in nums.iter().skip(1).enumerate() {
            if (b >> i & 1) == 0 {
                s += num;
            } else {
                s *= num;
            }
        }
        if s == test_value {
            return true;
        }
        b += 1
    }
    false
}
// 303766880536

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
