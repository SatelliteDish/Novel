use std::fs::read_to_string;

mod parser;
use parser::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Requires path to file!");
        return;
    }

    let text = match get_file(&args[1]) {
        Ok(txt) => txt,
        Err(e) => {
            eprintln!("{}",e);
            std::process::exit(1);
        }
    };
    let mut parser: Parser = Parser::new(&text);
    
    unsafe {
        let tree = &parser.parse();
        println!("{}\n",tree);
    }
}

fn get_file(path: &str) -> Result<String,String> { 
    match read_to_string(path) {
        Ok(txt) => Ok(txt),
        Err(e) => Err(e.to_string())
    }
}
