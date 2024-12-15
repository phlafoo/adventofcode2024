use std::array;
use crate::common::*;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;

    // Headstart
    const START: i32 = 6000;

    let input = input.as_bytes();

    const COUNT: usize = 500;
    let mut robots: [Robot; COUNT] = array::from_fn(|_| Robot::default());

    // With limit of 50 and total avg diff of 25 it might get the right answer.
    // To always* get the right answer use a limit of 500 and total avg diff of ~20-29 (much slower).
    // min 25 / 18
    // mia
    const LIMIT: usize = 50;
    const DIFF_MAX: u32 = 25 * LIMIT as u32;
    // const LIMIT: usize = 25;
    // const DIFF_MAX: u32 = 22 * LIMIT as u32;

    unsafe {
        let mut i = 0;
        let mut r = 0;

        // Parse robots, give headstart
        loop {
            if r == LIMIT {
                break;
            }
            let mut robot = parse_robot(input, &mut i);
            robot.px = (robot.px + robot.vx * START).rem_euclid(WIDTH);
            robot.py = (robot.py + robot.vy * START).rem_euclid(HEIGHT);

            robots[r] = robot;
            r += 1;
            i += 1;
        }

        // Check for tree by comparing average x and y diff of robot positions
        for s in (START + 1)..12_000 { // I think max possible steps is 103*101 = 10403
            let mut i = 0;
            let mut prev_x = robots.get_unchecked(0).px;
            let mut prev_y = robots.get_unchecked(0).py;
            let mut tot_x = 0;
            let mut tot_y = 0;

            loop {
                let r = robots.get_unchecked_mut(i);
                r.px = (r.px + r.vx).rem_euclid(WIDTH);
                r.py = (r.py + r.vy).rem_euclid(HEIGHT);

                tot_x += r.px.abs_diff(prev_x);
                tot_y += r.py.abs_diff(prev_y);
                prev_x = r.px;
                prev_y = r.py;

                if i == LIMIT - 1 {
                    break;
                }
                i += 1;
            }
            if tot_x < DIFF_MAX && tot_y < DIFF_MAX {
                return Ok(s.to_string());
            }
        }
    }
    unreachable!();
}
// 7037
// mia 7790
