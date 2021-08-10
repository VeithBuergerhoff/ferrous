use cursor::Cursor;

mod cursor;

const WHITESPACE_CHARS: [char; 23] = [
    ' ',          // space
    '\t', // tab
    '\u{000B}', // vertical tab
    '\u{000C}', // form feed
    '\u{00A0}', // no break space
    '\u{1680}', // ogham space mark
    '\u{180E}', // mongolian vowel seperator
    '\u{2000}', // en quad
    '\u{2001}', // em quad
    '\u{2002}', // en space
    '\u{2003}', // em space
    '\u{2004}', // three-per-em space
    '\u{2005}', // four-per-em space
    '\u{2006}', // six-per-em space
    '\u{2007}', // figure space
    '\u{2008}', // punctuation space
    '\u{2009}', // thin space
    '\u{200A}', // hair space
    '\u{200B}', // zero-width space
    '\u{202F}', // narrow no-break space
    '\u{205F}', // medium mathmatical space
    '\u{3000}', // idiographic space
    '\u{FEFF}', // zero with no-break space
];

const NEWLINE_CHARS: [char; 2] = [
    '\n',
    '\r',
];

/// does not include whitespace or newline characters!
const NON_LITERAL_CHARS: [char; 24] = [
    '+',
    '-',
    '*',
    '/',
    '%',
    '&',
    '|',
    '!',
    '=',
    '^',
    '<',
    '>',
    ';',
    ':',
    ',',
    '.',
    '(',
    ')',
    '[',
    ']',
    '{',
    '}',
    '~',
    '?',
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    Whitespace,
    Newline,

    LineComment,
    MultilineComment { terminated: bool },

    Slash,
    Star,
    StarStar,
    Minus,
    MinusMinus,
    Plus,
    PlusPlus,
    Caret,
    Percent,
    Tilde,

    Amp,
    AmpAmp,
    Bar,
    BarBar,

    Greater,
    GreaterGreater,
    Less,
    LessLess,
    EqualEqual,

    Equal,
    PlusEqual,
    MinusEqual,
    PercentEqual,
    StarEqual,
    SlashEqual,
    AmpEqual,
    BarEqual,
    BangEqual,
    GreaterEqual,
    LessEqual,
    CaretEqual,
    TildeEqual,
    QuestionQuestionEquals,

    LBracket,
    RBracket,
    LBrace,
    RBrace,
    LParen,
    RParen,

    Dot,
    DotDot,
    Comma,
    Semicolon,
    Colon,
    ColonColon,

    Bang,
    Question,
    QuestionQuestion,

    StringLiteral { terminated: bool },
    NumberLiteral { base: Base, has_digits: bool},
    Identifier,

    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    Binary,
    Hexadecimal,
    Octal,
    Decimal,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub len: usize,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, value: String, len: usize) -> Token {
        Token { kind, value, len }
    }
}

fn current_token(src: &str) -> Token {
    Cursor::new(src).advance_token()
}

pub fn tokenize(mut src: &str) -> impl Iterator<Item = Token> + '_ {
    std::iter::from_fn(move || {
        if src.is_empty() {
            return None;
        }
    
        let token = current_token(src);
        src = &src[token.len..];
        Some(token)
    })
}

