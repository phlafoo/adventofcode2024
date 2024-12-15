
#[derive(Debug, Default)]
pub struct Robot {
    pub px: i32,
    pub py: i32,
    pub vx: i32,
    pub vy: i32,
}

#[inline(always)]
pub unsafe fn parse_robot(input: &[u8], i: &mut usize) -> Robot {
    *i += 2;

    // Get px
    let mut px = (*input.get_unchecked(*i) - b'0') as i32;
    *i += 1;
    let mut b = *input.get_unchecked(*i);
    while b != b',' {
        px = px * 10 + (b - b'0') as i32;
        *i += 1;
        b = *input.get_unchecked(*i);
    }
    *i += 1;

    // Get py
    let mut py = (*input.get_unchecked(*i) - b'0') as i32;
    *i += 1;
    b = *input.get_unchecked(*i);
    while b != b' ' {
        py = py * 10 + (b - b'0') as i32;
        *i += 1;
        b = *input.get_unchecked(*i);
    }
    *i += 3;

    // Get vx
    let sign = if *input.get_unchecked(*i) == b'-' {
        *i += 1;
        -1
    } else {
        1
    };
    let mut vx = (*input.get_unchecked(*i) - b'0') as i32;
    *i += 1;
    b = *input.get_unchecked(*i);
    while b != b',' {
        vx = vx * 10 + (b - b'0') as i32;
        *i += 1;
        b = *input.get_unchecked(*i);
    }
    vx *= sign;
    *i += 1;

    // Get vy
    let sign = if *input.get_unchecked(*i) == b'-' {
        *i += 1;
        -1
    } else {
        1
    };
    let mut vy = (*input.get_unchecked(*i) - b'0') as i32;
    *i += 1;
    b = *input.get_unchecked(*i);
    while b != b'\n' {
        vy = vy * 10 + (b - b'0') as i32;
        *i += 1;
        b = *input.get_unchecked(*i);
    }
    vy *= sign;

    Robot { px, py, vx, vy }
}
