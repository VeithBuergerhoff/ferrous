use ferrousc_ast::nodes::*;
use ferrousc_lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(token_iterator: impl Iterator<Item = Token>) -> Parser {
        Parser{ tokens: token_iterator.collect(), pos: 0 }
    }

    pub(crate) fn peek(&self) -> Option<Token> {
        self.peek_n(0)
    }

    pub(crate) fn peek_n(&self, n: usize) -> Option<Token> {
        match self.tokens.get(self.pos + n) {
            Some(token) => Some(token.clone()),
            None => None,
        }
    }

    pub(crate) fn eat(&mut self) -> Option<Token> {
        let token = self.peek();
        self.pos += 1;
        token
    }
}

impl Parser {
    pub fn parse(&mut self) -> CompilationUnit {
        CompilationUnit { trailin_trivia: vec![], statements: vec![], trivia: self.eat_trivia() }
    }

    fn eat_trivia(&mut self) -> Vec<Box<dyn Trivia>> {
        let mut vec = Vec::<Box<dyn Trivia>>::new();
        while let Some(token) = self.peek() {
            match token.kind {
                TokenKind::Whitespace => {
                    self.eat();
                    let boxed = Box::new(WhitespaceTrivia{whitespace_token: token});
                    vec.push(boxed);
                },
                TokenKind::Newline => {
                    self.eat();
                    let boxed = Box::new(NewlineTrivia{newline_token: token});
                    vec.push(boxed);
                },
                _ => break,
            }
        }
        vec
    }
}