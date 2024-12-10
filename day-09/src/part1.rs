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

    let mut i = 0; // Left index
    let mut j = len - 1; // Right index

    let mut result = 0;
    let mut pos = 0;
    let mut remaining = parse_u8(input[j]); // How many ids are remaining from the right side

    let calculate_step = |id, count, pos| -> usize {
        (id * count * (count + 2 * pos - 1)) / 2
    };

    'outer: loop {
        if j <= i {
            // Left and right pointers have met
            // Add last remaining ids
            result += calculate_step(left_id, remaining, pos);
            break;
        }

        // Process file block from the left
        let in_left = parse_u8(input[i]);
        result += calculate_step(left_id, in_left, pos);

        // Process empty block from the left
        pos += in_left;
        i += 1;
        left_id += 1;
        let mut remaining_empty = parse_u8(input[i]);

        // Loop until the empty block is filled with files from the right
        loop {
            if j <= i {
                // Left and right pointers have met
                break 'outer;
            }
            if remaining_empty < remaining {
                let count = remaining_empty;
                remaining -= count;

                result += calculate_step(right_id, count, pos);

                // Empty block is filled, move to next file from the left
                pos += count;
                i += 1;

                break;
            } else {
                let count = remaining;
                remaining_empty -= count;

                j -= 2;
                remaining = parse_u8(input[j]);

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

fn parse_u8(b: u8) -> usize {
    (b - b'0') as usize
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // let input = "2333133121414131402";
        // let a = process(input)?;
        // let b = process1(input)?;

        // if let Some(e) = a.iter().zip(b.clone()).find(|(a, b)| *a != b) {
        //     panic!("{}, {}", e.0, e.1);
        // }
        // assert_eq!(a, b, "{:?}\n\n{:?}", a, b);

        // assert_eq!("1928", process(input)?);
        // let input = "1121301";

        for _ in 0..10 {
            let input = get_input();
            let input = input.as_str();

            assert_eq!(process(input)?, process_naive(input)?, "INPUT: {input}");
        }

        // let input = "9130516303707930824472047296569685782634907323869243854531871452348163127494304435158484071376708528315121461192641810632762750185821910445209640158493646506120563197694112986690223491389790263030750399905648970961917332387918733117427991813918861583395484440898572277343127329023363732806309434171442152343592048740237053917849822401059368321448690431276394345821822474499775702551718447557341997839133803177680389616039622959676099330179012705711641214954207032765968113344505659230905656769298872696856221072606236591898058985263612203979167131077927208172036583850560936175069157596937426783184510479179530675719930511418692976494368716234438973550539719215319416548773587049683720636619915525366979301131378829295909510334135787117675806983408265686816554084763265070989877788904798613652634907933628092165225202269487981155142518962532771229443939192905711828630795837235853245292763048074746710333379769960852757130715065686342856839454494457556128963418298478620875370328287617630318133118681205893815610648901196585034682916343588623361212063434654487546904907295046544848570333371820288492147817070837550932747929486251180878791035186597395387276849990867221760339714999747922821669207324618642679212476767996518450303223110947552966523273859367724879581195996091048091032741963735455663258415699996478575925102028381061866127996618046023880199244028736494870842724441491783020783145462596328582598833891227922652124636945793690884661095230664029913188310728226298775531949686187206728585157407872126981393049022946920871987769314871863526629815337644907431951737794606969610367537492214067735530474448066051066828705056287721492992426181723652593726773664076748387169771994021440742536165493269110309";
        // left: "3856138847"
        //right: "3856138848"
        // 022111222......
        // 2*1 + 2*2 + 1*3 + 1*4 + 1*5 + 2*6 + 2*7 + 2*8
        // assert_eq!("3856138848", process(input)?);
        Ok(())
    }

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
