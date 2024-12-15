use crate::common::*;
use std::array;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = unsafe { inner(input.as_bytes()) };
    Ok(result.to_string())
}
// 7037
// mia 7790

pub unsafe fn inner(input: &[u8]) -> i32 {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;

    // COUNT must be <= 500. The higher the better chance of getting the right answer. 50 seems good enough.
    const COUNT: usize = 50;
    let mut robots: [Robot; COUNT] = array::from_fn(|_| Robot::default());

    // Parse robots
    let mut i = 0;
    for r in robots.iter_mut() {
        *r = parse_robot(input, &mut i);
        i += 1;
    }

    // We only check the first 103 steps for y steps and 101 steps for x steps.
    // At each step we approximate the x and y variance separately and save the steps which had
    // the lowest variance. Then we use chinese remainder theorem to solve these equations:
    //    R = sx (mod 101)
    //    R = sy (mod 103)
    // Where R is the total number of steps to get the christmas tree, sx is the step < 101 that
    // had the lowest x-coord variance, and sy is the step < 103 that had the lowest y-coord
    // variance.

    let mut sx = 0;
    let mut sy = 0;
    let mut min_var_x = u32::MAX;
    let mut min_var_y = u32::MAX;

    for s in 1..=HEIGHT as usize {
        let mut i = 0;
        let mut prev_x = robots.get_unchecked(0).px;
        let mut prev_y = robots.get_unchecked(0).py;
        let mut tot_x = 0;
        let mut tot_y = 0;

        loop {
            let r = robots.get_unchecked_mut(i);

            // Take step
            r.px = (r.px + r.vx).rem_euclid(WIDTH);
            r.py = (r.py + r.vy).rem_euclid(HEIGHT);

            // Rough approximation of variance
            tot_x += r.px.abs_diff(prev_x);
            tot_y += r.py.abs_diff(prev_y);
            prev_x = r.px;
            prev_y = r.py;

            if i == COUNT - 1 {
                break;
            }
            i += 1;
        }

        // Update optimal step for x/y if variance was low enough
        if tot_x < min_var_x && s <= WIDTH as usize {
            min_var_x = tot_x;
            sx = s;
        }
        if tot_y < min_var_y {
            min_var_y = tot_y;
            sy = s;
        }
    }
    let sx = sx as i32;
    let sy = sy as i32;

    // We need to get to this equation:
    //   R = (fw * 101) + (fh * 103)
    // Where:
    //   103 * fh = sx (mod 101)
    //   101 * fw = sy (mod 103)
    // =>
    //    2 * fh = sx (mod 101)
    //   -2 * fw = sy (mod 103)
    // Therefore if sx is even, then we have:
    //   fh = sx / 2
    // Otherwise, since width is odd and odd + odd = even, we would have:
    //   fh = (sx + 101) / 2
    // Same applies to fw except with -2 instead of 2.

    let fh = if sx & 1 == 0 {
        (sx) / 2
    } else {
        (sx + WIDTH) / 2
    };

    let fw = if sy & 1 == 0 {
        (sy) / 2
    } else {
        (sy - HEIGHT).abs() / 2
    };

    (fw * WIDTH + fh * HEIGHT).rem_euclid(WIDTH * HEIGHT)
}
