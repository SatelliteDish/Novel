use std::{
    fs::read_to_string,
    io,
    io::Write as _,
    process,
    fmt,
};

mod parser;
use parser::Parser;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let argc = args.len();

    if argc != 2 && argc != 3 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "-i" => {
            if let Err(e) = run_repl() {
                eprintln!("Error in REPL:\n{e}");
                process::exit(1);
            }
        },
        "-f" => {
            println!("File");
        },
        _ => {
            print_usage();
            process::exit(1);
        },
    };
    let mut parser: Parser = Parser::new("");




    // let tree = &parser.parse();
    // println!("{}\n",tree);
}

fn print_usage() {
    println!("USAGE: <BINARY> [-f path-to-file, -i]");
    println!("-f: File input");
    println!("-i: Interactive REPL");
}


fn get_file(path: &str) -> Result<String,String> {
    match read_to_string(path) {
        Ok(txt) => Ok(txt),
        Err(e) => Err(e.to_string())
    }
}

fn run_repl() -> Result<(),String> {
    loop {
        print!("> ");
        io::stdout().flush().map_err(|e| e.to_string())?;


        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;

        if !input.is_empty() {
            println!("> {input}");
        } else {
            break;
        }
    }

    Ok(())
}
