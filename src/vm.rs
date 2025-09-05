use core::panic;

use crate::eval::{LuaType, TValue, Value};
use crate::opcodes::{Instruction, as_ra, as_rkb, as_rkc};
use crate::undump::Constant;

struct Proto {
    constant_table: Vec<Constant>,
    constant_index: usize,
}

struct CallInfo {
    base: TValue,
    func: TValue,
    top: TValue,
    proto: Proto,
}

struct LuaState {
    stack: Vec<TValue>,
    ci: Vec<CallInfo>,
    base: usize,
}

impl LuaState {
    fn set_number(&mut self, ra: usize, tval: TValue) {
        self.stack[ra] = tval.clone();
    }
}

fn vm_execute(state: &mut LuaState, insts: Vec<Instruction>) {
    let mut pc: usize = 0;
    let k = state.ci.first().unwrap().proto.constant_index;
    loop {
        match insts[pc] {
            // Instruction::Move(inst) => {}
            Instruction::LoadK(inst) => {

            }
            Instruction::Add(inst) => {
                let ra = as_ra(inst, state.base);
                let rb = as_rkb(inst, state.base, k);
                let rc = as_rkc(inst, state.base, k);
                let nb = TValue::new(Value::Integer(rb as i64), LuaType::Number);
                let nc = TValue::new(Value::Integer(rc as i64), LuaType::Number);
                match (nb.lua_type(), nc.lua_type()) {
                    (Some(LuaType::Number), Some(LuaType::Number)) => {
                        state.set_number(ra, TValue::new(nb.value() + nc.value(), LuaType::Number));
                    }
                    (_, _) => panic!("not match lua type"),
                }
            }
            _ => unimplemented!(),
        }
    }
}
