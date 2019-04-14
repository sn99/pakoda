mod lexer;
mod parser;

use std::env;
use std::fs;

const USAGE: &'static str = "
Usage: cargo run SOURCE_FILE OUTPUT_FILE
Options:
     
";

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("{}", USAGE);
        return Ok(());
    }

    let source_file = &args[1];
    let output_file = &args[2];

    let contents = fs::read_to_string(source_file).expect("Can't read file");

    let token_list = lexer::tokenize(&contents);
    println!("{:?}", token_list);

    let mut program = parser::Program::new(source_file, contents.as_str());
    program.parse();
    println!("\n\n{:#?}", program);

    Ok(())
}
