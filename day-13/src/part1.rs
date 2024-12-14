#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input.as_bytes();
    let mut result = 0;
    
    let mut i = 0;    
    loop {
        if i == input.len() + 1 {
            break;
        }
        let m = parse_machine(input, &mut i);
        i += 2;

        // This formula only works if vectors A and B are not integer multiples of each other.
        // `a` and `b` must be integers, so we multiply by 10 in the formula, then check that there
        // is no remainder when dividing by 10, which tells us that the value will be an integer.
        let mut b = (m.ax * m.cy - m.ay * m.cx) * 10 / (m.ax * m.by - m.ay * m.bx);

        // `a` and `b` cannot be > 100
        if !(0..=1000).contains(&b) || b % 10 != 0 {
            continue;
        }
        b /= 10;
        let a = (m.cx - b * m.bx) * 10 / m.ax;
        if !(0..=1000).contains(&a) || a % 10 != 0 {
            continue;
        }
        result += (a / 10) * 3 + b;
    }

    Ok(result.to_string())
}
// 35729

#[derive(Debug)]
struct Machine {
    ax: i32,
    ay: i32,
    bx: i32,
    by: i32,
    cx: i32,
    cy: i32,
}

#[inline(always)]
fn parse_machine(input: &[u8], i: &mut usize) -> Machine {
    *i += 12;
    let ax = ((input[*i] - b'0') * 10 + input[*i + 1] - b'0') as i32;
    *i += 6;
    let ay = ((input[*i] - b'0') * 10 + input[*i + 1] - b'0') as i32;
    *i += 15;
    let bx = ((input[*i] - b'0') * 10 + input[*i + 1] - b'0') as i32;
    *i += 6;
    let by = ((input[*i] - b'0') * 10 + input[*i + 1] - b'0') as i32;

    *i += 12;
    let mut cx = 0;
    loop {
        cx = cx * 10 + (input[*i] - b'0') as i32;
        *i += 1;
        if input[*i] == b',' {
            break;
        }
    }
    *i += 4;
    let mut cy = 0;
    loop {
        cy = cy * 10 + (input[*i] - b'0') as i32;
        *i += 1;
        if input[*i] == b'\n' {
            break;
        }
    }
    Machine {
        ax,
        ay,
        bx,
        by,
        cx,
        cy,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
