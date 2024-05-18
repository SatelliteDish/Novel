use tokenizer::{self, Tokenizer};

pub fn parse(text: &str) -> String {
    let mut tokenizer = Tokenizer::new(text);
    let mut more_tokens = true;
    let mut result = String::new();
    while more_tokens {
        match tokenizer.next_token() {
            Some(token) => {
                result = result + &token;
            },
            None => {
                more_tokens = false;
            }
        } 
    }
    result
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
