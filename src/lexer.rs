use crate::Token;

pub fn create_tokens(s: &str) -> Vec<Token> {
    println!("{}", s);

    let mut v = Vec::new();
    let mut cursor_pos = 0;

    while cursor_pos < s.len() {
        let chr = s.chars().nth(cursor_pos).unwrap_or_default();
        if chr != ' ' {
            let token = match chr as u8 {
                40 => Token::OpenBracket,
                41 => Token::CloseBracket,
                42 => Token::Multiply,
                43 => Token::Add,
                45 => Token::Subtract,
                47 => Token::Divide,
                48..=57 => {
                    let mut num_str = String::from(chr);
                    let num_range = 48..=57;
                    let mut next_ascii = s.chars().nth(cursor_pos + 1).unwrap_or_default();
                    while num_range.contains(&(next_ascii as u8)) {
                        num_str.push(next_ascii);
                        cursor_pos += 1;
                        next_ascii = s.chars().nth(cursor_pos + 1).unwrap_or_default();
                    }
                    let num: i32 = num_str.parse::<i32>().unwrap_or_default();
                    Token::Number(num)
                }
                _ => {
                    panic!("unknown token")
                }
            };
            v.push(token);
        }
        cursor_pos += 1;
    }
    v
}
