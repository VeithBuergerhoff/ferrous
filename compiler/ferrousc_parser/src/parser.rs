use ferrousc_ast::nodes::*;
use ferrousc_lexer::{Token, TokenKind};

const ASSIGNMENT_TOKENS: [TokenKind; 2] = [
    TokenKind::Equal, 
    TokenKind::QuestionQuestionEquals,
];

const OPERATORS: [TokenKind; 44] = [
    TokenKind::Plus, 
    TokenKind::Minus,     
    TokenKind::Star,     
    TokenKind::Slash,
    TokenKind::Bang,
    TokenKind::LBracket,
    TokenKind::QuestionLBracket,

    TokenKind::Dot,
    TokenKind::QuestionDot,
    TokenKind::DotDot,
    TokenKind::DotDotEqual,
    TokenKind::ColonColon,  

    TokenKind::MinusMinus,
    TokenKind::PlusPlus,
    TokenKind::Caret,
    TokenKind::Percent,
    TokenKind::Tilde,    
    TokenKind::Amp,
    TokenKind::AmpAmp,
    TokenKind::Bar,
    TokenKind::BarBar,    
    TokenKind::Greater,
    TokenKind::GreaterGreater,
    TokenKind::Less,
    TokenKind::LessLess,
    TokenKind::EqualEqual,

    TokenKind::Equal,
    TokenKind::PlusEqual,
    TokenKind::MinusEqual,
    TokenKind::PercentEqual,
    TokenKind::StarEqual,
    TokenKind::SlashEqual,
    TokenKind::AmpEqual,
    TokenKind::BarEqual,
    TokenKind::BangEqual,
    TokenKind::GreaterEqual,
    TokenKind::LessEqual,
    TokenKind::CaretEqual,
    TokenKind::TildeEqual,        
    TokenKind::LessLessEqual,
    TokenKind::GreaterGreaterEqual,
    TokenKind::QuestionQuestionEquals, 
  
    TokenKind::Question,
    TokenKind::QuestionQuestion,
];

const INT_TYPES: [&str; 16] = [
    "sbyte", "i8", 
    "short", "i16", 
    "int", "i32", 
    "long", "i64",

    "byte", "u8", 
    "ushort", "u16", 
    "uint", "u32", 
    "ulong", "u64",
];

const FLOAT_TYPES: [&str; 4] = [
    "float", "f32", 
    "double", "f64",
];

const STRING_TYPES: [&str; 1] = [
    "string"
];

const CHAR_TYPES: [&str; 1] = [
    "char"
];

