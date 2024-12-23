use std::ops::Sub;

/*
A       A           0
<A      v<<A>>^A    1
^A      <A>A        2
vA      <vA^>A      3
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

// priority: < ^ v >
*/

pub const A: usize = 0;
pub const LA: usize = 1;
pub const UA: usize = 2;
pub const DA: usize = 3;
pub const RA: usize = 4;
// pub const LLA: usize = 5;
// pub const RRA: usize = 6;
pub const DLLA: usize = 7;
pub const RRUA: usize = 8;
pub const LUA: usize = 9;
pub const RUA: usize = 10;
pub const URA: usize = 11;
pub const DRA: usize = 12;
pub const LDA: usize = 13;
pub const DLA: usize = 14;

// Faster than using Option<usize>
pub const NONE: usize = usize::MAX;
pub const LUT: [[usize; 4]; 15] = [
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
pub const STEP_COUNT: [i64; 15] = [
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

/// Returns xy coords on numpad. (0,0) is at bottom left.
pub fn get_coords(b: u8) -> Vec2 {
    match b {
        b'0' => Vec2::new(1, 0),
        b'A' => Vec2::new(2, 0),
        _ => Vec2::new((b - b'0' - 1) as i64 % 3, (b - b'0' + 2) as i64 / 3),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Vec2 {
    pub fn new(x: i64, y: i64) -> Self {
        Vec2 { x, y }
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

pub struct MoveSequence {
    pub first: usize,
    pub second: Option<usize>,
    pub return_to_a: usize,
}

impl MoveSequence {
    fn new(first: usize, second: Option<usize>, return_to_a: usize) -> Self {
        MoveSequence {
            first,
            second,
            return_to_a,
        }
    }
}

pub fn get_move_sequence(d: Vec2, flip_priority: bool) -> MoveSequence {
    use std::cmp::Ordering::*;

    match (d.y.cmp(&0), d.x.cmp(&0), flip_priority) {
        // y > 0, x > 0
        (Greater, Greater, true) => MoveSequence::new(DA, Some(LUA), RA),
        (Greater, Greater, false) => MoveSequence::new(LA, Some(DRA), UA),
        // y > 0, x < 0
        (Greater, Less, true) => MoveSequence::new(LA, Some(DLA), RRUA),
        (Greater, Less, false) => MoveSequence::new(DLLA, Some(RUA), RA),
        // y > 0, x == 0
        (Greater, Equal, _) => MoveSequence::new(LA, None, RA),

        // y < 0, x > 0
        (Less, Greater, true) => MoveSequence::new(DA, Some(LA), URA),
        (Less, Greater, false) => MoveSequence::new(LDA, Some(RA), UA),
        // y < 0, x < 0
        (Less, Less, true) => MoveSequence::new(LDA, Some(LA), RRUA),
        (Less, Less, false) => MoveSequence::new(DLLA, Some(RA), URA),
        // y < 0, x == 0
        (Less, Equal, _) => MoveSequence::new(LDA, None, URA),

        // y = 0
        (Equal, Greater, _) => MoveSequence::new(DA, None, UA),
        (Equal, Less, _) => MoveSequence::new(DLLA, None, RRUA),
        (Equal, Equal, _) => unreachable!("No move sequence for same positions"),
    }
}
