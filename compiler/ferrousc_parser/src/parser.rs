use ferrousc_ast::nodes::*;
use ferrousc_lexer::{Token, TokenKind};
use crate::cursor::Cursor;

pub struct Parser {
    cursor: Cursor,
}

impl Parser {
    pub fn new(token_iterator: impl Iterator<Item = Token>) -> Parser {
        Parser{cursor: Cursor::new(token_iterator)}
    }

    pub fn parse(&mut self) -> CompilationUnit {
        CompilationUnit { trailin_trivia: vec![], statements: vec![], trivia: self.eat_trivia() }
    }

    fn eat_trivia(&mut self) -> Vec<Box<dyn Trivia>> {
        let mut vec = Vec::<Box<dyn Trivia>>::new();
        while let Some(token) = self.cursor.peek() {
            match token.kind {
                TokenKind::Whitespace => {
                    self.cursor.eat();
                    let boxed = Box::new(WhitespaceTrivia{whitespace_token: token});
                    vec.push(boxed);
                },
                TokenKind::Newline => {
                    self.cursor.eat();
                    let boxed = Box::new(NewlineTrivia{newline_token: token});
                    vec.push(boxed);
                },
                _ => break,
            }
        }
        vec
    }
}