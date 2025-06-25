use crate::core::bus::Bus;

pub struct Cpu {
    pub v_regs: [u8; 16],
    pub i_reg: u16,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub stack: [u16; 16],
}

impl Cpu {
    pub fn new() -> Self {
        Self { 
            v_regs: [0; 16], 
            i_reg: 0, 
            program_counter: 0, 
            stack_pointer: 0, 
            stack: [0; 16] 
        }
    }
    
    pub fn execute_op(&mut self, op: u16, bus: &mut Bus) {
        let first_bit = (op & 0xF000) >> 12;
        let second_bit = (op & 0x0F00) >> 8;
        let third_bit = (op & 0x00F0) >> 4;
        let fourth_bit = op & 0x000F;

        match (first_bit, second_bit, third_bit, fourth_bit) {
            (0,0,0xE,0) => bus.display.clear(),
            (0,0,0xE,0xE) => {
                self.program_counter = self.stack[self.stack_pointer as usize];
                self.stack_pointer -= 1;
            },
            (0,_,_,_) => return,
            (1,_,_,_) => {
                let nnn = op & 0x0FFF;
                self.program_counter = nnn;
            },
            (2,_,_,_) => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer as usize] = self.program_counter;
                let nnn = op & 0x0FFF;
                self.program_counter = nnn;
            },
            (3,_,_,_) => {
                let vx = self.v_regs[second_bit as usize];
                let kk = (op & 0x00FF) as u8;
                if vx == kk {
                    self.program_counter += 2;
                }
            },
            (4,_,_,_) => {
                let vx = self.v_regs[second_bit as usize];
                let kk = (op & 0x00FF) as u8;
                if vx != kk {
                    self.program_counter += 2;
                }
            },
            (5,_,_,0) => {
                let vx = self.v_regs[second_bit as usize];
                let vy = self.v_regs[third_bit as usize];
                if vx == vy {
                    self.program_counter += 2;
                }
            },
            (6,_,_,_) => {
                let kk = (op & 0x00FF) as u8;
                self.v_regs[second_bit as usize] = kk;
            },
            (7,_,_,_) => {
                let kk = (op & 0x00FF) as u8;
                self.v_regs[second_bit as usize] += kk;
            },
            (8,_,_,0) => {
                let vy = self.v_regs[third_bit as usize];
                self.v_regs[second_bit as usize] = vy;
            },
            (8,_,_,1) => {
                let vy = self.v_regs[third_bit as usize];
                self.v_regs[second_bit as usize] |= vy;
            },
            (8,_,_,2) => {
                let vy = self.v_regs[third_bit as usize];
                self.v_regs[second_bit as usize] &= vy;
            },
            (8,_,_,3) => {
                let vy = self.v_regs[third_bit as usize];
                self.v_regs[second_bit as usize] ^= vy;
            },
            (8,_,_,4) => {
                let vx = self.v_regs[second_bit as usize];
                let vy = self.v_regs[third_bit as usize];
                let (result, overflow) = vx.overflowing_add(vy);
                self.v_regs[second_bit as usize] = result;
                self.v_regs[0xF] = overflow as u8;
            },
            (8,_,_,5) => {
                let vx = self.v_regs[second_bit as usize];
                let vy = self.v_regs[third_bit as usize];
                let (result, overflow) = vx.overflowing_sub(vy);
                self.v_regs[second_bit as usize] = result;
                self.v_regs[0xF] = !overflow as u8;
            },
            (8,_,_,6) => {
                let vx = self.v_regs[second_bit as usize];
                let lsb = vx & 1;
                self.v_regs[second_bit as usize] >>= 1;
                self.v_regs[0xF] = lsb
            },
            (8,_,_,7) => {
                
            },
            (8,_,_,0xE) => {},
            (9,_,_,0) => {},
            (0xA,_,_,_) => {},
            (0xB,_,_,_) => {},
            (0xC,_,_,_) => {},
            (0xD,_,_,_) => {},
            (0xE,_,9,0xE) => {},
            (0xE,_,0xA,1) => {},
            (0xF,_,0,7) => {},
            (0xF,_,0,0xA) => {},
            (0xF,_,1,5) => {},
            (0xF,_,1,8) => {},
            (0xF,_,1,0xE) => {},
            (0xF,_,2,9) => {},
            (0xF,_,3,3) => {},
            (0xF,_,5,5) => {},
            (0xF,_,6,5) => {},
            (_,_,_,_) => panic!("Invalid opcode: {op}"),
        }
            
    }
}
