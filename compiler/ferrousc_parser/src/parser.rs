use ferrousc_ast::nodes::*;
use ferrousc_lexer::{Token, TokenKind};

const ASSIGNMENT_TOKENS: [TokenKind; 2] = [
    TokenKind::Equal, 
    TokenKind::QuestionQuestionEquals
];

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
        let trivia = self.eat_trivia();
        let statements = self.parse_statements();
        CompilationUnit { leading_trivia: trivia, statements }
    }

    fn parse_statements(&mut self) -> Vec<Stat> {
        let mut statements = vec![];
        while self.peek().is_some() {
            statements.push(self.parse_statement());
        }

        statements
    }

    fn parse_statement(&mut self) -> Stat {
        let next = self.peek();
        /*  
        if next.is_none() {
            // TODO: early return some indication of end of tokens here!
            return ---;
        }
        */

        match next.unwrap().kind {
            TokenKind::LetKeyword => self.parse_var_declaration(),
            _ => {
                let _unexpected_token = self.eat();
                let statement = self.parse_statement();
                // TODO: unwrap statment until a valid statement or end of tokens is found 
                // then aggregate the tokens together and add them in diagnostics before returning
                statement
            }
        }
    }

    fn parse_var_declaration(&mut self) -> Stat {
        let let_token = self.parse_token();

        let mut_token = if is_some_and_kind(self.peek(), TokenKind::MutKeyword) {
            Some(self.parse_token())
        } 
        else { 
            None 
        };

        let identifier = self.parse_identifier();
        let type_id = self.parse_type_id();
        let initial_value = self.parse_equals_value();

        let semicolon_token = self.parse_expected_token(TokenKind::Semicolon);

        Stat::VarDefinition{ let_token, mut_token, identifier, type_id, initial_value, semicolon_token }
    }

    fn parse_equals_value(&mut self) -> Option<EqualsValue> {
        if !is_some_and_some_kind(self.peek(), ASSIGNMENT_TOKENS.iter()) {
            return None;
        }

        let equals_token = self.parse_token();
        let expression = self.parse_expression();

        Some(EqualsValue{ equals_token, expression })
    }

    fn parse_expression(&mut self) -> Expr {
        Expr::Literal{ kind: LiteralKind::Number{ number_literal: self.parse_token() } }
    }

    fn parse_identifier(&mut self) -> Identifier {
        Identifier{ identifier: self.parse_expected_token(TokenKind::Identifier) }
    }

    fn parse_type_id(&mut self) -> Option<TypeId> {
        if !is_some_and_kind(self.peek(), TokenKind::Colon) {
            return None;
        }
        let colon_token = self.parse_token();
        let identifier = self.parse_identifier();
        Some(TypeId{ colon_token, type_name: identifier })
    }

    fn parse_token(&mut self) -> SyntaxToken {
        SyntaxToken{ token: self.eat().unwrap(), trivia: self.eat_trivia(), diagnostics: vec![], }
    }

    fn parse_expected_token(&mut self, expected_kind: TokenKind) -> SyntaxToken {
        if is_some_and_kind(self.peek(), expected_kind) {
            self.parse_token()
        }
        else {
            // TODO: prev whitespace could be used for expected pos and length
            let next = self.peek();
            let expected = Token{ kind: TokenKind::Identifier, len: 0, value: String::new() };
            let diagnostic = Diagnostic{ 
                kind: ErrorKind::MissingToken{
                    expected: expected.clone(), 
                    actual: next,
                },
            };
            SyntaxToken{ token: expected, trivia: self.eat_trivia(), diagnostics: vec![diagnostic], }
        }
    }

    fn eat_trivia(&mut self) -> Vec<Trivia> {
        let mut vec: Vec<Trivia> = vec![];
        while let Some(trivia_token) = self.peek() {
            match trivia_token.kind {
                TokenKind::Whitespace
                | TokenKind::Newline
                | TokenKind::LineComment 
                | TokenKind::MultilineComment{..} => {
                    self.eat();
                    vec.push(Trivia{ trivia_token });
                },
                _ => break,
            }
        }
        vec
    }
}

fn is_some_and_kind(token: Option<Token>, kind: TokenKind) -> bool {
    token.is_some() && token.unwrap().kind == kind
}

fn is_some_and_some_kind<'a>(token: Option<Token>, mut kinds: impl Iterator<Item = &'a TokenKind>) -> bool {
    if token.is_none() {
        return false;
    }

    let token = token.unwrap();
    kinds.any(|kind| token.kind == *kind)
}

