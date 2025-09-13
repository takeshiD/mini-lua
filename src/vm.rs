use core::panic;
use std::sync::{Arc, Mutex};

use crate::eval::{LuaNumber, LuaType, TValue, Value};
use crate::opcodes::{Instruction, as_kbx, as_ra, as_rkb, as_rkc};
use crate::undump::Constant;


pub struct Proto {
    constant_table: Vec<Constant>,
    constant_index: usize,
}

pub struct CallInfo {
    base: usize,
    func: usize,
    top: usize,
    proto: Proto,
}
impl CallInfo {
    pub fn with_consts(constant_table: Vec<Constant>) -> Self {
        Self {
            base: 0,
            func: 0,
            top: 0,
            proto: Proto {
                constant_table,
                constant_index: 0,
            },
        }
    }
}

struct GlobalState {

}

pub struct LuaState {
    stack: Vec<TValue>,
    base_ci: Vec<CallInfo>,
    ci: usize,
    base: usize,
}

impl LuaState {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            base_ci: Vec::new(),
            ci: 0,
            base: 0,
        }
    }
    pub fn with_consts(constant_table: Vec<Constant>) -> Self {
        Self {
            stack: vec![
                TValue::new(Value::Nil, LuaType::Nil),
                TValue::new(Value::Nil, LuaType::Nil),
                TValue::new(Value::Nil, LuaType::Nil),
                TValue::new(Value::Nil, LuaType::Nil),
            ],
            base_ci: vec![CallInfo::with_consts(constant_table)],
            ci: 0,
            base: 0,
        }
    }
    fn set_number(&mut self, ra: usize, tval: TValue) {
        self.stack[ra] = tval.clone();
    }
    fn set_const(&mut self, ra: usize, kst: usize) {
        match self.base_ci[self.ci].proto.constant_table[kst] {
            Constant::Number(n) => {
                self.stack[ra] = TValue::new(Value::Number(n), LuaType::Number);
            }
            _ => unimplemented!(),
        }
    }
    pub fn print_register(&self) {
        for (i, tval) in self.stack.iter().enumerate() {
            println!("R[{i:02x}] = {tval}");
        }
    }
}

pub fn vm_execute(state: &mut LuaState, insts: Vec<Instruction>) {
    let mut pc: usize = 0;
    let k = state.base_ci.first().unwrap().proto.constant_index;
    loop {
        match insts[pc] {
            // Instruction::Move(inst) => {}
            Instruction::LoadK(inst) => {
                let ra = as_ra(inst, state.base);
                let k = as_kbx(inst, state.base);
                println!("Load {} {}", ra, k);
                state.set_const(ra, k);
                pc += 1;
            }
            Instruction::Add(inst) => {
                let ra = as_ra(inst, state.base);
                let rb = as_rkb(inst, state.base, k);
                let rc = as_rkc(inst, state.base, k);
                let nb = TValue::new(Value::Integer(rb as i64), LuaType::Number);
                let nc = TValue::new(Value::Integer(rc as i64), LuaType::Number);
                println!("Add {} {} {}", ra, rb, rc);
                match (nb.lua_type(), nc.lua_type()) {
                    (Some(LuaType::Number), Some(LuaType::Number)) => {
                        state.set_number(ra, TValue::new(nb.value() + nc.value(), LuaType::Number));
                    }
                    (_, _) => panic!("not match lua type"),
                };
                pc += 1;
            }
            Instruction::GetGlobal(inst) => {
                unimplemented!()
            }
            Instruction::SetGlobal(inst) => {
                let ra = as_ra(inst, state.base);
                println!("SetGlobal {}", ra);
                pc += 1;
            }
            Instruction::Return(_) => break,
            _ => unimplemented!(),
        }
    }
    println!("Finish");
}
