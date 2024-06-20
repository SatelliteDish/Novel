use std::fs::read_to_string;

mod parser;

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
    println!("{}",std::env::current_dir().unwrap().display());
    println!("{}\n",parser::parse(&text));
}

fn get_file(path: &str) -> Result<String,String> { 
    match read_to_string(path) {
        Ok(txt) => Ok(txt),
        Err(e) => Err(e.to_string())
    }
}
