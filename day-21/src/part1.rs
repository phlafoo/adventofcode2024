#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = unsafe { inner(input.as_bytes()) };
    Ok(result.to_string())
}
// 94284

const DPAD_WIDTH: i32 = 3;
const DPAD_HEIGHT: i32 = 2;
/*
0,0         0
0,1|1,0     2,

<vA<AA>>^AvAA<^A>A <v<A>>^AvA^A <vA>^A<v<A>^A>AAvA^A <v<A>A>^AAAvA<^A>A
v<<A>>^A           <A>A         vA<^AA>A             <vAAA>^A
<A                 ^A           >^^A                 vvvA
029A

379A :::

<v<A >>^A vA ^A <vA <AA >>^AA vA <^A >AA vA ^A <vA >^AA <A >A <v<A >A >^AAA vA <^A >A
   <    A  >  A   v  <<    AA  >   ^  AA  >  A   v   AA  ^  A    <  v   AAA  >   ^  A

.
<v<A>>^AvA^A <vA<AA>>^AA vA<^A>AA vA^A              <vA>^AA<A>A<v<A>A>^AAAvA<^A>A
<A>A         v<<AA       >^AA     >A                vAA^A<vAAA>^A
             <<^^A

<v<A>>^AvA^A v<<A>>^AA v<A<A>>^AA vAA<^A>A          v<A>^AA <A>A v<A<A>>^AAA vA<^A>A
<A>A         <AA       v<AA       >>^A              vAA     ^A   v<AAA       >^A
^A           ^^<<A                                  >>A     vvvA
379A

<v<A>>^AA<vA<A>>^AAvAA<^A>A <vA>^A<A>A <vA>^A<A>A <v<A>A>^AAvA<^A>A
<AA

<<vAA>A>^AAvA<^A>AAvA^A     <vA>^A<A>A <vA>^A<A>A <<vA>A>^AAvA<^A>A
<<vAA>^AA>A                 vA^A       vA^A       <vAA>^A
<<^^A                       >A         >A         vvA
456A ::::


<<vA>A>^AAvA^A<A>A
<vAA>A^A
vv>A

v<<A>>^A<vA<A>>^AAvAA<^A>A v<<A>>^AvA^A <vA>^Av<<A>>^AAvA<^A>A <vA>^A<A>A
<Av<AA>>^A                 <A>A         vA<AA>^A               vA^A
^<<A                       ^A           >vvA                   >A
140A
26 12 22

v<<A>>^A<vA<A>>^AAvAA<^A>A <vA>^AA<<vA>^A>AvA^A
<Av<AA>>^A                 vAA<^A>A
^<<A                       >>^A
169A

v<<A>>^A<vA<A>>^AAvAA<^A>A
<Av<AA>>^A
^<<A                       ^^A >vvA>A
170A


<<vA>>^AAv<<A>A>^AvAA<^A>A
<AA<vA>>^A
^^<A


<<vA>A>^AAAvA^A<A>A
<vAAA>A^A
vvv>A

<vA>^A<<vA>>^AAAvA<^A>A
vA<AAA>^A
>vvvA

<<vAA>A>^AvA<^A>AAvA^A
<<vA>^AA>A
<^^A       vA ^^A         >vvvA
528A

340A

// priority: < ^ v >
*/
fn inner(input: &[u8]) -> i32 {
    // const LUT = [

    // ]

    let get_coords = |b: u8| -> (i32, i32) {
        match b {
            b'0' => (1, 0),
            b'A' => (2, 0),
            _ => ((b - b'0' - 1) as i32 % 3, (b - b'0' + 2) as i32 / 3),
        }
    };

    // let (dx, dy) = (-1, 0);
    // let r = rec(dx, dy, 0);
    // println!("{:>2},{:>2}  ->  {r}", dx, dy);
    let mut result = 0;
    for line in input[..input.len() - 1].split(|&b| b == b'\n') {
        let mut num = 0;
        let (x, y) = get_coords(line[0]);
        let dx = x - 2;
        let dy = y;
        let flip_priority = x == 0;
        let r = rec_first(dx, dy, flip_priority);
        println!("{:>2},{:>2}  ->  {r}", dx, dy);

        let mut total = r;

        for p in line.windows(2) {
            num = num * 10 + (p[0] - b'0') as i32;
            let (x0, y0) = get_coords(p[0]);
            let (x1, y1) = get_coords(p[1]);
            let dx = x1 - x0;
            let dy = y1 - y0;
            let flip_priority = (x1 == 0 && y0 == 0) || (x0 == 0 && y1 == 0);

            let r = rec_first(dx, dy, flip_priority);
            println!("{:>2},{:>2}  ->  {r}", dx, dy);
            total += r;
        }
        let complexity = total * num;
        println!("tot: {total}, num: {num}\n");
        result += complexity;
        // break;
    }

    // for dx in -2..3 {
    //     for dy in -3..4 {
    //         let r = rec(dx, dy, 0);
    //         println!("{:>2},{:>2}  ->  {r}", dx, dy);
    //     }
    // }

    // for x in 0..3 {
    //     for y in 0..4 {
    //         if x == 0 && y == 0 {
    //             continue;
    //         }
    //         for tx in 0..3 {
    //             for ty in 0..4 {
    //                 if tx == 0 && ty == 0 {
    //                     continue;
    //                 }
    //                 let dx = tx - x;
    //                 let dy = ty - y;

    //                 let r = rec(dx, dy, 0);
    //                 println!("{:>2},{:>2}  ->  {r}", dx, dy);
    //             }
    //         }
    //     }
    // }

    result
}
/*

<vA<AA>>^AvAA<^A>A

0,2 = 5
<AA>A
0,3 = 6
<AAA>A

-1, 0
v<<A>>^A
-2,-1  2,1



*/
// priority: < ^ v >
#[allow(clippy::comparison_chain)]
fn rec_first(dx: i32, dy: i32, flip_priority: bool) -> i32 {
    if dx == 0 && dy == 0 {
        return 1;
    }
    // Pressing button repeatedly
    let mut total = dy.abs() + dx.abs();

    if dy > 0 {
        if dx > 0 {
            if flip_priority {
                total += rec(0, -1, 0);
                total += rec(-1, 1, 0);
                // Return to A
                total += rec(1, 0, 0);
            } else {
                total += rec(-1, 0, 0);
                total += rec(1, -1, 0);
                // Return to A
                total += rec(0, 1, 0);
            }
        } else if dx < 0 {
            if flip_priority {
                total += rec(-1, 0, 0);
                total += rec(-1, -1, 0);
                // Return to A
                total += rec(2, 1, 0);
            } else {
                total += rec(-2, -1, 0);
                total += rec(1, 1, 0);
                // Return to A
                total += rec(1, 0, 0);
            }
        } else {
            total += rec(-1, 0, 0);
            // Return to A
            total += rec(1, 0, 0);
        }
    } else if dy < 0 {
        if dx > 0 {
            if flip_priority {
                total += rec(0, -1, 0);
                total += rec(-1, 0, 0);
                // Return to A
                total += rec(1, 1, 0);
            } else {
                total += rec(-1, -1, 0);
                total += rec(1, 0, 0);
                // Return to A
                total += rec(0, 1, 0);
            }
        } else if dx < 0 {
            if flip_priority {
                total += rec(-1, -1, 0);
                total += rec(-1, 0, 0);
                // Return to A
                total += rec(2, 1, 0);
            } else {
                total += rec(-2, -1, 0);
                total += rec(1, 0, 0);
                // Return to A
                total += rec(1, 1, 0);
            }
        } else {
            total += rec(-1, -1, 0);
            // Return to A
            total += rec(1, 1, 0);
        }
    } else if dx > 0 {
        total += rec(0, -1, 0);
        // Return to A
        total += rec(0, 1, 0);
    } else if dx < 0 {
        total += rec(-2, -1, 0);
        // total += rec(0, -1, depth + 1, false);
        // total += rec(-2, 0, depth + 1, false);

        // Return to A
        total += rec(2, 1, 0);
    }
    total + 1
}

