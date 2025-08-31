use crate::eval::TValue;

/// lua5.1
#[rustfmt::skip]
#[repr(u8)]
pub enum OpCode {
    OpMove      = 0u8,
    OpLoadK     = 1u8,
    OpLoadBool  = 2u8,
    OpLoadNil   = 3u8,
    OpGetUpval  = 4u8,
    OpGetTable  = 5u8,
    OpSetGlobal = 6u8,
    OpSetUpval  = 7u8,
    OpSetTable  = 8u8,
    OpNewTable  = 9u8,
    OpSelf      = 10u8,
    OpAdd       = 11u8,
    OpSub       = 12u8,
    OpMul       = 13u8,
    OpDiv       = 14u8,
    OpMod       = 15u8,
    OpPow       = 16u8,
    OpUnm       = 17u8,
    OpNot       = 18u8,
    OpLen       = 19u8,
    OpConcat    = 20u8,
    OpJmp       = 21u8,
    OpEq        = 22u8,
    OpLt        = 23u8,
    OpLe        = 24u8,
    OpTest      = 25u8,
    OpTestSet   = 26u8,
    OpCall      = 27u8,
    OpTailCall  = 28u8,
    OpReturn    = 29u8,
    OpForLoop   = 30u8,
    OpForPrep   = 31u8,
    OpTForLoop  = 32u8,
    OpSetList   = 33u8,
    OpClose     = 34u8,
    OpClosure   = 35u8,
    OpVarArg    = 36u8,
}

pub type OperandA = u8;
pub type OperandB = u16;
pub type OperandC = u16;
pub type OperandBx = u32;

#[repr(C)]
pub enum Instruction {
    Move(OpCode, OperandA, OperandB),
    LoadK(OpCode, OperandA, OperandBx),
    Add(OpCode, OperandA, OperandB, OperandC),
}
