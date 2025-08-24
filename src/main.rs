mod eval;
mod parser;
mod opcodes;
mod undump;

use anyhow::Result;
use unindent::unindent;

fn main() -> Result<()> {
    let program = unindent(
        "
    a = 1 + 2
    ",
    );
    let ast = parser::parse(program.as_str()).unwrap();
    let mut env = eval::Env::new();
    eval::eval_block(ast.nodes(), &mut env)?;
    println!("{:#?}", env);
    Ok(())
}