#[allow(clippy::comparison_chain)]
fn rec(dx: i32, dy: i32, depth: i32) -> i32 {
    if depth == 1 {
        return dx.abs() + dy.abs();
    }
    // let last = 1; //(depth == 1) as i32;
    if dx == 0 && dy == 0 {
        return 1;
    }
    // Pressing button repeatedly
    let mut total = dy.abs() + dx.abs();

    if dy > 0 {
        if dx > 0 {
            total += rec(-1, 0, depth + 1);
            total += rec(1, -1, depth + 1);
            // Return to A
            total += rec(0, 1, depth + 1);
        } else if dx < 0 {
            total += rec(-2, -1, depth + 1);
            total += rec(1, 1, depth + 1);
            // Return to A
            total += rec(1, 0, depth + 1);
        } else {
            total += rec(-1, 0, depth + 1);
            // Return to A
            total += rec(1, 0, depth + 1);
        }
    } else if dy < 0 {
        if dx > 0 {
            total += rec(-1, -1, depth + 1);
            total += rec(1, 0, depth + 1);
            // Return to A
            total += rec(0, 1, depth + 1);
        } else if dx < 0 {
            total += rec(-2, -1, depth + 1);
            total += rec(1, 0, depth + 1);
            // Return to A
            total += rec(1, 1, depth + 1);
        } else {
            total += rec(-1, -1, depth + 1);
            // Return to A
            total += rec(1, 1, depth + 1);
        }
    } else if dx > 0 {
        total += rec(0, -1, depth + 1);
        // Return to A
        total += rec(0, 1, depth + 1);
    } else if dx < 0 {
        total += rec(-2, -1, depth + 1);
        // total += rec(0, -1, depth + 1);
        // total += rec(-2, 0, depth + 1);

        // Return to A
        total += rec(2, 1, depth + 1);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    // 379A
    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
029A
980A
179A
456A
379A
";
        //         let input = "\
        // 456A
        // ";
        assert_eq!("126384", process(input)?);
        Ok(())
    }
}
