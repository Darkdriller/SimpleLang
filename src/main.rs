use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
mod lexer;
mod parser;
mod ast;

use lexer::Lexer;
use parser::Parser;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid arguments"));
    }
    //set env variable rust_backtrace=1 to see backtrace
    env::set_var("RUST_BACKTRACE", "1");
    
    let input_filename = &args[1];
    let source_code = read_file(input_filename)?;
    let mut lexer = Lexer::new(source_code);
    let tokens = lexer.tokenize();
    //print tokens with token number 
    for (i, token) in tokens.iter().enumerate() {
        println!("Token {}: {:?}", i, token);
    }
    let mut parser = Parser::new(tokens);

    let ast = parser.parse();

    let output_filename = generate_output_filename(input_filename);

    let mut output_file = File::create(output_filename.clone())?;
    writeln!(output_file, ".text")?;
    ast.generate_code(&mut output_file)?;
    writeln!(output_file, "hlt")?; 
    println!("Assembly code successfully written to {}", output_filename);
    Ok(())
}

fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn generate_output_filename(input_filename: &str) -> String {
    let path = Path::new(input_filename);
    let output_filename = path.with_extension("asm");
    output_filename.to_string_lossy().into_owned()
}
