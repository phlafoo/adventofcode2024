use std::ops::Sub;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = unsafe { inner(input.as_bytes()) };
    Ok(result.to_string())
}
// 116821732384052
/*
-2, 1  ->  31420065370
 0, 1  ->  14752615084
 1,-2  ->  27052881364
  */

const DPAD_WIDTH: i64 = 3;
const DPAD_HEIGHT: i64 = 2;
/*
0,0         0
0,1|1,0     2,

18 12 20 18
8  4  8  8
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







A       A           0
<A      v<<A        1
^A      <A>A        2
vA      <vA         3
>A      vA^A        4
<<A     v<<AA>>^A   5
>>A     vAA^A       6
v<<A    <vA<AA>>^A  7
>>^A    vAA<^A>A    8
<^A     v<<A>^A>A   9
>^A     vA<^A>A     10
^>A     <Av>A^A     11
v>A     <vA>A^A     12
<vA     v<<A>A^>A   13
v<A     <vA<A>>^A   14
00 00 00

A 0001
< 0010
^ 0011
v 0100
> 010101010101
// priority: < ^ v >
*/

const A: usize = 0;
const LA: usize = 1;
const UA: usize = 2;
const DA: usize = 3;
const RA: usize = 4;
const LLA: usize = 5;
const RRA: usize = 6;
const DLLA: usize = 7;
const RRUA: usize = 8;
const LUA: usize = 9;
const RUA: usize = 10;
const URA: usize = 11;
const DRA: usize = 12;
const LDA: usize = 13;
const DLA: usize = 14;