const BOOL_TYPES: [&str; 1] = [
    "bool"
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
        
        if next.is_none() {
            todo!("early return some indication of end of tokens here!");
        }

        match next.unwrap().kind {
            TokenKind::LetKeyword => self.parse_var_definition(),
            TokenKind::LBrace => self.parse_block_statement(),
            TokenKind::IfKeyword => self.parse_if_statement(),
            TokenKind::BreakKeyword => self.parse_break_statement(),
            TokenKind::ReturnKeyword => self.parse_return_statement(),
            TokenKind::WhileKeyword => self.parse_while_statement(),
            TokenKind::FunctionKeyword => self.parse_function_definition(),
            TokenKind::ForKeyword => self.parse_for_statement(),
            _ if is_possible_expression(&self.peek()) => self.parse_expression_statement(),
            _ => {
                let _unexpected_token = self.eat();
                let statement = self.parse_statement();
                // TODO: unwrap statment until a valid statement or end of tokens is found 
                // then aggregate the tokens together and add them in diagnostics before returning
                statement
            }
        }
    }

    fn parse_for_statement(&mut self) -> Stat {        
        let for_token = self.parse_token();
        let identifier = self.parse_identifier();
        let in_token = self.parse_expected_token(TokenKind::InKeyword);
        let range = self.parse_expression();
        let statement = Box::new(self.parse_statement());

        Stat::For{ for_token, identifier, in_token, range, statement }
    }

    fn parse_expression_statement(&mut self) -> Stat {
        let expr = self.parse_expression();
        let semicolon_token = self.parse_expected_token(TokenKind::Semicolon);

        Stat::Expr{ expr, semicolon_token }
    }

    fn parse_return_statement(&mut self) -> Stat {        
        let return_token = self.parse_token();
        let expression = if !is_some_and_kind(&self.peek(), TokenKind::Semicolon) {
            Some(self.parse_expression())
        }
        else {
            None
        };
        let semicolon_token = self.parse_expected_token(TokenKind::Semicolon);

        Stat::Return{return_token, expression, semicolon_token}
    }

    fn parse_break_statement(&mut self) -> Stat {        
        let break_token = self.parse_token();
        let semicolon_token = self.parse_expected_token(TokenKind::Semicolon);

        Stat::Break{break_token, semicolon_token}
    }

    fn parse_while_statement(&mut self) -> Stat {        
        let while_token = self.parse_token();

        let expression = self.parse_expression();
        let statement = self.parse_statement();

        Stat::While{while_token, expression, statement: Box::new(statement)}
    }

    fn parse_if_statement(&mut self) -> Stat {        
        let if_token = self.parse_token();

        let expression = self.parse_expression();
        let statement = self.parse_statement();
        
        let else_statement = self.parse_else_statement();

        Stat::If{if_token, expression, statement: Box::new(statement), else_statement}
    }

    fn parse_else_statement(&mut self) -> Option<Box<Stat>> { 
        if is_some_and_kind(&self.peek(), TokenKind::ElseKeyword) {        
            let else_token = self.parse_token();
            let statement = self.parse_statement();
            Some(Box::new(Stat::Else{else_token, statement: Box::new(statement)}))
        }
        else {
            None
        }
    }

    fn parse_block_statement(&mut self) -> Stat {        
        let mut statements = vec![];
        let l_brace = self.parse_token();

        while !is_some_and_kind(&self.peek(), TokenKind::RBrace) {
            statements.push(self.parse_statement());
        }

        let r_brace = self.parse_expected_token(TokenKind::RBrace);

        Stat::Block{l_brace, statements, r_brace}
    }

    fn parse_function_definition(&mut self) -> Stat {
        let fn_token = self.parse_token();

        let identifier = self.parse_identifier();

        let parameter_list = self.parse_parameter_list();

        let return_type = self.parse_function_return_type();

        let body = self.parse_function_body();

        Stat::FunctionDefinition{ fn_token, identifier, parameter_list, return_type, body }
    }



    fn parse_function_body(&mut self) -> Box<FunctionBody> {
        if is_some_and_kind(&self.peek(), TokenKind::EqualsGreater) {
            let fat_arrow_token = self.parse_token();
            let statement = self.parse_statement();
            Box::new(FunctionBody::ExpressionBody{ fat_arrow_token, statement })
        }
        else {
            let block = self.parse_block_statement();
            Box::new(FunctionBody::BlockStatement{ block })
        }
    }

    fn parse_function_return_type(&mut self) -> Option<ReturnType> {
        if is_some_and_kind(&self.peek(), TokenKind::MinusGreater) {
            let small_arrow_token = self.parse_token();
            let type_kind = self.parse_type();
            Some(ReturnType{ small_arrow_token, type_kind })
        }
        else {
            None
        }
    }

    fn parse_parameter_list(&mut self) -> ParameterList {
        let l_paran = self.parse_token();

        let parameters = self.parse_parameters();

        let r_paran = self.parse_expected_token(TokenKind::RParen);

        ParameterList{ l_paran, parameters, r_paran }
    }
    
    fn parse_parameters(&mut self) -> Vec<Parameter> {
        let mut parameters = Vec::<Parameter>::new();

        while is_some_and_kind(&self.peek(), TokenKind::Identifier) {
            let identifier = self.parse_identifier();
            let type_id = self.parse_type_id().unwrap();
            
            let comma_token = if is_some_and_kind(&self.peek(), TokenKind::Comma) {
                Some(self.parse_token())
            }
            else {
                None
            };

            parameters.push(Parameter{ identifier, type_id, comma_token });
        }

        parameters
    }

    fn parse_var_definition(&mut self) -> Stat {
        let let_token = self.parse_token();

        let mut_token = if is_some_and_kind(&self.peek(), TokenKind::MutKeyword) {
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
        if !is_some_and_some_kind(&self.peek(), ASSIGNMENT_TOKENS.iter()) {
            return None;
        }

        let equals_token = self.parse_token();
        let expression = self.parse_expression();

        Some(EqualsValue{ equals_token, expression })
    }

    fn parse_type(&mut self) -> TypeKind {
        let identifier = self.parse_identifier();
        if INT_TYPES.contains(&identifier.identifier.token.value.as_str()) 
        || FLOAT_TYPES.contains(&identifier.identifier.token.value.as_str()) 
        || STRING_TYPES.contains(&identifier.identifier.token.value.as_str())
        || CHAR_TYPES.contains(&identifier.identifier.token.value.as_str())
        || BOOL_TYPES.contains(&identifier.identifier.token.value.as_str()) {
            TypeKind::Internal{identifier}
        }
        else {
            TypeKind::UserDefined{identifier}
        }
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_expression_bp(0)
    }

    fn parse_expression_bp(&mut self, min_bp: u8) -> Expr {
        // for more information: https://en.wikipedia.org/wiki/Operator-precedence_parser
        // based on: https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

        let mut lhs = if is_operator(&self.peek()) 
                            // special rule for lbracket, since they are used for array initializers!
                            && self.peek().unwrap().kind != TokenKind::LBracket {

            let op = self.parse_token();
            let ((), r_bp) = prefix_binding_power(op.token.kind);
            let rhs = self.parse_expression_bp(r_bp);
            bake_unary_expression(op, rhs)
        }
        else if is_some_and_kind(&self.peek(), TokenKind::LParen) {
            let lparen = self.parse_token();
            let expr = self.parse_expression_bp(0);
            let rparen = self.parse_expected_token(TokenKind::RParen);

            decorate_expression(lparen, rparen, expr)
        }
        else {
            self.parse_expression_atom()
        };
        
        loop {
            if !is_operator(&self.peek()) {
                break;
            }

            if let Some((l_bp, ())) = postfix_binding_power(self.peek().unwrap().kind) {
                if l_bp < min_bp {
                    break;
                }

                let op = self.parse_token();

                lhs = if op.token.kind == TokenKind::LBracket 
                        || op.token.kind == TokenKind::QuestionLBracket {
                    let lbracket = op;
                    let expr = self.parse_expression_bp(0);
                    let rbracket = self.parse_expected_token(TokenKind::RBracket);
                    Expr::Index{ lhs: Box::new(lhs), lbracket, expr: Box::new(expr), rbracket }
                } else {
                    bake_unary_expression(op, lhs)
                };
                continue;
            }

            if let Some((l_bp, r_bp)) = infix_binding_power(self.peek().unwrap().kind) {
                if l_bp < min_bp { 
                    break;
                }
                
                let op = self.parse_token();
                
                lhs = if op.token.kind == TokenKind::Question {
                let mhs = self.parse_expression_bp(0);
                let op2 = self.parse_expected_token(TokenKind::Colon);
                let rhs = self.parse_expression_bp(r_bp);
                bake_ternary_expression(lhs, op, mhs, op2, rhs)
                } else {
                    let rhs = self.parse_expression_bp(r_bp);
                    bake_binary_expression(lhs, op, rhs)
                };
                
                continue;
            }
            
            break;
        }

        lhs
    }

    fn parse_expression_atom(&mut self) -> Expr {
        if let Some(expr) = self.peek() {
            match expr.kind {
                TokenKind::NumberLiteral{..} =>
                    Expr::Literal{ kind: LiteralKind::Number{ number_literal: self.parse_token() } },
                TokenKind::StringLiteral{..} => 
                    Expr::Literal{ kind: LiteralKind::String{ string_literal: self.parse_token() } },
                TokenKind::CharLiteral{..} =>
                    Expr::Literal{ kind: LiteralKind::Char{ char_literal: self.parse_token() } },
                TokenKind::TrueKeyword | TokenKind::FalseKeyword =>
                    Expr::Literal{ kind: LiteralKind::Bool{ bool_literal: self.parse_token() } },
                TokenKind::LBracket => self.parse_array_initializer(),
                TokenKind::MatchKeyword => self.parse_match_expression(),
                TokenKind::Identifier{..} => {
                    let identifier = self.parse_identifier();
                    
                    if let Some(expr) = self.peek() {
                        match expr.kind {
                            TokenKind::LParen => self.parse_call(identifier),
                            _ => Expr::IdentifierUsage{ identifier },
                        }
                    }
                    else {
                        panic!("unexpected end of stream")
                    }
                },
                _ => panic!("unknown expression type {:?}", expr),
            }
        }
        else {
            panic!("unexpected end of stream")
        }
    }

    fn parse_array_initializer(&mut self) -> Expr {
        let mut items = Vec::<InitializerItem>::new();

        let lbracket = self.parse_token();

        while is_possible_expression(&self.peek()) {
            let expr = self.parse_expression();
            
            let comma_token = if is_some_and_kind(&self.peek(), TokenKind::Comma) {
                Some(self.parse_token())
            }
            else {
                None
            };
            
            items.push(InitializerItem{ expr, comma_token });
        }

        let rbracket = self.parse_expected_token(TokenKind::RBracket);

        Expr::ArrayInitializer{ lbracket, items, rbracket }
    }

    fn parse_match_expression(&mut self) -> Expr {
        let match_token = self.parse_token();
        let expr = Box::new(self.parse_expression());
        let body = self.parse_match_body();
        
        Expr::Match{ match_token, expr, body }
    }

    fn parse_match_body(&mut self) -> MatchBody {
        let mut arms = Vec::<MatchArm>::new();
        let l_brace = self.parse_token();

        while is_possible_match_arm(&self.peek()) {
            arms.push(self.parse_match_arm());
        }

        let r_brace = self.parse_expected_token(TokenKind::RBrace);

        MatchBody{ l_brace, r_brace, arms }
    }

    fn parse_match_arm(&mut self) -> MatchArm {
        let pattern = match self.peek().as_ref().unwrap().kind {
            TokenKind::StringLiteral{..}
            | TokenKind::NumberLiteral{..}
            | TokenKind::CharLiteral{..}
            | TokenKind::FalseKeyword 
            | TokenKind::TrueKeyword => MatchPattern::Literal(self.parse_token()),
            _ => panic!("unexpected token found when trying to parse match pattern {:?}", self.peek()),
        };

        let fat_arrow = self.parse_expected_token(TokenKind::EqualsGreater);

        let expr = self.parse_expression();

        let comma_token = if is_some_and_kind(&self.peek(), TokenKind::Comma) {
            Some(self.parse_token())
        }
        else {
            None
        };

        MatchArm{ pattern, fat_arrow, expr, comma_token }
    }

    fn parse_call(&mut self, identifier: Identifier) -> Expr {
        let argument_list = self.parse_argument_list();
        Expr::Call{ identifier, argument_list }
    }

    fn parse_argument_list(&mut self) -> ArgumentList {
        let mut arguments = Vec::<Argument>::new();
        let l_paran = self.parse_token();
        
        while is_possible_expression(&self.peek()) {
            let expr = self.parse_expression();
            
            let comma_token = if is_some_and_kind(&self.peek(), TokenKind::Comma) {
                Some(self.parse_token())
            }
            else {
                None
            };
            
            arguments.push(Argument{ expr, comma_token });
        }
        
        let r_paran = self.parse_expected_token(TokenKind::RParen);

        ArgumentList { l_paran, arguments, r_paran}
    }

    fn parse_identifier(&mut self) -> Identifier {
        Identifier{ identifier: self.parse_expected_token(TokenKind::Identifier) }
    }

    fn parse_type_id(&mut self) -> Option<TypeId> {
        if !is_some_and_kind(&self.peek(), TokenKind::Colon) {
            return None;
        }

        let colon_token = self.parse_token();
        let type_kind = self.parse_type();
        Some(TypeId{ colon_token, type_kind })
    }

    fn parse_token(&mut self) -> SyntaxToken {
        SyntaxToken{ token: self.eat().unwrap(), trivia: self.eat_trivia(), diagnostics: vec![], }
    }

    fn parse_expected_token(&mut self, expected_kind: TokenKind) -> SyntaxToken {
        if is_some_and_kind(&self.peek(), expected_kind) {
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

fn is_some_and_kind(token: &Option<Token>, kind: TokenKind) -> bool {
    token.is_some() && token.as_ref().unwrap().kind == kind
}

fn is_some_and_some_kind<'a>(token: &Option<Token>, kinds: impl Iterator<Item = &'a TokenKind>) -> bool {
    if token.is_none() {
        return false;
    }

    is_some_kind(token.as_ref().unwrap().kind, kinds)
}

fn is_some_kind<'a>(kind: TokenKind, mut kinds: impl Iterator<Item = &'a TokenKind>) -> bool {
    kinds.any(|k| kind == *k)
}

fn is_possible_match_arm(token: &Option<Token>) -> bool {
    token.is_some() && matches!(token.as_ref().unwrap().kind, TokenKind::StringLiteral{..}
        | TokenKind::NumberLiteral{..}
        | TokenKind::CharLiteral{..}
        | TokenKind::FalseKeyword 
        | TokenKind::TrueKeyword)
}

fn is_operator(token: &Option<Token>) -> bool {
    is_some_and_some_kind(token, OPERATORS.iter())
}

fn is_possible_expression(token: &Option<Token>) -> bool {
    token.is_some() && 
    (is_some_kind(token.as_ref().unwrap().kind, OPERATORS.iter()) 
    || matches!(token.as_ref().unwrap().kind, TokenKind::StringLiteral{..} 
                                    | TokenKind::CharLiteral{..} 
                                    | TokenKind::NumberLiteral{..} 
                                    | TokenKind::TrueKeyword 
                                    | TokenKind::FalseKeyword
                                    | TokenKind::MatchKeyword
                                    | TokenKind::Identifier))
}

fn prefix_binding_power(kind: TokenKind) -> ((), u8) {
    match kind {
        TokenKind::MinusMinus
        | TokenKind::PlusPlus
        | TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Tilde
        | TokenKind::Bang => ((), 29),
        _ => panic!("unknown operator: {:?}", kind),
    }
}

fn infix_binding_power(kind: TokenKind) -> Option<(u8, u8)> {
    match kind {        
        TokenKind::QuestionDot
        | TokenKind::Dot
        | TokenKind::ColonColon => Some((32, 31)),

        TokenKind::DotDot
        | TokenKind::DotDotEqual => Some((28, 27)),

        TokenKind::Star 
        | TokenKind::Slash
        | TokenKind::Percent => Some((25, 26)),

        TokenKind::Plus 
        | TokenKind::Minus => Some((23, 24)),

        TokenKind::LessLess
        | TokenKind::GreaterGreater => Some((21, 22)),

        TokenKind::Greater
        | TokenKind::Less
        | TokenKind::GreaterEqual
        | TokenKind::LessEqual => Some((19, 20)),

        TokenKind::EqualEqual
        | TokenKind::BangEqual => Some((17, 18)),

        TokenKind::Amp => Some((15, 16)),
        
        TokenKind::Caret => Some((13, 14)),

        TokenKind::Bar => Some((11, 12)),

        TokenKind::AmpAmp => Some((9, 10)),

        TokenKind::BarBar => Some((7, 8)),

        TokenKind::QuestionQuestion => Some((6, 5)),  

        TokenKind::Question => Some((4, 3)),  

        TokenKind::Equal
        | TokenKind::PlusEqual
        | TokenKind::MinusEqual
        | TokenKind::PercentEqual
        | TokenKind::StarEqual
        | TokenKind::SlashEqual
        | TokenKind::AmpEqual 
        | TokenKind::BarEqual
        | TokenKind::LessLessEqual
        | TokenKind::GreaterGreaterEqual
        | TokenKind::QuestionQuestionEquals
        | TokenKind::CaretEqual
        | TokenKind::TildeEqual => Some((2, 1)),
        _ => None,
    }
}

fn postfix_binding_power(kind: TokenKind) -> Option<(u8, ())> {
    match kind {
        TokenKind::MinusMinus
        | TokenKind::PlusPlus
        | TokenKind::QuestionLBracket
        | TokenKind::LBracket => Some((30, ())),
        _ => None,
    }
}

fn bake_ternary_expression(lhs: Expr, op1: SyntaxToken, mhs: Expr, op2: SyntaxToken, rhs: Expr) -> Expr {
    Expr::Ternary{ lhs: Box::new(lhs), op1, mhs: Box::new(mhs), op2, rhs: Box::new(rhs) }
}

fn bake_binary_expression(lhs: Expr, op: SyntaxToken, rhs: Expr) -> Expr {
    Expr::Binary{ lhs: Box::new(lhs), op, rhs: Box::new(rhs) }
}

fn bake_unary_expression(op: SyntaxToken, operand: Expr) -> Expr {
    Expr::Unary{ op, operand: Box::new(operand) }
}

fn decorate_expression(l: SyntaxToken, r: SyntaxToken, expr: Expr) -> Expr {
    Expr::Decorated{ l, expr: Box::new(expr), r }
}                       