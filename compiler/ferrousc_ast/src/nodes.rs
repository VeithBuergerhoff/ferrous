use ferrousc_lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Literal { kind: LiteralKind }
}

#[derive(Debug)]
pub enum Stat {
    VarDefinition {     
        let_token: SyntaxToken,
        mut_token: Option<SyntaxToken>,
        identifier: Identifier,
        type_id: Option<TypeId>,
        initial_value: Option<EqualsValue>,
        semicolon_token: SyntaxToken,
    },
    Block {        
        l_brace: SyntaxToken,
        statements: Vec<Stat>,
        r_brace: SyntaxToken,
    },
    While {        
        while_token: SyntaxToken,
        expression: Expr,
        statement: Box<Stat>,
    },
    FunctionDefinition {        
        fn_token: SyntaxToken,
        identifier: Identifier,
        parameter_list: ParameterList,
        return_type: Option<ReturnType>,
        statement: Box<Stat>,
    },
    If {        
        if_token: SyntaxToken,
        expression: Expr,
        statement: Box<Stat>,
        else_statement: Option<Box<Stat>>,
    },
    Else {        
        else_token: SyntaxToken,
        statement: Box<Stat>,
    },
    Break {        
        break_token: SyntaxToken,
        semicolon_token: SyntaxToken,
    },
    Return {        
        return_token: SyntaxToken,
        expression: Option<Expr>,
        semicolon_token: SyntaxToken,
    },
}

#[derive(Debug)]
pub enum LiteralKind {
    Number { number_literal: SyntaxToken },
    String { string_literal: SyntaxToken },
    Char { char_literal: SyntaxToken },
    Bool { bool_literal: SyntaxToken },
}

#[derive(Debug)]
pub struct ReturnType {
    pub small_arrow_token: SyntaxToken,
    pub identifier: Identifier,
}

#[derive(Debug)]
pub struct ParameterList {
    pub l_paran: SyntaxToken,
    pub r_paran: SyntaxToken,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug)]
pub struct Parameter {
    pub identifier: Identifier,
    pub type_id: TypeId,
    pub comma_token: Option<SyntaxToken>,
}

#[derive(Debug)]
pub struct SyntaxToken {
    pub token: Token,
    pub trivia: Vec<Trivia>,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug)]
pub struct Diagnostic {
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    MissingToken{ 
        expected: Token,
        actual: Option<Token>,
    },
}

#[derive(Debug)]
pub struct Trivia {
    pub trivia_token: Token,
}

#[derive(Debug)]
pub struct Identifier {
    pub identifier: SyntaxToken,
}

#[derive(Debug)]
pub struct TypeId {
    pub colon_token: SyntaxToken,
    pub type_name: Identifier,
}

#[derive(Debug)]
pub struct EqualsValue {
    pub equals_token: SyntaxToken,
    pub expression: Expr,
}

#[derive(Debug)]
pub struct CompilationUnit {
    pub leading_trivia: Vec<Trivia>,
    pub statements: Vec<Stat>,
}

impl CompilationUnit {
    pub fn walk(&self, it: impl Fn(&Stat)) {
        for st in &self.statements {
            it(st);
        }
    }
}