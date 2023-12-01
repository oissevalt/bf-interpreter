use super::{token::Token, lexer::Lexer};
use std::io::{self, Read, Write};

pub struct Interpreter {
    lexer: Lexer
}

impl Interpreter {
    pub fn new(content: impl Into<String>) -> Self {
        let lexer = Lexer::new(content);
        Interpreter { lexer }
    }

    pub fn interpret(&mut self) {
        let tokens = self.lexer.tokenize();
        self.execute(&tokens, 0, &mut vec![0u8; 30000]);
    }

    fn execute(&mut self, tokens: &Vec<Token>, mut cursor: usize, stack: &mut Vec<u8>) -> usize {
        let mut in_loop = false;
        let mut token_cursor = 0;
        while token_cursor < tokens.len() {
            let token = &tokens[token_cursor];
            if in_loop && !matches!(token, Token::ExitLoop) {
                continue;
            }
            match token {
                Token::Left => {
                    if cursor <= 0 {
                        panic!("Pointer underflow!");
                    }
                    cursor -= 1;
                }
                Token::Right => {
                    if cursor >= 29999 {
                        panic!("Pointer overflow!");
                    }
                    cursor += 1;
                }
                Token::Plus => {
                    if stack[cursor] == 255 {
                        stack[cursor] = 0;
                    }else {
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
                    io::stdin().read_exact(&mut buffer).expect("Failed to read");
                    stack[cursor] = buffer[0];
                }
                Token::EnterLoop => {
                    let start_cursor = token_cursor;

                    let mut level = 1;
                    let mut end_cursor = start_cursor;
                    while level != 0 {
                        end_cursor += 1;
                        match tokens[end_cursor] {
                            Token::EnterLoop => level += 1,
                            Token::ExitLoop => level -= 1,
                            _ => ()
                        }
                    }

                    let loop_body = tokens[(start_cursor + 1)..end_cursor].to_vec();
                    // The loop body executes until stack[cursor] == 0.
                    // However, cursor can be called to move from the loop.
                    self.handle_loop(loop_body, cursor, stack);
                    token_cursor = end_cursor;
                    continue;
                }
                Token::ExitLoop => in_loop = false,
            }
            token_cursor += 1;
        }
        cursor
    }

    fn handle_loop(&mut self, body: Vec<Token>, mut cursor: usize, stack: &mut Vec<u8>) {
        while stack[cursor] != 0 {
            cursor = self.execute(&body, cursor, stack);
        }
    }
}
