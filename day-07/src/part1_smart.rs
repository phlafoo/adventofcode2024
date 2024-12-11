// The approach here and in part2_smart are not my own ideas.

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

    if recursive_check(&nums, test_value, nums.len() - 1) {
        test_value
    } else {
        0
    }
}
// 303766880536

#[inline(always)]
fn recursive_check(nums: &[u64], total: u64, i: usize) -> bool {
    let n = nums[i];

    if i == 0 {
        return total == n;
    }

    let divisible = total % n == 0;
    if divisible && recursive_check(nums, total / n, i - 1) {
        return true;
    }

    let subtractable = total > n;
    subtractable && recursive_check(nums, total - n, i - 1)
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
