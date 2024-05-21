use std::fs::read_to_string;
use parser;

fn main() {
    let text = match read_to_string("./src/code/First.nvl")  {
        Ok(val) => val,
        Err(_e) => format!("Error: {}",_e),
    };
    println!("{}\n",parser::parse(&text));
}
