use std::{env, fs};

use calculator::{
    compile::Compile,
    lexer::Lexer,
    parser::Parser,
    trav::Traversal,
    typecheck::TypeCheck
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let source = fs::read_to_string(&args[1]).unwrap();

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);

    let (mut expr, _) = parser.parse_expr().unwrap();
    println!("{}", expr);
    println!("Location: {}", expr.loc());

    let mut typecheck = TypeCheck { };
    let _typ = typecheck.trav_expr(&mut expr).unwrap();

    let context = inkwell::context::Context::create();
    let mut compile = Compile::new(&context)?;
    compile.compile(expr)?;

    println!("{}", unsafe { compile.run() }?);

    Ok(())
}
