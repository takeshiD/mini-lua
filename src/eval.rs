use anyhow::Result;
use core::panic;
use std::collections::HashMap;

use full_moon::ast::{BinOp, Block, Expression, LastStmt, Stmt};
use full_moon::tokenizer::TokenType;

#[derive(Debug, Clone)]
pub struct TValue {
    val: Value,
    ttag: TypeTag,
}

impl TValue {
    pub fn new(val: Value, lua_type: LuaType) -> TValue {
        TValue {
            val,
            ttag: TypeTag::from_basictype(lua_type),
        }
    }
    pub fn lua_type(&self) -> Option<LuaType> {
        self.ttag.lua_type()
    }
    pub fn value(&self) -> Value {
        self.val.clone()
    }
}

impl std::fmt::Display for TValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TValue({},{})", self.val, self.ttag)
    }
}

pub type LuaNumber = f64;
pub type LuaInteger = i64;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Integer(LuaInteger),
    Number(LuaNumber),
}

impl std::ops::Add for Value {
    type Output = Value;
    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l + r),
            (Value::Number(l), Value::Number(r)) => Value::Number(l + r),
            (_, _) => panic!("not match type"),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::Integer(n) => write!(f, "{}", n),
            Value::Number(n) => write!(f, "{}", n),
        }
    }
}

const TAG_MASK: u8 = 0x0F; // b0000_1111
const VARIANT_MASK: u8 = 0x30; // b0011_0000
const VARIANT_SHIFT: u8 = 4;
const COLLECTABILITY_FLAG: u8 = 0x40; // b0100_0000

#[derive(Debug, Clone)]
struct TypeTag {
    /// bits 0-3: BasicType
    /// bits 4-5: Variant bits
    /// bit  6: Collectability flag
    /// bit  7: unused
    bits: u8,
}

impl TypeTag {
    pub fn from_basictype(lua_type: LuaType) -> TypeTag {
        TypeTag {
            bits: lua_type as u8,
        }
    }
    fn tag(&self) -> u8 {
        self.bits & TAG_MASK
    }
    pub fn lua_type(&self) -> Option<LuaType> {
        match self.tag() {
            0 => Some(LuaType::Nil),
            1 => Some(LuaType::Boolean),
            2 => Some(LuaType::LightUserData),
            3 => Some(LuaType::Number),
            4 => Some(LuaType::String),
            5 => Some(LuaType::Table),
            6 => Some(LuaType::Function),
            7 => Some(LuaType::UserData),
            8 => Some(LuaType::Thread),
            9 => Some(LuaType::Proto),
            10 => Some(LuaType::Upval),
            11 => Some(LuaType::Deadkey),
            _ => None,
        }
    }
}

impl std::fmt::Display for TypeTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.lua_type().unwrap())
    }
}

/// called `Basic type`(LUA_T*) in lua type tag system
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum LuaType {
    Nil = 0,
    Boolean = 1,
    LightUserData = 2,
    Number = 3,
    String = 4,
    Table = 5,
    Function = 6,
    UserData = 7,
    Thread = 8,
    Proto = 9,
    Upval = 10,
    Deadkey = 11,
}
impl std::fmt::Display for LuaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            LuaType::Nil => "Nil",
            LuaType::Boolean => "Boolean",
            LuaType::LightUserData => "LightUserData",
            LuaType::Number => "Number",
            LuaType::String => "String",
            LuaType::Table => "Table",
            LuaType::Function => "Function",
            LuaType::UserData => "UserData",
            LuaType::Thread => "Thread",
            LuaType::Proto => "Proto",
            LuaType::Upval => "Upval",
            LuaType::Deadkey => "Deadkey",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct Env {
    envs: HashMap<String, TValue>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            envs: HashMap::new(),
        }
    }
    fn insert(&mut self, key: String, val: TValue) {
        self.envs.insert(key, val);
    }
}

pub fn eval_block(block: &Block, env: &mut Env) -> Result<()> {
    for stmt in block.stmts() {
        eval_stmt(stmt, env);
    }
    // eval_last_stmt(block.last_stmt(), env);
    Ok(())
}

pub fn eval_stmt(stmt: &Stmt, env: &mut Env) -> Result<()> {
    match stmt {
        Stmt::Assignment(assign) => {
            for (name, e) in assign.variables().iter().zip(assign.expressions()) {
                let val = eval_expr(e, env)?;
                env.insert(name.to_string(), val);
            }
            Ok(())
        }
        _ => unimplemented!(),
    }
}

// pub fn eval_last_stmt(last_stmt: &LastStmt, env: &mut Env) -> Result<()> {
//     match last_stmt {
//         Stmt::Assignment(assign) => {
//             for (name, e) in assign.variables().iter().zip(assign.expressions()) {
//                 let val = eval_expr(e, env)?;
//                 env.insert(name.to_string(), val);
//             }
//             Ok(())
//         }
//         _ => unimplemented!(),
//     }
// }

pub fn eval_expr(expr: &Expression, env: &Env) -> Result<TValue> {
    match expr {
        Expression::Number(tkn) => match tkn.token_type() {
            TokenType::Number { text } => Ok(TValue::new(
                Value::Number(text.as_str().parse::<f64>()?),
                LuaType::Number,
            )),
            _ => panic!(),
        },
        Expression::BinaryOperator { lhs, binop, rhs } => {
            let lhs_val = eval_expr(lhs, env)?;
            let rhs_val = eval_expr(rhs, env)?;
            match binop {
                BinOp::Plus(_) => match (lhs_val.val, rhs_val.val) {
                    (Value::Number(lf), Value::Number(rf)) => {
                        Ok(TValue::new(Value::Number(lf + rf), LuaType::Number))
                    }
                    (Value::Integer(li), Value::Integer(ri)) => {
                        Ok(TValue::new(Value::Integer(li + ri), LuaType::Number))
                    }
                    (_, _) => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
