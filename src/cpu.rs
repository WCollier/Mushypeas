use crate::{fonts, instr::Instr, opcode::Opcode, Result};
use std::convert::TryInto;
use std::time::Duration;

pub(crate) const SCREEN_WIDTH: usize = 64;

pub(crate) const SCREEN_HEIGHT: usize = 32;

pub(crate) const NUM_KEYS: usize = 16;

const MEM_SIZE: usize = 4096;

const INSTR_START: usize = 0x200;

const INSTR_SIZE: usize = 2;

const REGS: usize = 16;

const STACK_SIZE: usize = 16;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Cpu {
    pub(crate) display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub(crate) keys: [bool; NUM_KEYS],
    pub(crate) should_rerender: bool,
    stack: [usize; STACK_SIZE],
    registers: [u8; REGS],
    memory: [u8; MEM_SIZE],
    ticks: u128,
    index: usize,
    pc: usize,
    sp: usize,
    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub(crate) fn new() -> Self {
        Cpu {
            display: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            keys: [false; NUM_KEYS],
            should_rerender: false,
            stack: [0; STACK_SIZE],
            registers: [0; REGS],
            memory: [0; MEM_SIZE],
            ticks: 0,
            index: 0,
            pc: INSTR_START,
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub(crate) fn load(&mut self, instrs: &[u8]) {
        self.memory[0..fonts::FONTS.len()].clone_from_slice(fonts::FONTS);

        for (i, instr) in instrs.iter().enumerate() {
            self.memory[INSTR_START + i] = *instr;
        }

        //self.memory[INSTR_START..INSTR_START + instrs.len()].clone_from_slice(instrs as &[usize]);
    }

    pub(crate) fn run(&mut self) {
        if self.pc > MEM_SIZE - 1 {
            return;
        }

        self.ticks += 1;

        // Attempt to decrement the timers 60 times per second (at 500Hz)
        // TODO: Add configurable running rate
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;

            // TODO: Add proper beep sound
            println!("Beep");
        }

        match Cpu::decode_instr(self.memory[self.pc], self.memory[self.pc + 1]) {
            Ok(ref instr) => self.eval(instr),
            Err(e) => println!("{:?}, pc: {}", e, self.pc),
        }
    }

    fn decode_instr(left: u8, right: u8) -> Result<Instr> {
        let instr = (left as u16) << 8 | (right) as u16;

        let opcode = Opcode::new(instr);

        opcode.try_into()
    }

    fn eval(&mut self, instr: &Instr) {
        //println!("Evaluating: {:?}, pc: {}", instr, self.pc);

        match *instr {
            // Most interpreters ignore this
            Instr::JumpToMachineCode { addr: _ } => {
                self.end_instr();
            }
            Instr::Clear => {
                for pixel in self.display.iter_mut() {
                    *pixel = false;
                }

                self.should_rerender = true;

                self.end_instr();
            }
            Instr::Return => {
                self.sp -= 1;

                self.pc = self.stack[self.sp];
            }
            Instr::Jump { addr } => self.pc = addr,
            Instr::Call { addr } => {
                // After the call, skip the instruction after the present one
                self.stack[self.sp] = self.pc + INSTR_SIZE;

                self.sp += 1;

                self.pc = addr;
            }
            Instr::SkipNextEqualLiteral { reg, lit } => {
                if self.registers[reg] == lit {
                    self.skip_instr();
                } else {
                    self.end_instr();
                }
            }
            Instr::SkipNextNotEqualLiteral { reg, lit } => {
                if self.registers[reg] != lit {
                    self.skip_instr();
                } else {
                    self.end_instr();
                }
            }
            Instr::SkipNextEqualRegister { left, right } => {
                if self.registers[left] == self.registers[right] {
                    self.skip_instr();
                } else {
                    self.end_instr();
                }
            }
            Instr::RegisterSetLiteral { reg, lit } => {
                self.registers[reg] = lit;

                self.end_instr();
            }
            Instr::RegisterAddAssign { reg, lit } => {
                let reg_val = self.registers[reg] as u16;

                let lit = lit as u16;

                self.registers[reg] = (reg_val + lit) as u8;

                self.end_instr();
            }
            Instr::RegisterSetRegister { left, right } => {
                self.registers[left] = self.registers[right];

                self.end_instr();
            }
            Instr::RegisterSetRegisterBitwiseOr { left, right } => {
                self.registers[left] |= self.registers[right];

                self.end_instr();
            }
            Instr::RegisterSetRegisterBitwiseAnd { left, right } => {
                self.registers[left] &= self.registers[right];

                self.end_instr();
            }
            Instr::RegisterSetRegisterBitwiseXor { left, right } => {
                self.registers[left] ^= self.registers[right];

                self.end_instr();
            }
            Instr::RegisterSetRegisterAdd { left, right } => {
                let left_val = self.registers[left];

                let right_val = self.registers[right];

                let (left_res, overflowed) = left_val.overflowing_add(right_val);

                self.registers[left] = left_res;

                self.registers[0xF] = u8::from(overflowed);

                self.end_instr();
            }
            Instr::RegisterSetRegisterSub { left, right } => {
                let left_val = self.registers[left];

                let right_val = self.registers[right];

                let (left_res, overflowed) = left_val.overflowing_sub(right_val);

                self.registers[left] = left_res;

                self.registers[0xF] = u8::from(!overflowed);

                self.end_instr();
            }
            Instr::RegisterSetRegisterShr { left, right: _ } => {
                self.registers[0xF] = self.registers[left] & 0x1;

                self.registers[left] >>= 1;

                self.end_instr();
            }
            Instr::RegisterSetRegisterSubn { left, right } => {
                let left_val = self.registers[left];

                let right_val = self.registers[right];

                let (res, overflowed) = right_val.overflowing_sub(left_val);

                self.registers[left] = res;

                self.registers[0xF] = u8::from(!overflowed);

                self.end_instr();
            }
            Instr::RegisterSetRegisterShl { reg } => {
                self.registers[0xF] = self.registers[reg] >> 7;

                self.registers[reg] <<= 1;

                self.end_instr();
            }
            Instr::SkipNextNotEqualRegister { left, right } => {
                if self.registers[left] != self.registers[right] {
                    self.skip_instr();
                } else {
                    self.end_instr();
                }
            }
            Instr::SetIndex { value } => {
                self.index = value;

                self.end_instr();
            }
            Instr::JumpTo { addr } => self.pc = (self.registers[0] as usize) + addr,
            Instr::RandBitwiseAnd { reg, lit } => {
                self.registers[reg] = rand::random::<u8>() & lit;

                self.end_instr();
            }
            Instr::DrawSprite { x, y, size } => {
                let x = self.registers[x] as usize;

                let y = self.registers[y] as usize;

                self.draw_sprite(x, y, size as usize);

                self.end_instr();
            }
            Instr::SkipNextKeyPressed { reg } => {
                if self.keys[self.registers[reg] as usize] {
                    self.skip_instr();
                } else {
                    self.end_instr();
                }
            }
            Instr::SkipNextKeyNotPressed { reg } => {
                if !self.keys[self.registers[reg] as usize] {
                    self.skip_instr();
                } else {
                    self.end_instr();
                }
            }
            Instr::SetDelayTimerValue { reg } => {
                self.registers[reg] = self.delay_timer;

                self.end_instr();
            }
            Instr::KeyPressWait { reg } => {
                // TODO: Try and use an iterator here
                for (i, key) in self.keys.iter().enumerate() {
                    if *key {
                        self.registers[reg] = i as u8;

                        self.end_instr();

                        break;
                    }
                }
            }
            Instr::SetDelayTimerRegister { reg } => {
                self.delay_timer = self.registers[reg];

                self.end_instr();
            }
            Instr::SetSoundTimerRegister { reg } => {
                self.sound_timer = self.registers[reg];

                self.end_instr();
            }
            Instr::IndexAddAssignRegister { reg } => {
                self.index += self.registers[reg] as usize;

                self.end_instr();
            }
            Instr::SetIndexToDigitSprite { reg } => {
                // * 5 because each sprite is 5 bytes long
                self.index = (self.registers[reg] as usize) * 5;

                self.end_instr();
            }
            Instr::StoreBCDAtIndex { reg } => {
                let reg = self.registers[reg];

                let index = self.index;

                // Hundreds
                self.memory[index] = reg / 100;

                // Tens
                self.memory[index + 1] = (reg % 100) / 10;

                // Digits
                self.memory[index + 2] = reg % 10;

                self.end_instr();
            }
            Instr::StoreRegistersAtIndex { start_addr } => {
                let regs = &self.registers[0..(start_addr + 1)];

                let reg_end_addr = self.index + start_addr + 1;

                self.memory[(self.index)..reg_end_addr].copy_from_slice(regs);

                self.end_instr();
            }
            Instr::ReadRegistersAtIndex { start_addr } => {
                let reg_end_addr = (self.index + start_addr) + 1;

                let memory_values = &self.memory[(self.index)..reg_end_addr];

                self.registers[0..(start_addr) + 1].clone_from_slice(memory_values);

                self.end_instr();
            }
        }
    }

    fn draw_sprite(&mut self, x: usize, y: usize, size: usize) {
        self.registers[0xF] = 0;

        for i in 0..size {
            let y = (y + i) % SCREEN_HEIGHT;

            for bit in 0..8 {
                let x = (x + bit) % SCREEN_WIDTH;

                let index = x + y * SCREEN_WIDTH;

                let colour = (self.memory[self.index + i]) >> (7 - bit) & 1;

                self.registers[0xF] |= colour & (self.display[index]) as u8;

                self.display[index] ^= colour == 1;
            }
        }

        self.should_rerender = true;
    }

    fn end_instr(&mut self) {
        self.pc += INSTR_SIZE;
    }

    fn skip_instr(&mut self) {
        self.pc += INSTR_SIZE * 2;
    }
}
