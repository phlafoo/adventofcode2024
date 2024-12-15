use std::cmp::Ordering;
use crate::common::*;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // const WIDTH: i32 = 11;
    // const HEIGHT: i32 = 7;
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;

    const HALF_WIDTH: i32 = WIDTH / 2;
    const HALF_HEIGHT: i32 = HEIGHT / 2;
    const SECONDS: i32 = 100;
    
    let input = input.as_bytes();
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let mut i = 0;
    loop {
        if i == input.len() {
            break;
        }
        // Parse robot
        let r = unsafe { parse_robot(input, &mut i) };
        i += 1;

        // Find final position
        let dx = r.vx * SECONDS;
        let dy = r.vy * SECONDS;

        let new_x = (r.px + dx).rem_euclid(WIDTH);
        let new_y = (r.py + dy).rem_euclid(HEIGHT);

        // Check quadrant
        match new_x.cmp(&HALF_WIDTH) {
            Ordering::Less => match new_y.cmp(&HALF_HEIGHT) {
                Ordering::Less => q2 += 1,      // top left
                Ordering::Greater => q3 += 1,   // bottom left
                Ordering::Equal => (),
            },
            Ordering::Greater => match new_y.cmp(&HALF_HEIGHT) {
                Ordering::Less => q1 += 1,      // top right
                Ordering::Greater => q4 += 1,   // bottom right
                Ordering::Equal => (),
            },
            Ordering::Equal => (),
        }
    }
    let result = q1 * q2 * q3 * q4;

    Ok(result.to_string())
}
// 218965032
// mia 228421332

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
        assert_eq!("12", process(input)?);
        Ok(())
    }
}