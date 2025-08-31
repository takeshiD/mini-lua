mod eval;
mod opcodes;
mod parser;
mod undump;
mod vm;

use std::env::args;
use std::path::PathBuf;

use anyhow::Result;
use unindent::unindent;

use crate::undump::Undump;

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
    let args: Vec<String> = std::env::args().collect();
    let mut p = PathBuf::new();
    p.push(args.get(1).unwrap());
    let data = std::fs::read(p)?;
    let mut ud = Undump::new(data);
    ud.print();
    Ok(())
}
