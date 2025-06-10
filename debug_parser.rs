use std::env;
use std::path::Path;

mod ast;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.eppx>", args[0]);
        return;
    }

    let file_path = Path::new(&args[1]);
    println!("Parsing file: {:?}", file_path);

    match parser::parse_eppx_file(file_path) {
        Ok(ast_nodes) => {
            println!("Successfully parsed {} AST nodes:", ast_nodes.len());
            for (i, node) in ast_nodes.iter().enumerate() {
                println!("Node {}: {:?}", i, node);
            }
        }
        Err(e) => {
            println!("Parse error: {}", e);
        }
    }
}
