#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i32),
    Computation,
    Variable,
    Plus,
    Minus,
    Times,
    Divide,
    Remainder,
    Assignment,
    Openpar,
    Closepar,
    Semicolon,
    EOC,
}

pub struct Tokenizer {
    input: Vec<u8>,
    index: usize,
}

impl Tokenizer {
    // create an instance
    pub fn new(input: String) -> Self {
        Self {
            input: input.into_bytes(),
            index: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.peek_char() {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Times,
            '%' => Token::Remainder,
            '/' => Token::Divide,
            '(' => Token::Openpar,
            ')' => Token::Closepar,
            ';' => Token::Semicolon,
            '.' => Token::EOC,
            '<' => {
                self.next_char();
                if self.peek_char() == '-' {
                    Token::Assignment
                } else {
                    panic!("Assignment error");
                }
            },
            'a'..='z' | 'A'..='Z' => return self.get_identifier(),
            '0'..='9' => return self.get_number(),
            _ => panic!("Unexpected character to match token"),
        };

        self.next_char();
        token
    }

    pub fn peek_token(&mut self) -> Token {
        let previous_index = self.index;
        let token = self.next_token();
        self.index = previous_index;

        token
    }

    fn peek_char(&self) -> char {
        self.input[self.index] as char
    }

    fn next_char(&mut self) -> char {
        let c = self.peek_char();
        self.index += 1;

        c 
    }

    fn consume_identifier(&mut self) -> String {
        let start_index = self.index;
        // NOTE: there's a slight more way to shorten this, but might decrease readability?
        let mut c = self.peek_char();

        while c.is_ascii_alphanumeric() {
            self.index += 1;
            c = self.peek_char();

        }

        String::from_utf8_lossy(&self.input[start_index..self.index]).to_string()
    }

    fn consume_number(&mut self) -> String {
        let start_index = self.index;
        let mut c = self.peek_char();

        while c.is_ascii_digit() {
            self.index += 1;
            c = self.peek_char();

        }

        String::from_utf8_lossy(&self.input[start_index..self.index]).to_string()
    }

    fn get_identifier(&mut self) -> Token {
        let name = self.consume_identifier();
        match name.as_str() {
            "var" => Token::Variable,
            "computation" => Token::Computation,
            _ => Token::Identifier(name),
        }
    }

    fn get_number(&mut self) -> Token {
        let value = self.consume_number();

        Token::Number(value.parse::<i32>().expect("Cannot unwrap Result in get_number()"))
    }

    fn skip_whitespace(&mut self) {
        let mut c = self.peek_char();
        while c.is_ascii_whitespace() {
            self.index += 1;
            c = self.peek_char();
        }
    }
}



