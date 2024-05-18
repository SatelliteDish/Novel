use std::error::Error;
use json::{self, object};
pub struct Tokenizer {
    text: String,
    pointer: usize
}
impl Tokenizer {
    pub fn new(text: &str) -> Tokenizer {
        Tokenizer {
            text: String::from(text),
            pointer: 0
        }
    }
    pub fn next_token(&mut self) -> Option<String> {
        match self.text.chars().nth(0) {
            Some(char) => {
                match self.get_token(&char) {
                    Ok(val) => Some(val),
                    Err(e) => {
                        print!("{}", e);
                        None
                    }
                }
            },
            None => None
        }
    }
    fn get_token(&mut self, char: &char) -> Result<String,&str> {
        let mut token = String::new();
        if char.is_numeric() {
            token = self.get_number_literal();
        } else {
            return Err("Error: Invalid Syntax");
        }
        Ok(String::from(token))
    }
    fn get_number_literal(&mut self) -> String {
        let mut number = String::new();
        for char in self.text.chars() {
            self.pointer += 1;
            print!("{}\n",char);
            if char.is_numeric() || char == '.' {
                number.push(char);
                print!("number = {}", number);
            } else { break; }
        }
        json::stringify(object!{
            type: "NumericLiteral",
            value: match number.parse::<f64>() {
                Ok(num) => num,
                Err(e) => {
                    print!("Error: {} \n", e);
                    0.0
                }
            }
        })
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
