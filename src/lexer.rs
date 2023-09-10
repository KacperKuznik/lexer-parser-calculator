use crate::Token;

pub fn create_tokens(s: &str) -> Result<Vec<Token>, &'static str> {
    let mut v = Vec::new();
    let mut cursor_pos = 0;

    while let Some(chr) = s.chars().nth(cursor_pos) {
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
                    let mut next_ascii = s.chars().nth(cursor_pos + 1).unwrap_or_default();
                    while next_ascii.is_digit(10) {
                        num_str.push(next_ascii);
                        cursor_pos += 1;
                        next_ascii = s.chars().nth(cursor_pos + 1).unwrap_or_default();
                    }
                    let num: i32 = num_str.parse::<i32>().unwrap_or_default();
                    Token::Number(num)
                }
                _ => return Err("Detected unknown token"),
            };
            v.push(token);
        }
        cursor_pos += 1;
    }
    Ok(v)
}
