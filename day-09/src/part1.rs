pub fn process_naive(input: &str) -> miette::Result<String> {
    let input = input.as_bytes();

    let mut list = vec![];
    let mut file = true;
    let mut id = 0;

    for &b in input {
        if file {
            for _ in 0..parse_u8(b) {
                list.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..parse_u8(b) {
                list.push(None);
            }
        }
        file = !file;
    }
    let mut result = 0;
    let mut right = list.len();

    let count = list.iter().filter(|f| f.is_some()).count();

    let mut left_prev = list[0].unwrap();
    let mut right_prev = list[right - 1].unwrap();

    let mut ret = vec![];

    for (left, i) in (0..count).enumerate() {
        if let Some(id) = list[left] {
            if id != left_prev {
                left_prev = id;
                ret.push(result);
            }
            result += id * i;
        } else {
            loop {
                right -= 1;
                if let Some(id) = list[right] {
                    if id != right_prev {
                        right_prev = id;
                        ret.push(result);
                    }
                    result += id * i;
                    break;
                }
            }
        }
    }

    Ok(result.to_string())
}
// 6353658451014

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input.as_bytes();
    let len = input.len();

    let mut left_id = 0;
    let mut right_id = len / 2;

    let mut left = 0; // Left index
    let mut right = len - 1; // Right index

    let mut result = 0;
    let mut pos = 0;
    let mut remaining = parse_u8(input[right]); // How many ids are remaining from the right side

    let calculate_step = |id, count, pos| -> usize { (id * count * (count + 2 * pos - 1)) / 2 };

    'outer: loop {
        if right <= left {
            // Left and right pointers have met
            // Add last remaining ids
            result += calculate_step(left_id, remaining, pos);
            break;
        }

        // Process file block from the left
        let in_left = parse_u8(input[left]);
        result += calculate_step(left_id, in_left, pos);

        // Process empty block from the left
        pos += in_left;
        left += 1;
        left_id += 1;
        let mut remaining_empty = parse_u8(input[left]);

        // Loop until the empty block is filled with files from the right
        loop {
            if right <= left {
                // Left and right pointers have met
                break 'outer;
            }
            if remaining_empty < remaining {
                let count = remaining_empty;
                remaining -= count;

                result += calculate_step(right_id, count, pos);

                // Empty block is filled, move to next file from the left
                pos += count;
                left += 1;

                break;
            } else {
                let count = remaining;
                remaining_empty -= count;

                right -= 2;
                remaining = parse_u8(input[right]);

                result += calculate_step(right_id, count, pos);
                right_id -= 1;

                // Empty block not yet filled, move the next file from the right
                pos += count;
            };
        }
    }

    Ok(result.to_string())
}
// 6353658451014

#[inline(always)]
fn parse_u8(b: u8) -> usize {
    (b - b'0') as usize
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);

        // for _ in 0..10 {
        //     let input = get_input();
        //     let input = input.as_str();

        //     assert_eq!(process(input)?, process_naive(input)?, "INPUT: {input}");
        // }
        Ok(())
    }

    #[allow(unused)]
    fn get_input() -> String {
        let mut rng = thread_rng();

        let mut input = "".to_string();
        let len = 20_101;

        let mut prev = 0;
        loop {
            let n: u8 = rng.gen_range(0..10);
            if (prev == 0 || input.len() == len - 1) && n == 0 {
                continue;
            }
            prev = n;
            input.push((n + b'0') as char);
            if input.len() == len {
                break;
            }
        }
        println!("input: {}", input);
        input
    }
}
/*
12345

0..111....22222
02.111....2222.
022111....222..
0221112...22...
02211122..2....
022111222......

1121301
0.11.2223
0311.222.
0311222..
3*1 + 2 + 3 + 2*4 + 2*5 + 2*6 = 38

033111..22..333
0331113322..3


2333133121414131402

00...111...2...333.44.5555.6666.777.888899

0 + (18 + 27) + (4 * 8) = 45 + 32 = 77
.. 77 + 5 + 6 + 7 = 95
.. 95 + 8 * 8 + 8 * 9 + 8 * 10 = 95 + 64 + 72 + 80 = 311
.. 311 + (2 * 11) = 333
.. 333 + (7*12) + (7*13) + (7*14) = 606
.. 606 + (3*15) + (3*16) + (3*17) = 750

009..111...2...333.44.5555.6666.777.88889.
0099.111...2...333.44.5555.6666.777.8888..
00998111...2...333.44.5555.6666.777.888...
009981118..2...333.44.5555.6666.777.88....
0099811188.2...333.44.5555.6666.777.8.....
009981118882...333.44.5555.6666.777.......
0099811188827..333.44.5555.6666.77........
00998111888277.333.44.5555.6666.7.........
009981118882777333.44.5555.6666...........
009981118882777333644.5555.666............
00998111888277733364465555.66.............
0099811188827773336446555566..............


*/