const NONE: usize = usize::MAX;
const LUT: [[usize; 4]; 15] = [
    [A, NONE, NONE, NONE],    // A    : A
    [DLLA, RRUA, NONE, NONE], // <A   : v<<A>>^A
    [LA, RA, NONE, NONE],     // ^A   : <A>A
    [LDA, URA, NONE, NONE],   // vA   : <vA^>A
    [DA, UA, NONE, NONE],     // >A   : vA^A
    [DLLA, A, RRUA, NONE],    // <<A  : v<<AA>>^A
    [DA, A, UA, NONE],        // >>A  : vAA^A
    [LDA, LA, A, RRUA],       // v<<A : <vA<AA>>^A
    [DA, A, LUA, RA],         // >>^A : vAA<^A>A
    [DLLA, RUA, RA, NONE],    // <^A  : v<<A>^A>A
    [DA, LUA, RA, NONE],      // >^A  : vA<^A>A
    [LA, DRA, UA, NONE],      // ^>A  : <Av>A^A
    [LDA, RA, UA, NONE],      // v>A  : <vA>A^A
    [DLLA, RA, URA, NONE],    // <vA  : v<<A>A^>A
    [LDA, LA, RRUA, NONE],    // v<A  : <vA<A>>^A
];
const STEP_COUNT: [i64; 15] = [
    1, // A
    2, // <A
    2, // ^A
    2, // vA
    2, // >A
    3, // <<A
    3, // >>A
    4, // v<<A
    4, // >>^A
    3, // <^A
    3, // >^A
    3, // ^>A
    3, // v>A
    3, // <vA
    3, // v<A
];
fn inner(input: &[u8]) -> i64 {
    // const LUT = [

    // ]

    let get_coords = |b: u8| -> (i64, i64) {
        match b {
            b'0' => (1, 0),
            b'A' => (2, 0),
            _ => ((b - b'0' - 1) as i64 % 3, (b - b'0' + 2) as i64 / 3),
        }
    };

    // let (dx, dy) = (-1, 0);
    // let r = rec(dx, dy, 0);
    // println!("{:>2},{:>2}  ->  {r}", dx, dy);
    let mut result = 0;
    let mut memo = [[0; 24]; 15];

    for line in input[..input.len() - 1].split(|&b| b == b'\n') {
        let mut num = 0;
        let (x, y) = get_coords(line[0]);
        let dx = x - 2;
        let dy = y;
        let flip_priority = x == 0;
        // let r = rec_first(dx, dy, flip_priority);

        let p0 = Vec2::new(2, 0);
        let p1 = Vec2::new(x, y);
        let r = rec_first1(p0, p1, &mut memo);
        // println!("{:>2},{:>2}  ->  {r}", dx, dy);

        let mut total = r;

        for p in line.windows(2) {
            num = num * 10 + (p[0] - b'0') as i64;
            let (x0, y0) = get_coords(p[0]);
            let (x1, y1) = get_coords(p[1]);
            let dx = x1 - x0;
            let dy = y1 - y0;
            let flip_priority = (x1 == 0 && y0 == 0) || (x0 == 0 && y1 == 0);

            let p0 = Vec2::new(x0, y0);
            let p1 = Vec2::new(x1, y1);
            let r = rec_first1(p0, p1, &mut memo);
            // let r = rec_first(dx, dy, flip_priority);
            // println!("{:>2},{:>2}  ->  {r}", dx, dy);
            total += r;
        }
        let complexity = total * num;
        // println!("tot: {total}, num: {num}\n");
        result += complexity;
        // break;
    }

    result
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    const ZERO: Vec2 = Vec2 { x: 0, y: 0 };
    fn new(x: i64, y: i64) -> Self {
        Vec2 { x, y }
    }
    fn l1_norm(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn rec1(i: usize, depth: u32, memo: &mut [[i64; 24]; 15]) -> i64 {
    if depth == 24 {
        return STEP_COUNT[i];
    }
    let r = memo[i][depth as usize];
    if r != 0 {
        return r;
    }
    let mut total = 0;
    for s in LUT[i] {
        if s == NONE {
            break;
        }
        total += rec1(s, depth + 1, memo);
        // total += 1;
    }
    memo[i][depth as usize] = total;


    total
}
// 029A
// priority: < ^ v >
fn rec_first1(p0: Vec2, p1: Vec2, memo: &mut [[i64; 24]; 15]) -> i64 {
    if p0 == p1 {
        return 1;
    }
    let d = p1 - p0;

    let flip_priority = (p1.x == 0 && p0.y == 0) || (p0.x == 0 && p1.y == 0);
    // let mut total = d.l1_norm() + 1;
    // let mut total = 0;
    let mut total = d.x.abs().max(1) + d.y.abs().max(1) - 2;
    // let mut memo = [[0; 24]; 15];

    match d.y.cmp(&0) {
        std::cmp::Ordering::Greater => {
            match d.x.cmp(&0) {
                std::cmp::Ordering::Greater => {
                    if flip_priority {
                        total += rec1(DA, 0, memo);
                        total += rec1(LUA, 0, memo);
                        // Return to A
                        total += rec1(RA, 0, memo);
                    } else {
                        total += rec1(LA, 0, memo);
                        total += rec1(DRA, 0, memo);
                        // Return to A
                        total += rec1(UA, 0, memo);
                    }
                }
                std::cmp::Ordering::Less => {
                    if flip_priority {
                        total += rec1(LA, 0, memo);
                        total += rec1(DLA, 0, memo);
                        // Return to A
                        total += rec1(RRUA, 0, memo);
                    } else {
                        total += rec1(DLLA, 0, memo);
                        total += rec1(RUA, 0, memo);
                        // Return to A
                        total += rec1(RA, 0, memo);
                    }
                }
                std::cmp::Ordering::Equal => {
                    total += rec1(LA, 0, memo);
                    // Return to A
                    total += rec1(RA, 0, memo);
                }
            }
        }

        std::cmp::Ordering::Less => {
            match d.x.cmp(&0) {
                std::cmp::Ordering::Greater => {
                    if flip_priority {
                        total += rec1(DA, 0, memo);
                        total += rec1(LA, 0, memo);
                        // Return to A
                        total += rec1(URA, 0, memo);
                    } else {
                        total += rec1(LDA, 0, memo);
                        total += rec1(RA, 0, memo);
                        // Return to A
                        total += rec1(UA, 0, memo);
                    }
                }
                std::cmp::Ordering::Less => {
                    if flip_priority {
                        total += rec1(LDA, 0, memo);
                        total += rec1(LA, 0, memo);
                        // Return to A
                        total += rec1(RRUA, 0, memo);
                    } else {
                        total += rec1(DLLA, 0, memo);
                        total += rec1(RA, 0, memo);
                        // Return to A
                        total += rec1(URA, 0, memo);
                    }
                }
                std::cmp::Ordering::Equal => {
                    total += rec1(LDA, 0, memo);
                    // Return to A
                    total += rec1(URA, 0, memo);
                }
            }
        }
        std::cmp::Ordering::Equal => {
            match d.x.cmp(&0) {
                std::cmp::Ordering::Greater => {
                    total += rec1(DA, 0, memo);
                    // Return to A
                    total += rec1(UA, 0, memo);
                }
                std::cmp::Ordering::Less => {
                    total += rec1(DLLA, 0, memo);
                    // Return to A
                    total += rec1(RRUA, 0, memo);
                }
                std::cmp::Ordering::Equal => {
                    total += rec1(LDA, 0, memo);
                    // Return to A
                    total += rec1(URA, 0, memo);
                }
            }
        }
    }
    total
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
fn rec_first(dx: i64, dy: i64, flip_priority: bool) -> i64 {
    if dx == 0 && dy == 0 {
        return 1;
    }
    // Pressing button repeatedly
    let mut total = dy.abs() + dx.abs();

    // let recl = |dx, dy| -> i64 {
    //     let (xa, ya) = (2, 1);
    //     let (x, y) = (xa)
    // };

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
fn rec(dx: i64, dy: i64, depth: i64) -> i64 {
    if depth == 24 {
        return dx.abs() + dy.abs();
    }
    // let last = 1; //(depth == 1) as i64;
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
