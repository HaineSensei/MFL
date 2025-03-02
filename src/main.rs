use std::env;
use std::fs;
use std::process;

mod mfl_regex;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <filename.mfl>", args[0]);
        process::exit(1);
    }
    
    let filename: &String = &args[1];
    
    let source: String = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file {}: {}", filename, err);
            process::exit(1);
        }
    };
    let tokens = lexer::tokenise(&source);

}
