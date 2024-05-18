use json::{self, object};
use regex::{Regex};
use std::collections::HashMap;
#[derive(PartialEq, Eq, Hash, Clone)]

pub struct Tokenizer {
    text: String,
    pointer: usize
}
impl Tokenizer {
    pub fn new(text: &str) -> Tokenizer {
        Tokenizer {
            text: String::from(text) + " ",
            pointer: 0
        }
    }
    pub fn next_token(&mut self) -> Option<String> {
        let mut result: Option<String> = None;
        let regex = [
            ("NumericLiteral", Regex::new(r"^\d+(\.[\d]+)?").unwrap())
        ];
        for (key, regex) in regex {
            if regex.is_match(&self.text) {
                let val = regex.captures(&self.text).unwrap()[0].to_string();
                self.text = String::from(&self.text[{val.len() + 1}..]);
                result = Some(json::stringify(object!{
                    type: key.to_string(),
                    val: val,
                }))
            }
        }
        result
        /*match self.text.chars().nth(0) {
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
        } */ 
    }
    fn get_token(&mut self, char: &char) -> Result<String,&str> {
        let mut result: Result<String, &str> = Err("Error: Invalid Syntax");
        if char.is_numeric() {
            result = Ok(String::from(self.get_number_literal()))
        }
        result
    }
    fn get_number_literal(&mut self) -> String {
        let mut number = String::new();
        for char in self.text.chars() {
            self.pointer += 1;
            if char.is_numeric() || char == '.' {
                number.push(char);
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
