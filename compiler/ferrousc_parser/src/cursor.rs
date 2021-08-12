use ferrousc_lexer::Token;

pub(crate) struct Cursor {
    tokens: Vec<Token>,
    pos: usize,
}

impl Cursor {
    pub(crate) fn new(token_iterator: impl Iterator<Item = Token>) -> Cursor {
        Cursor { tokens: token_iterator.collect(), pos: 0 }
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