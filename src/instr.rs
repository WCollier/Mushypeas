#[derive(Copy, Clone, Debug)]
pub(crate) enum Instr {
    JumpToMachineCode { addr: usize },
    Clear,
    Return,
    Jump { addr: usize },
    Call { addr: usize },
    SkipNextEqualLiteral { reg: usize, lit: u8 },
    SkipNextNotEqualLiteral { reg: usize, lit: u8 },
    SkipNextEqualRegister { left: usize, right: usize },
    RegisterSetLiteral { reg: usize, lit: u8 },
    RegisterAddAssign { reg: usize, lit: u8 },
    RegisterSetRegister { left: usize, right: usize },
    RegisterSetRegisterBitwiseOr { left: usize, right: usize },
    RegisterSetRegisterBitwiseAnd { left: usize, right: usize },
    RegisterSetRegisterBitwiseXor { left: usize, right: usize },
    RegisterSetRegisterAdd { left: usize, right: usize },
    RegisterSetRegisterSub { left: usize, right: usize },
    RegisterSetRegisterShr { left: usize, right: usize },
    RegisterSetRegisterSubn { left: usize, right: usize },
    RegisterSetRegisterShl { reg: usize },
    SkipNextNotEqualRegister { left: usize, right: usize },
    SetIndex { value: usize },
    JumpTo { addr: usize },
    RandBitwiseAnd { reg: usize, lit: u8 },
    DrawSprite { x: usize, y: usize, size: u8 },
    SkipNextKeyPressed { reg: usize },
    SkipNextKeyNotPressed { reg: usize },
    SetDelayTimerValue { reg: usize },
    KeyPressWait { reg: usize },
    SetDelayTimerRegister { reg: usize },
    SetSoundTimerRegister { reg: usize },
    IndexAddAssignRegister { reg: usize },
    SetIndexToDigitSprite { reg: usize },
    StoreBCDAtIndex { reg: usize },
    StoreRegistersAtIndex { start_addr: usize },
    ReadRegistersAtIndex { start_addr: usize },
}