impl Cursor<'_> {
    pub(crate) fn advance_token(&mut self) -> Token {
        let c = self.eat();

        let trivia = self.eat_trivia(c);
        if let Some(t) = trivia {
            return t;
        }

        match c {
            '/' => match self.peek() {
                '/' => {
                    self.eat();
                    let mut lexeme = "//".to_owned();
                    lexeme.push_str(&self.lex_to_eol());
                    let len = lexeme.chars().count();
                    Token::new(TokenKind::LineComment, lexeme, len)
                },
                _ => Token::new(TokenKind::Slash, "/".to_owned(), 1),
            },
            '*' => match self.peek() {
                '=' => {
                    self.eat();
                    Token::new(TokenKind::StarEqual, "*=".to_owned(), 2)
                },
                '*' => {
                    self.eat();
                    Token::new(TokenKind::StarStar, "**".to_owned(), 2)
                },
                _ => Token::new(TokenKind::Star, "*".to_owned(), 1),
            },
            '+' => match self.peek() {
                '=' => {
                    self.eat();
                    Token::new(TokenKind::PlusEqual, "+=".to_owned(), 2)
                },
                '+' => {
                    self.eat();
                    Token::new(TokenKind::PlusPlus, "++".to_owned(), 2)
                },
                _ => Token::new(TokenKind::Plus, "+".to_owned(), 1),
            },
            '-' => match self.peek() {
                '=' => {
                    self.eat();
                    Token::new(TokenKind::MinusEqual, "-=".to_owned(), 2)
                },
                '-' => {
                    self.eat();
                    Token::new(TokenKind::MinusMinus, "--".to_owned(), 2)
                },
                _ => Token::new(TokenKind::Minus, "-".to_owned(), 1),
            },
            '&' => match self.peek() {
                '=' => {
                    self.eat();
                    Token::new(TokenKind::AmpEqual, "&=".to_owned(), 2)
                },
                '&' => {
                    self.eat();
                    Token::new(TokenKind::AmpAmp, "&&".to_owned(), 2)
                },
                _ => Token::new(TokenKind::Amp, "&".to_owned(), 1),
            },
            '|' => match self.peek() {
                '=' => {
                    self.eat();
                    Token::new(TokenKind::BarEqual, "|=".to_owned(), 2)
                },
                '|' => {
                    self.eat();
                    Token::new(TokenKind::BarBar, "||".to_owned(), 2)
                },
                _ => Token::new(TokenKind::Bar, "|".to_owned(), 1),
            },
            '>' => match self.peek() {
                '=' => {
                    self.eat();
                    Token::new(TokenKind::GreaterEqual, ">=".to_owned(), 2)
                },
                '>' => {
                    self.eat();
                    Token::new(TokenKind::GreaterGreater, ">>".to_owned(), 2)
                },
                _ => Token::new(TokenKind::Greater, ">".to_owned(), 1),
            },
            '<'=> match self.peek() {
                '=' => {
                    self.eat();
                    Token::new(TokenKind::LessEqual, "<=".to_owned(), 2)
                },
                '<' => {
                    self.eat();
                    Token::new(TokenKind::LessLess, "<<".to_owned(), 2)
                },
                _ => Token::new(TokenKind::Less, "<".to_owned(), 1),
            },
            '?'=> match self.peek() {
                '?' => {
                    self.eat();
                    match self.peek() {
                        '=' => {
                            self.eat();
                            Token::new(TokenKind::QuestionQuestionEquals, "??=".to_owned(), 3)
                        },
                        _ => Token::new(TokenKind::QuestionQuestion, "??".to_owned(), 2),
                    }
                },
                _ => Token::new(TokenKind::Question, "<".to_owned(), 1),
            },
            '=' => match self.peek() {
                '=' => {
                    self.eat();
                    Token::new(TokenKind::EqualEqual, "==".to_owned(), 2)
                },
                _ => Token::new(TokenKind::Equal, "=".to_owned(), 1),
            },
            '!' => match self.peek() {
                '=' => {
                    self.eat();
                    Token::new(TokenKind::BangEqual, "!=".to_owned(), 2)
                },
                _ => Token::new(TokenKind::Bang, "!".to_owned(), 1),
            },
            '%' => match self.peek() {
                '=' => {
                    self.eat();
                    Token::new(TokenKind::PercentEqual, "%=".to_owned(), 2)
                },
                _ => Token::new(TokenKind::Percent, "%".to_owned(), 1),
            },
            ':' => match self.peek() {
                ':' => {
                    self.eat();
                    Token::new(TokenKind::ColonColon, "::".to_owned(), 2)
                },
                _ => Token::new(TokenKind::Colon, ":".to_owned(), 1),
            },
            '.' => match self.peek() {
                '.' => {
                    self.eat();
                    Token::new(TokenKind::DotDot, "..".to_owned(), 2)
                },
                _ => Token::new(TokenKind::Dot, ".".to_owned(), 1),
            },
            '~' => match self.peek() {
                '=' => {
                    self.eat();
                    Token::new(TokenKind::TildeEqual, "~=".to_owned(), 2)
                },
                _ => Token::new(TokenKind::Tilde, "~".to_owned(), 1),
            },
            '^' => match self.peek() {
                '=' => {
                    self.eat();
                    Token::new(TokenKind::CaretEqual, "^=".to_owned(), 2)
                },
                _ => Token::new(TokenKind::Caret, "^".to_owned(), 1),
            },
            ',' => Token::new(TokenKind::Comma, ",".to_owned(), 1),
            ';' => Token::new(TokenKind::Semicolon, ";".to_owned(), 1),
            '(' => Token::new(TokenKind::LParen, "(".to_owned(), 1),
            ')' => Token::new(TokenKind::RParen, ")".to_owned(), 1),
            '[' => Token::new(TokenKind::LBracket, "[".to_owned(), 1),
            ']' => Token::new(TokenKind::RBracket, "]".to_owned(), 1),
            '{' => Token::new(TokenKind::LBrace, "{".to_owned(), 1),
            '}' => Token::new(TokenKind::RBrace, "}".to_owned(), 1),
            '"' => self.lex_string_literal(&c),
            '0'..='9' => self.lex_number_literal(&c),
            c if is_literal(&c) => self.lex_identifier(&c),
            _ => panic!("unknown match encountered when every possible char should be handled!")
        }
    }

    fn eat_trivia(&mut self, mut current_char: char) -> Option<Token> {
        let newline = self.lex_newline(&current_char);        
        if let Some(token) = newline {
            return Some(token);
        }
        
        let mut lexeme: String = String::new();
        while potential_whitespace(&current_char) {
            lexeme.push(current_char);
            current_char = self.eat();
        }

        if lexeme.is_empty() {
            None
        }
        else {
            let len = lexeme.chars().count();
            Some(Token::new(TokenKind::Whitespace, lexeme, len))
        }
    }

    fn lex_to_eol(&mut self) -> String {
        let mut lexeme = String::new();
        while !self.is_eof() && !potential_eol(&self.peek()) {
            lexeme.push(self.eat());
        }
        return lexeme;
    }

    fn lex_newline(&mut self, char: &char) -> Option<Token> {
        match char {
            '\r' => match self.peek() {
                '\n' => {
                    self.eat();
                    Some(Token::new(TokenKind::Newline, "\r\n".to_owned(), 2))
                },
                _ => {
                    Some(Token::new(TokenKind::Newline, "\r".to_owned(), 1))
                }
            },
            '\n' => Some(Token::new(TokenKind::Newline, "\n".to_owned(), 1)),
            _ => None, 
        }
    }

    fn lex_string_literal(&mut self, char: &char) -> Token {
        let mut lexeme = String::from(*char);
        let mut terminated = false;

        while self.peek() != '"' && !self.is_eof() {
            if self.peek() == '\\' && self.peek_n(1) == '"' {
                lexeme.push(self.eat());
            }
            lexeme.push(self.eat());
        }

        if !self.is_eof() {
            lexeme.push(self.eat());
            terminated = true;
        }

        let len = lexeme.chars().count();
        Token::new(TokenKind::StringLiteral{terminated}, lexeme, len)
    }

    fn lex_number_literal(&mut self, char: &char) -> Token {
        match (char, self.peek()) {
            ('0', 'b') => self.lex_binary(char),
            ('0', 'o') => self.lex_octal(char),
            ('0', 'x') => self.lex_hexadecimal(char),
            (_, _) => self.lex_decimal(char),
        }
    }

    fn lex_binary(&mut self, c: &char) -> Token {
        let mut lexeme = String::from(*c);
        lexeme.push(self.eat());
        let mut has_digits = false;

        loop {
            match self.peek() {
                '_' => lexeme.push(self.eat()),
                '0' | '1' => {
                    has_digits = true;
                    lexeme.push(self.eat());
                },
                _ => break,
            }
        }

        let len = lexeme.chars().count();
        Token::new(TokenKind::NumberLiteral{base: Base::Binary, has_digits}, lexeme, len)
    }

    fn lex_octal(&mut self, c: &char) -> Token {
        let mut lexeme = String::from(*c);       
        lexeme.push(self.eat());
        let mut has_digits = false;

        loop {
            match self.peek() {
                '_' => lexeme.push(self.eat()),
                '0'..='7' => {
                    has_digits = true;
                    lexeme.push(self.eat());
                },
                _ => break,
            }
        }

        let len = lexeme.chars().count();
        Token::new(TokenKind::NumberLiteral{base: Base::Octal, has_digits}, lexeme, len)
    }

    fn lex_decimal(&mut self, c: &char) -> Token {
        let mut lexeme = String::from(*c);
        let mut had_dot = false;
        // todo: scientific notation for floating point
        loop {
            match self.peek() {
                '_' | '0'..='9' => lexeme.push(self.eat()),
                '.' if !had_dot => {
                    had_dot = true;
                    lexeme.push(self.eat());
                },
                _ => break,
            }
        }

        let len = lexeme.chars().count();
        Token::new(TokenKind::NumberLiteral{base: Base::Decimal, has_digits: true}, lexeme, len)
    }

    fn lex_hexadecimal(&mut self, c: &char) -> Token {
        let mut lexeme = String::from(*c);
        lexeme.push(self.eat());
        let mut has_digits = false;

        loop {
            match self.peek() {
                '_' => lexeme.push(self.eat()),
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    has_digits = true;
                    lexeme.push(self.eat());
                },
                _ => break,
            }
        }

        let len = lexeme.chars().count();
        Token::new(TokenKind::NumberLiteral{base: Base::Hexadecimal, has_digits}, lexeme, len)
    }

    fn lex_identifier(&mut self, char: &char) -> Token {
        let mut lexeme = String::from(*char);

        // todo: _ is another kind of token and _ has to be 
        // followed by a literal or number to be a valid identifier

        while is_literal(&self.peek()) {
            lexeme.push(self.eat());
        }

        let len = lexeme.chars().count();
        Token::new(TokenKind::Identifier, lexeme, len)
    }
}

fn is_literal(char: &char) -> bool {
    !NON_LITERAL_CHARS.contains(char) 
    && !NEWLINE_CHARS.contains(char) 
    && !WHITESPACE_CHARS.contains(char)
}

fn potential_eol(char: &char) -> bool {
    NEWLINE_CHARS.contains(char)
}

fn potential_whitespace(char: &char) -> bool {
    WHITESPACE_CHARS.contains(char)
}