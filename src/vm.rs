use core::panic;

use crate::eval::{LuaNumber, LuaType, TValue, Value};
use crate::opcodes::{Instruction, OpCode};
use crate::undump::Chunk;

struct CallInfo {
    base: TValue,
    func: TValue,
    top: TValue,
    proto: Chunk,
}

struct LuaState {
    stack: Vec<TValue>,
    ci: Vec<CallInfo>,
    base: TValue,
}

impl LuaState {
    fn set_number(&mut self, ra: usize, n: LuaNumber) {
        self.stack[ra] = TValue::new(Value::Number(n), LuaType::Number);
    }
}

macro_rules! RK {
    ($($x:expr, $base:expr)) => {
        
    };
}

fn vm_execute(state: &mut LuaState, insts: Vec<Instruction>) {
    let mut pc: usize = 0;
    loop {
        match insts[pc] {
            Instruction::Move(op, a, b) => {}
            Instruction::LoadK(op, a, bx) => {}
            Instruction::Add(op, a, b, c) => {
                let ra = a as usize;
                let rb = RK(b, state.base);
                let rc = c.as_rk(c);
                match (rb.lua_type(), rc.lua_type()) {
                    (Some(Value::Number(nb)), Some(Value::Number(nc))) => {
                        state.set_number(ra, nb + nc);
                    }
                    (Some(Value::Integer(ib)), Some(Value::Integer(ic))) => {
                        state.set_number(ra, (ib as LuaNumber) + (ic as LuaNumber));
                    }
                    (_, _) => panic!("not match lua type"),
                }
            }
            _ => unimplemented!(),
        }
    }
}
