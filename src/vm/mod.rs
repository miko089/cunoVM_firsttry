use crate::stack::Stack;

pub enum Instruction {
    HALT(i16),
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    NOT,
    OR,
    AND,
    EQ,
    GT,
    GTE,
    JMP(u16),
    JMPIF(u16),
    POP,
    PUSH(i16),
    STORE(u16),
    LOAD(u16),
    CALL(u16),
    RET,
    RETVAL,
    OUT,
}

pub struct VM {
    frames: Stack<Frame>,
    instructions: Vec<Instruction>,
    halted: bool,
    variables: [i16; 65536],
}
pub struct Frame {
    stack: Stack<i16>,
    ip: u16,
}

impl VM {
    pub fn new(instructions: Vec<Instruction>, start: u16) -> VM {
        let mut frames = Stack::new();
        let main_frame = Frame{
            stack: Stack::new(),
            ip: start
        };
        frames.push(main_frame);
        VM {
            frames,
            instructions,
            halted: false,
            variables: [0; 65536],
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn next_instruction(&mut self) -> Result<(), &str> {
        if self.frames.is_empty() {
            panic!("empty stack without halt")
        }
        let mut frame = self.frames.pop();
        if frame.ip >= self.instructions.len() as u16 {
            panic!("no halt operation")
        }
        match self.instructions[frame.ip as usize] {
            Instruction::HALT(code) => {
                self.halted = true;
                println!("return code is {}", code)
            },
            Instruction::ADD => {
                if (frame.stack.size() < 2) {
                    return Err("not enough arguments for ADD");
                }
                let arg1 = frame.stack.pop();
                let arg2 = frame.stack.pop();
                frame.stack.push(arg1+arg2);
            },
            Instruction::SUB => {
                if (frame.stack.size() < 2) {
                    return Err("not enough arguments for SUB");
                }
                let arg1 = frame.stack.pop();
                let arg2 = frame.stack.pop();
                frame.stack.push(arg1-arg2);
            },
            Instruction::MUL => {
                if (frame.stack.size() < 2) {
                    return Err("not enough arguments for MUL");
                }
                let arg1 = frame.stack.pop();
                let arg2 = frame.stack.pop();
                frame.stack.push(arg1*arg2);
            },
            Instruction::DIV => {
                if (frame.stack.size() < 2) {
                    return Err("not enough arguments for DIV");
                }
                let arg1 = frame.stack.pop();
                let arg2 = frame.stack.pop();
                frame.stack.push(arg1/arg2);
            },
            Instruction::MOD => {
                if (frame.stack.size() < 2) {
                    return Err("not enough arguments for MOD");
                }
                let arg1 = frame.stack.pop();
                let arg2 = frame.stack.pop();
                frame.stack.push(arg1%arg2);
            },
            Instruction::NOT => {
                if (frame.stack.size() < 1) {
                    return Err("not enough arguments for NOT");
                }
                let arg1 = frame.stack.pop();
                frame.stack.push((arg1 == 0) as i16);
            },
            Instruction::OR => {
                if (frame.stack.size() < 2) {
                    return Err("not enough arguments for OR");
                }
                let arg1 = frame.stack.pop();
                let arg2 = frame.stack.pop();
                frame.stack.push(((arg1 != 0) || (arg2 != 0)) as i16);
            },
            Instruction::AND => {
                if (frame.stack.size() < 2) {
                    return Err("not enough arguments for AND");
                }
                let arg1 = frame.stack.pop();
                let arg2 = frame.stack.pop();
                frame.stack.push(((arg1 != 0) && (arg2 != 0)) as i16);
            },
            Instruction::EQ => {
                if (frame.stack.size() < 2) {
                    return Err("not enough arguments for EQ");
                }
                let arg1 = frame.stack.pop();
                let arg2 = frame.stack.pop();
                frame.stack.push(((arg1 == arg2) as i16));
            },
            Instruction::GT => {
                if (frame.stack.size() < 2) {
                    return Err("not enough arguments for GT");
                }
                let arg1 = frame.stack.pop();
                let arg2 = frame.stack.pop();
                frame.stack.push(((arg1 > arg2) as i16));
            },
            Instruction::GTE => {
                if (frame.stack.size() < 2) {
                    return Err("not enough arguments for GTE");
                }
                let arg1 = frame.stack.pop();
                let arg2 = frame.stack.pop();
                frame.stack.push(((arg1 >= arg2) as i16));
            },
            Instruction::JMP(addr) => frame.ip = addr - 1,
            Instruction::JMPIF(addr) => {
                if ((frame.stack.pop()) != 0) {
                    frame.ip = addr - 1
                }
            },
            Instruction::POP => {
                frame.stack.pop();
            },
            Instruction::PUSH(val) => frame.stack.push(val),
            Instruction::STORE(addr) => self.variables[(addr) as usize] = frame.stack.pop(),
            Instruction::LOAD(addr) => frame.stack.push(self.variables[(addr) as usize]),
            Instruction::CALL(addr) => {
                frame.ip += 1;
                self.frames.push(frame);
                self.frames.push(Frame{
                    stack: Stack::new(),
                    ip: addr,
                });
                return Ok(());
            },
            Instruction::RET => {
                return Ok(());
            },
            Instruction::RETVAL => {
                let mut frame2 = self.frames.pop();
                frame2.stack.push(frame.stack.pop());
                self.frames.push(frame2);
                return Ok(());
            },
            Instruction::OUT => {
                println!("{}", frame.stack.pop());
            }
        }
        frame.ip += 1;
        self.frames.push(frame);
        return Ok(());
    }

    pub fn run(&mut self) {
        while !self.is_halted() {
            self.next_instruction().unwrap()
        }
    }
}