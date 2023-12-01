use super::token::Token;

pub(crate) struct Lexer {
    content: Vec<char>,
    cursor: usize,
}

impl Lexer {
    pub fn new(content: impl Into<String>) -> Self {
        let content: String = content.into();
        Lexer { content: content.chars().collect(), cursor: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.cursor < self.content.len() {
            let token = match self.content[self.cursor] {
                '<' => Token::Left,
                '>' => Token::Right,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '.' => Token::Output,
                ',' => Token::Input,
                '[' => Token::EnterLoop,
                ']' => Token::ExitLoop,
                _ => {
                    self.cursor += 1;
                    continue;
                }, // Spaces, new lines and comments
            };
            tokens.push(token);
            self.cursor += 1;
        }

        tokens
    }
}
