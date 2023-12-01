use super::{lexer::Lexer, token::Token};
use std::{
    error::Error,
    io::{self, Read, Write},
    process,
};

pub struct Interpreter {
    lexer: Lexer,
    stack_info: Vec<String>,
}

impl Interpreter {
    pub fn new(content: impl Into<String>) -> Self {
        let lexer = Lexer::new(content);
        Interpreter {
            lexer,
            stack_info: vec![],
        }
    }

    pub fn interpret(&mut self) {
        let tokens = self.lexer.tokenize();
        self.stack_info.push(String::new());
        if let Err(e) = self.execute(&tokens, 0, &mut vec![0u8; 30000]) {
            self.stack_info.reverse();
            println!(
                "\x1b[1;31mInterpreterError\x1b[0m  {}{}",
                e,
                if self.stack_info.len() != 0 {
                    format!("\n\tat cell {}", self.stack_info.join("\n\tat cell "))
                } else {
                    String::new()
                }
            );
            process::exit(1);
        }
    }

    fn execute(
        &mut self,
        tokens: &Vec<Token>,
        mut cursor: usize,
        stack: &mut Vec<u8>,
    ) -> Result<usize, Box<dyn Error>> {
        let mut token_cursor = 0;
        while token_cursor < tokens.len() {
            let token = &tokens[token_cursor];

            let last = if self.stack_info.len() == 0 {
                0
            } else {
                self.stack_info.len() - 1
            };
            self.stack_info[last] = format!(
                "{}, token: {:?} (index {})",
                cursor, tokens[token_cursor], token_cursor
            );

            match token {
                Token::Left => {
                    if cursor <= 0 {
                        Err("Pointer underflow!")?;
                    }
                    cursor -= 1;
                }
                Token::Right => {
                    if cursor >= 29999 {
                        Err("Pointer overflow!")?;
                    }
                    cursor += 1;
                }
                Token::Plus => {
                    if stack[cursor] == 255 {
                        stack[cursor] = 0;
                    } else {
                        stack[cursor] += 1;
                    }
                }
                Token::Minus => {
                    if stack[cursor] == 0 {
                        stack[cursor] = 255;
                    } else {
                        stack[cursor] -= 1;
                    }
                }
                Token::Output => {
                    let codepoint = stack[cursor];
                    print!("{}", char::from(codepoint));
                    _ = io::stdout().flush();
                }
                Token::Input => {
                    let mut buffer = [0; 1];
                    io::stdin().read_exact(&mut buffer)?;
                    stack[cursor] = buffer[0];
                }
                Token::EnterLoop => {
                    // `[`: If current cell's is 0, jump to matching `]`'s next token.
                    let mut level = 1;
                    let mut end_cursor = token_cursor;
                    while level != 0 {
                        if tokens.len() - 1 <= end_cursor
                            && !matches!(tokens[end_cursor], Token::ExitLoop)
                        {
                            Err("Loop not closed")?;
                        }
                        end_cursor += 1;
                        match tokens[end_cursor] {
                            Token::EnterLoop => level += 1,
                            Token::ExitLoop => level -= 1,
                            _ => (),
                        }
                    }
                    // start_cursor at `[`, end_cursor at outmost `]`

                    if stack[cursor] == 0 {
                        token_cursor = end_cursor + 1;
                        continue;
                    }
                }
                Token::ExitLoop => {
                    // `]`: If current cell's is not 0, jump to matching `[`'s next token.
                    if token_cursor == 0 {
                        Err("Loop end matches no start")?;
                    }

                    let mut level = 1;
                    let mut start_cursor = token_cursor;
                    while level != 0 {
                        if start_cursor == 0 && !matches!(tokens[start_cursor], Token::EnterLoop) {
                            Err("Loop end matches no start")?;
                        }
                        start_cursor -= 1;
                        match tokens[start_cursor] {
                            Token::EnterLoop => level -= 1,
                            Token::ExitLoop => level += 1,
                            _ => (),
                        }
                    }
                    // start_cursor at `[`

                    if stack[cursor] != 0 {
                        token_cursor = start_cursor + 1;
                        continue;
                    }
                }
            }
            token_cursor += 1;
        }
        Ok(cursor)
    }
}
