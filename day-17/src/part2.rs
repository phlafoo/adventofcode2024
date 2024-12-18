#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = unsafe { inner_part2(input.as_bytes()) };
    Ok(result.to_string())
}
// 164541017976509

unsafe fn inner_part2(input: &[u8]) -> u64 {
    // Skip past A register
    let mut ptr = input.as_ptr().add(12);
    while *ptr != b'\n' {
        ptr = ptr.add(1);
    }

    // Move ptr to first program instruction, skipping B and C reg
    ptr = ptr.add(38);

    // Parse program
    let mut program = vec![];
    while *ptr != b'\n' {
        program.push((*ptr.add(1) - b'0') as u64);
        ptr = ptr.add(2);
    }

    // (a, index in program to check output against)
    let mut stack = [(0, 0); 32];
    stack[0] = (0, program.len() - 1);
    let mut stack_len = 1;

    let mut computer = Computer::new(0, program);

    while stack_len > 0 {
        let (s, i) = *stack.get_unchecked(stack_len - 1);
        stack_len -= 1;
        match i {
            0 => for a in s..s + 8 {
                // If we are checking the last number then we must go in increasing order
                if computer.check(a, i) {
                    return a;
                }
            }
            _ => for a in (s..s + 8).rev() {
                // Go in reverse because we are adding to the stack
                if computer.check(a, i) {
                    *stack.get_unchecked_mut(stack_len) = (a << 3, i - 1);
                    stack_len += 1;
                }
            }
        }
    }
    panic!("Answer not found");
}

struct Computer {
    a: u64,
    b: u64,
    c: u64,
    inst_ptr: usize,
    /// For part 2 it's sufficient to output a single number at a time
    out: u64,
    program: Vec<u64>,
    // Allows instructions to be directly indexed by opcodes
    instruction: [fn(&mut Self, u64); 8],
    // Allows combo numbers to be indexed by their literal counterparts
    combo: [fn(&Self) -> u64; 8],
}

impl Computer {
    fn new(a: u64, program: Vec<u64>) -> Self {
        Computer {
            a,
            b: 0,
            c: 0,
            inst_ptr: 0,
            out: 0,
            program: program.to_vec(),
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
            ],
        }
    }

    #[inline(always)]
    unsafe fn check(&mut self, a: u64, i: usize) -> bool {
        // Reset
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.inst_ptr = 0;
        
        // Run program
        while self.inst_ptr < self.program.len() {
            let opcode = *self.program.get_unchecked(self.inst_ptr);
            let operand = *self.program.get_unchecked(self.inst_ptr + 1);
            self.instruction.get_unchecked(opcode as usize)(self, operand);
        }
        // We can stop as soon as we output our first number
        self.out == self.program[i]
    }

    #[inline(always)]
    fn get_combo(&self, op: u64) -> u64 {
        self.combo[op as usize](self)
    }

    #[inline(always)]
    fn advance_inst_ptr(&mut self) {
        self.inst_ptr += 2
    }

    #[inline(always)]
    fn divide(&self, op: u64) -> u64 {
        self.a / (1 << self.get_combo(op))
    }

    //  vvv  Instructions in order of opcode  vvv  //

    #[inline(always)]
    fn adv(&mut self, op: u64) {
        self.a = self.divide(op);
        self.advance_inst_ptr();
    }

    #[inline(always)]
    fn bxl(&mut self, op: u64) {
        self.b ^= op;
        self.advance_inst_ptr();
    }

    #[inline(always)]
    fn bst(&mut self, op: u64) {
        // Mod 8 => `& 7`
        self.b = self.get_combo(op) & 7;
        self.advance_inst_ptr();
    }

    #[inline(always)]
    fn jnz(&mut self, op: u64) {
        match self.a {
            0 => self.advance_inst_ptr(),
            _ => self.inst_ptr = op as usize,
        }
    }

    #[inline(always)]
    fn bxc(&mut self, _: u64) {
        self.b ^= self.c;
        self.advance_inst_ptr();
    }

    #[inline(always)]
    fn out(&mut self, op: u64) {
        // Immediately ends program execution
        self.out = self.get_combo(op) & 7;
        self.inst_ptr = self.program.len();
    }

    #[inline(always)]
    fn bdv(&mut self, op: u64) {
        self.b = self.divide(op);
        self.advance_inst_ptr();
    }

    #[inline(always)]
    fn cdv(&mut self, op: u64) {
        self.c = self.divide(op);
        self.advance_inst_ptr();
    }
    // Instructions end

    //  vvv  Combo numbers  vvv  //

    #[inline(always)]
    fn combo_0(&self) -> u64 {
        0
    }

    #[inline(always)]
    fn combo_1(&self) -> u64 {
        1
    }

    #[inline(always)]
    fn combo_2(&self) -> u64 {
        2
    }

    #[inline(always)]
    fn combo_3(&self) -> u64 {
        3
    }

    #[inline(always)]
    fn combo_4(&self) -> u64 {
        self.a
    }

    #[inline(always)]
    fn combo_5(&self) -> u64 {
        self.b
    }

    #[inline(always)]
    fn combo_6(&self) -> u64 {
        self.c
    }

    #[inline(always)]
    fn combo_7(&self) -> u64 {
        unreachable!("Combo operand 7 invalid");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";
        assert_eq!("117440", process(input)?);
        Ok(())
    }
}
