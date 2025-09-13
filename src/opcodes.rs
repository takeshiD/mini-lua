/// lua5.1
#[rustfmt::skip]
#[repr(u8)]
pub enum OpCode {
    OpMove      = 0u8,
    OpLoadK     = 1u8,
    OpLoadBool  = 2u8,
    OpLoadNil   = 3u8,
    OpGetUpval  = 4u8,
    OpGetGlobal = 5u8,
    OpGetTable  = 6u8,
    OpSetGlobal = 7u8,
    OpSetUpval  = 8u8,
    OpSetTable  = 9u8,
    OpNewTable  = 10u8,
    OpSelf      = 11u8,
    OpAdd       = 12u8,
    OpSub       = 13u8,
    OpMul       = 14u8,
    OpDiv       = 15u8,
    OpMod       = 16u8,
    OpPow       = 17u8,
    OpUnm       = 18u8,
    OpNot       = 19u8,
    OpLen       = 20u8,
    OpConcat    = 21u8,
    OpJmp       = 22u8,
    OpEq        = 23u8,
    OpLt        = 24u8,
    OpLe        = 25u8,
    OpTest      = 26u8,
    OpTestSet   = 27u8,
    OpCall      = 28u8,
    OpTailCall  = 29u8,
    OpReturn    = 30u8,
    OpForLoop   = 31u8,
    OpForPrep   = 32u8,
    OpTForLoop  = 33u8,
    OpSetList   = 34u8,
    OpClose     = 35u8,
    OpClosure   = 36u8,
    OpVarArg    = 37u8,
}

pub type OperandA = u8; // 8bit
pub type OperandB = u16; // 9bit
pub type OperandC = u16; // 9bit
pub type OperandBx = u32; // 18bit

pub const SIZE_OP: usize = 6;
pub const SIZE_A: usize = 8;
pub const SIZE_C: usize = 9;
pub const SIZE_B: usize = 9;
pub const SIZE_BX: usize = 18;

pub const POS_OP: usize = 0;
pub const POS_A: usize = POS_OP + SIZE_OP;
pub const POS_C: usize = POS_A + SIZE_A;
pub const POS_B: usize = POS_C + SIZE_C;
pub const POS_BX: usize = POS_C;

pub const BITRK: usize = 1 << (SIZE_B - 1);

#[rustfmt::skip]
#[repr(u32)]
pub enum Mask {
    OP = 0b0000_0000_0000_0000_0000_0000_0011_1111,
    A  = 0b0000_0000_0000_0000_0011_1111_1100_0000,
    C  = 0b0000_0000_0111_1111_1100_0000_0000_0000,
    B  = 0b1111_1111_1000_0000_0000_0000_0000_0000,
    BX = 0b1111_1111_1111_1111_1100_0000_0000_0000,
}

#[repr(u32)]
pub enum Instruction {
    Move(u32),
    LoadK(u32),
    GetGlobal(u32),
    SetGlobal(u32),
    Add(u32),
    Return(u32),
}

pub fn is_k(x: u32) -> bool {
    (x & (BITRK as u32)) != 0
}

pub fn index_k(x: u32) -> u32 {
    x & !(BITRK as u32)
}

pub fn as_ra(inst: u32, base: usize) -> usize {
    let ra = (inst & (Mask::A as u32)) >> POS_A;
    base + (ra as usize)
}

pub fn as_rkb(inst: u32, k: usize, base: usize) -> usize {
    let rb = (inst & (Mask::B as u32)) >> POS_B;
    if is_k(rb) {
        k + (rb as usize)
    } else {
        base + (rb as usize)
    }
}

pub fn as_kbx(inst: u32, k: usize) -> usize {
    let rbx = (inst & (Mask::BX as u32)) >> POS_BX;
    k + (rbx as usize)
}

pub fn as_rkc(inst: u32, k: usize, base: usize) -> usize {
    let rc = (inst & (Mask::C as u32)) >> POS_C;
    if is_k(rc) {
        k + (rc as usize)
    } else {
        base + (rc as usize)
    }
}
