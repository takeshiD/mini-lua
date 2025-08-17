mod parser;
mod eval;

use unindent::unindent;
fn main() {
    let program = unindent(
        "
    local x = 1
    ",
    );
    println!("{:#?}", parser::parse(program.as_str()));
}
