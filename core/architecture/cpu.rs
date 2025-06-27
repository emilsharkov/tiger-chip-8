use crate::core::{bus::Bus, ram::PROGRAM_START, vram::{DISPLAY_HEIGHT, DISPLAY_WIDTH}};

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
            program_counter: PROGRAM_START as u16, 
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
            (0,0,0xE,0) => bus.vram.clear(),
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
                let vx = self.v_regs[second_bit as usize];
                let vy = self.v_regs[third_bit as usize];
                let (result, overflow) = vy.overflowing_sub(vx);
                self.v_regs[second_bit as usize] = result;
                self.v_regs[0xF] = !overflow as u8;
            },
            (8,_,_,0xE) => {
                let vx = self.v_regs[second_bit as usize];
                let msb = vx & 0x80;
                self.v_regs[second_bit as usize] <<= 1;
                self.v_regs[0xF] = msb;
            },
            (9,_,_,0) => {
                let vx = self.v_regs[second_bit as usize];
                let vy = self.v_regs[third_bit as usize];
                if vx != vy {
                    self.program_counter += 2;
                }
            },
            (0xA,_,_,_) => {
                let nnn = op & 0x0FFF;
                self.i_reg = nnn;
            },
            (0xB,_,_,_) => {
                let nnn = op & 0x0FFF;
                let v0 = self.v_regs[0] as u16;
                self.program_counter = nnn + v0;
            },
            (0xC,_,_,_) => {
                let rand = rand::random::<u8>();
                let kk = (op & 0x00FF) as u8;
                self.v_regs[second_bit as usize] = rand & kk;
            },
            (0xD,_,_,_) => {
                let vx = self.v_regs[second_bit as usize] as usize;
                let vy = self.v_regs[third_bit as usize] as usize;
                let num_bytes = fourth_bit as usize;
                let mut collision = false;

                for byte_idx in 0..num_bytes {
                    let address = self.i_reg as usize + byte_idx;
                    let mem_pixels = bus.ram.read(address);
                    for bit_idx in 0..u8::BITS as usize {
                        let mem_pixel = mem_pixels & (0x80 >> bit_idx);
                        let new_x = (vx + bit_idx) % DISPLAY_WIDTH;
                        let new_y = (vy + byte_idx) % DISPLAY_HEIGHT;
                        let display_index = new_y * DISPLAY_WIDTH + new_x;
                        let display_pixel = bus.vram.get_pixel(display_index);

                        if mem_pixel != 0 {
                            if display_pixel {
                                collision = true;
                            }
                            bus.vram.update_pixel(display_index, !display_pixel);
                        }
                    }
                }

                self.v_regs[0xF] = collision as u8;
            },
            (0xE,_,9,0xE) => {
                let vx = self.v_regs[second_bit as usize];
                if bus.keypad.keys[vx as usize] == true {
                    self.program_counter += 2;
                }
            },
            (0xE,_,0xA,1) => {
                let vx = self.v_regs[second_bit as usize];
                if bus.keypad.keys[vx as usize] == false {
                    self.program_counter += 2;
                }
            },
            (0xF,_,0,7) => {
                self.v_regs[second_bit as usize] = bus.timers.delay_timer
            },
            (0xF,_,0,0xA) => {
                let pressed_index = bus.keypad.keys.iter().position(|key| *key == true);
                match pressed_index {
                    Some(key) => {
                        self.v_regs[second_bit as usize] = key as u8;
                    },
                    None => {
                        self.program_counter -= 2;
                    }
                }
            },
            (0xF,_,1,5) => {
                let vx = self.v_regs[second_bit as usize];
                bus.timers.delay_timer = vx;
            },
            (0xF,_,1,8) => {
                let vx = self.v_regs[second_bit as usize];
                bus.timers.sound_timer = vx;
            },
            (0xF,_,1,0xE) => {
                let vx = self.v_regs[second_bit as usize] as u16;
                self.i_reg += vx;
            },
            (0xF,_,2,9) => {
                let vx = self.v_regs[second_bit as usize] as u16;
                self.i_reg = vx * 5;
            },
            (0xF,_,3,3) => {
                let vx = self.v_regs[second_bit as usize];
                let hundreds = (vx / 100) as u8;
                let tens = ((vx / 10) % 10) as u8;
                let ones = (vx % 10) as u8;
                let address = self.i_reg as usize;
                bus.ram.write(address, hundreds);
                bus.ram.write(address + 1, tens);
                bus.ram.write(address + 2, ones);
            },
            (0xF,_,5,5) => {
                for idx in 0..second_bit as usize {
                    let vi = self.v_regs[idx as usize];
                    let address = self.i_reg as usize + idx as usize;
                    bus.ram.write(address, vi);
                }
            },
            (0xF,_,6,5) => {
                for idx in 0..second_bit as usize {
                    let address = self.i_reg as usize + idx as usize;
                    self.v_regs[idx as usize] = bus.ram.read(address);
                }
            },
            (_,_,_,_) => panic!("Invalid opcode: {op}"),
        }
            
    }
}
