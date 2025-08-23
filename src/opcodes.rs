#[repr(C)]
enum OpCode {
    Move,
    LoadK,
    SetGlobal,
    GetGlobal,
    Return,
}
