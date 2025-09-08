mod eval;
mod opcodes;
mod parser;
mod undump;
mod vm;

use std::path::PathBuf;

use anyhow::Result;

use crate::undump::Constant;
use crate::{opcodes::Instruction, undump::Undump};
use crate::vm::{LuaState, vm_execute};

fn main() -> Result<()> {
    // let program = unindent(
    //     "
    // a = 1 + 2
    // ",
    // );
    // let ast = parser::parse(program.as_str()).unwrap();
    // let mut env = eval::Env::new();
    // eval::eval_block(ast.nodes(), &mut env)?;
    // println!("{:#?}", env);
    // let args: Vec<String> = std::env::args().collect();
    // let mut p = PathBuf::new();
    // p.push(args.get(1).unwrap());
    // let data = std::fs::read(p)?;
    // let mut ud = Undump::new(data);
    // ud.print();
    let mut state = LuaState::with_consts(vec![
        Constant::Number(100.0),
        Constant::Number(123.0),
    ]);
    let insts = vec![
        Instruction::LoadK(0x00000001),
        Instruction::Add(0x0040404c),
        // 0000 0000 0100 0000 0100 0000 0100 1100
        // BBBB BBBB BCCC CCCC CCAA AAAA AAOO OOOO
        Instruction::Return(0x0000001d),
    ];
    vm_execute(&mut state, insts);
    state.print_register();
    Ok(())
}
