use crate::{instr::Instr, EmulatorError};
use std::convert::TryInto;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Opcode {
    pub(crate) opcode: u16,
    raw: u16,
    nnn: usize,
    x: usize,
    y: usize,
    n: u8,
    kk: u8,
}

impl Opcode {
    pub(crate) fn new(opcode: u16) -> Self {
        Opcode {
            opcode: ((opcode & 0xF000) >> 12),
            raw: opcode,
            nnn: (opcode & 0x0FFF) as usize,
            x: ((opcode & 0x0F00) >> 8) as usize,
            y: ((opcode & 0x00F0) >> 4) as usize,
            n: (opcode & 0x000F) as u8,
            kk: (opcode & 0x00FF) as u8,
        }
    }
}

impl TryInto<Instr> for Opcode {
    type Error = EmulatorError;

    fn try_into(self) -> Result<Instr, Self::Error> {
        match self.opcode {
            0x0 => match self.kk {
                0xE0 => Ok(Instr::Clear),
                0xEE => Ok(Instr::Return),

                // This isn't used, but is is necessary (should this be the default case?)
                _ => Ok(Instr::JumpToMachineCode { addr: self.nnn }),
            },
            0x1 => Ok(Instr::Jump { addr: self.nnn }),
            0x2 => Ok(Instr::Call { addr: self.nnn }),
            0x3 => Ok(Instr::SkipNextEqualLiteral {
                reg: self.x,
                lit: self.kk,
            }),
            0x4 => Ok(Instr::SkipNextNotEqualLiteral {
                reg: self.x,
                lit: self.kk,
            }),
            0x5 => Ok(Instr::SkipNextEqualRegister {
                left: self.x,
                right: self.y,
            }),
            0x6 => Ok(Instr::RegisterSetLiteral {
                reg: self.x,
                lit: self.kk,
            }),
            0x7 => Ok(Instr::RegisterAddAssign {
                reg: self.x,
                lit: self.kk,
            }),
            0x8 => match self.n {
                0x0 => Ok(Instr::RegisterSetRegister {
                    left: self.x,
                    right: self.y,
                }),
                0x1 => Ok(Instr::RegisterSetRegisterBitwiseOr {
                    left: self.x,
                    right: self.y,
                }),
                0x2 => Ok(Instr::RegisterSetRegisterBitwiseAnd {
                    left: self.x,
                    right: self.y,
                }),
                0x3 => Ok(Instr::RegisterSetRegisterBitwiseXor {
                    left: self.x,
                    right: self.y,
                }),
                0x4 => Ok(Instr::RegisterSetRegisterAdd {
                    left: self.x,
                    right: self.y,
                }),
                0x5 => Ok(Instr::RegisterSetRegisterSub {
                    left: self.x,
                    right: self.y,
                }),
                0x6 => Ok(Instr::RegisterSetRegisterShr {
                    left: self.x,
                    right: self.y,
                }),
                0x7 => Ok(Instr::RegisterSetRegisterSubn {
                    left: self.x,
                    right: self.y,
                }),
                0xE => Ok(Instr::RegisterSetRegisterShl { reg: self.x }),
                _ => Err(EmulatorError::UnknownOpcode(self)),
            },
            0x9 => Ok(Instr::SkipNextNotEqualRegister {
                left: self.x,
                right: self.y,
            }),
            0xA => Ok(Instr::SetIndex { value: self.nnn }),
            0xB => Ok(Instr::JumpTo { addr: self.nnn }),
            0xC => Ok(Instr::RandBitwiseAnd {
                reg: self.x,
                lit: self.kk,
            }),
            0xD => Ok(Instr::DrawSprite {
                x: self.x,
                y: self.y,
                size: self.n,
            }),
            0xE => match self.kk {
                0x9E => Ok(Instr::SkipNextKeyPressed { reg: self.x }),
                0xA1 => Ok(Instr::SkipNextKeyNotPressed { reg: self.x }),
                _ => Err(EmulatorError::UnknownOpcode(self)),
            },
            0xF => match self.kk {
                0x07 => Ok(Instr::SetDelayTimerValue {
                    reg: self.x,
                }),
                0x0A => Ok(Instr::KeyPressWait { reg: self.x }),
                0x15 => Ok(Instr::SetDelayTimerRegister { reg: self.x }),
                0x18 => Ok(Instr::SetSoundTimerRegister { reg: self.x }),
                0x1E => Ok(Instr::IndexAddAssignRegister { reg: self.x }),
                0x29 => Ok(Instr::SetIndexToDigitSprite { reg: self.x }),
                0x33 => Ok(Instr::StoreBCDAtIndex { reg: self.x }),
                0x55 => Ok(Instr::StoreRegistersAtIndex { start_addr: self.x }),
                0x65 => Ok(Instr::ReadRegistersAtIndex { start_addr: self.x }),
                _ => Err(EmulatorError::UnknownOpcode(self)),
            },
            _ => Err(EmulatorError::UnknownOpcode(self)),
        }
    }
}
