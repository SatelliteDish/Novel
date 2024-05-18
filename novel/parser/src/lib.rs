use tokenizer::{self, Tokenizer};

pub fn parse(text: &str) -> String {
    let mut tokenizer = Tokenizer::new(text);
    match tokenizer.next_token() {
        Some(text) => text,
        None => " ".to_string()
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
