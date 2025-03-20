mod ast;
mod compile;
mod lexer;
mod loc;
mod parser;

use std::{env, fs};

use compile::Compile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let source = fs::read_to_string(&args[1]).unwrap();

    let lexer = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(lexer);

    let (expr, _) = parser.parse_expr(parser::OperatorGroup::LeftToRight(0)).unwrap();
    println!("{}", expr);
    println!("Location: {}", expr.loc());

    let context = inkwell::context::Context::create();
    let mut compile = Compile::new(&context)?;
    compile.compile(expr)?;

    println!("{}", unsafe { compile.run() }?);

    Ok(())
}
