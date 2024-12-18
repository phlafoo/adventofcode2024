use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = unsafe { inner_part1(input.as_bytes()) };
    Ok(result)
}
// 7,6,1,5,3,1,4,2,6

unsafe fn inner_part1(input: &[u8]) -> String {
    // Parse reg A
    let mut ptr = input.as_ptr().add(12);
    let mut a = (*ptr - b'0') as i32;
    ptr = ptr.add(1);

    while *ptr != b'\n' {
        a = (a * 10) + (*ptr - b'0') as i32;
        ptr = ptr.add(1);
    }

    // Move ptr to first program instruction, skipping B and C reg
    ptr = ptr.add(38);

    // Parse program
    let mut program = vec![];
    while *ptr != b'\n' {
        program.push((*ptr.add(1) - b'0') as i32);
        ptr = ptr.add(2);
    }

    // Run program
    let mut computer = Computer::new(a, program);
    computer.run()
}

struct Computer {
    a: i32,
    b: i32,
    c: i32,
    inst_ptr: usize,
    out: Vec<i32>,
    program: Vec<i32>,
    // Allows instructions to be directly indexed by opcodes
    instruction: [fn(&mut Self, i32); 8],
    // Allows combo numbers to be indexed by their literal counterparts
    combo: [fn(&Self) -> i32; 8],
}

impl Computer {
    fn new(a: i32, program: Vec<i32>) -> Self {
        Computer {
            a,
            b: 0,
            c: 0,
            inst_ptr: 0,
            out: vec![],
            program,
            instruction: [
                Self::adv,
                Self::bxl,
                Self::bst,
                Self::jnz,
                Self::bxc,
                Self::out,
                Self::bdv,
                Self::cdv,

            ],
            combo: [
                Self::combo_0,
                Self::combo_1,
                Self::combo_2,
                Self::combo_3,
                Self::combo_4,
                Self::combo_5,
                Self::combo_6,
                Self::combo_7,
            ]
        }
    }

    unsafe fn run(&mut self) -> String {
        // Run program
        while self.inst_ptr < self.program.len() {
            let opcode = *self.program.get_unchecked(self.inst_ptr);
            let operand = *self.program.get_unchecked(self.inst_ptr + 1);
            self.instruction.get_unchecked(opcode as usize)(self, operand);
        }
        self.out.iter().format(",").to_string()
    }

    #[inline(always)]
    fn get_combo(&self, op: i32) -> i32 {
        self.combo[op as usize](self)
    }

    #[inline(always)]
    fn advance_inst_ptr(&mut self) {
        self.inst_ptr += 2
    }

    #[inline(always)]
    fn divide(&self, op: i32) -> i32 {
        self.a / (1 << self.get_combo(op))
    }

    //  vvv  Instructions in order of opcode  vvv  //

    #[inline(always)]
    fn adv(&mut self, op: i32) {
        self.a = self.divide(op);
        self.advance_inst_ptr();
    }

    #[inline(always)]
    fn bxl(&mut self, op: i32) {
        self.b ^= op;
        self.advance_inst_ptr();
    }

    #[inline(always)]
    fn bst(&mut self, op: i32) {
        // Mod 8 => `& 7`
        self.b = self.get_combo(op) & 7;
        self.advance_inst_ptr();
    }

    #[inline(always)]
    fn jnz(&mut self, op: i32) {
        match self.a {
            0 => self.advance_inst_ptr(),
            _ => self.inst_ptr = op as usize,
        }
    }

    #[inline(always)]
    fn bxc(&mut self, _: i32) {
        self.b ^= self.c;
        self.advance_inst_ptr();
    }

    #[inline(always)]
    fn out(&mut self, op: i32) {
        self.out.push(self.get_combo(op) & 7);
        self.advance_inst_ptr();
    }

    #[inline(always)]
    fn bdv(&mut self, op: i32) {
        self.b = self.divide(op);
        self.advance_inst_ptr();
    }

    #[inline(always)]
    fn cdv(&mut self, op: i32) {
        self.c = self.divide(op);
        self.advance_inst_ptr();
    }
    // Instructions end

    //  vvv  Combo numbers  vvv  //

    #[inline(always)]
    fn combo_0(&self) -> i32 {
        0
    }
    
    #[inline(always)]
    fn combo_1(&self) -> i32 {
        1
    }
    
    #[inline(always)]
    fn combo_2(&self) -> i32 {
        2
    }
    
    #[inline(always)]
    fn combo_3(&self) -> i32 {
        3
    }
    
    #[inline(always)]
    fn combo_4(&self) -> i32 {
        self.a
    }
    
    #[inline(always)]
    fn combo_5(&self) -> i32 {
        self.b
    }
    
    #[inline(always)]
    fn combo_6(&self) -> i32 {
        self.c
    }
    
    #[inline(always)]
    fn combo_7(&self) -> i32 {
        panic!("Combo operand 7 invalid");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", process(input)?);
        Ok(())
    }
}
