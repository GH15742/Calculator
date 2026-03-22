#[derive(Debug)]
pub struct Token {
    _type: TokenType,
    _value: String,
    _object: Option<i32>,
    _location: i32,
}
impl Token {
    pub fn new(type_: TokenType, value: String, object: Option<i32>, location: i32) -> Self {
        Token {
            _type: type_,
            _value: value,
            _object: object,
            _location: location,
        }
    }
}
#[derive(Debug)]
pub enum TokenType {
    NUMBER,
    ADD,
}

// 簡單的模擬詞法分析
pub fn lexical_analysis(input: &str) -> Vec<Token> {
    let mut buf = String::with_capacity(32);
    let mut tok: Vec<Token> = Vec::new();
    let mut i = 1;
    for c in input.chars() {
        if c.is_ascii_digit() {
            // 如果是數字，則存進緩衝區
            buf.push(c);
        } else {
            // 如果不是數字，而是運算符，就將緩衝區裡的數字存進陣列，然後清空緩衝區，最後把運算符存進陣列
            if c == '+' {
                let val = buf.clone();
                let num = val.parse::<i32>().unwrap();
                tok.push(Token::new(TokenType::NUMBER, val, Some(num), i-1));
                buf.clear();
                tok.push(Token::new(TokenType::NUMBER, c.to_string(), None, i));
            }
        }
        i += 1;
    }
    if !buf.is_empty() {
        let val = buf.clone();
        let num = val.parse::<i32>().unwrap();
        tok.push(Token::new(
            TokenType::NUMBER,
            val,
            Some(num),
            i-1,
        ));
        buf.clear();
    }
    // print!("Token: {:#?}", tok);
    return tok;
}
