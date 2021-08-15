use ferrousc_ast_proc_macros::*;

use ferrousc_lexer::Token;

pub trait Node {}

pub trait Trivia : Node {}

pub trait SyntaxNode : Node {
    fn get_trivia(&self) -> &Vec<Box<dyn Trivia>>;
}

pub trait Expression : SyntaxNode {}

pub trait Statement : SyntaxNode  {}

#[trivia]
pub struct WhitespaceTrivia {
    pub whitespace_token: Token,
}

#[trivia]
pub struct NewlineTrivia {
    pub newline_token: Token,
}

#[node]
pub struct Identifier {
    pub identifier_token: Token,
    trivia: Vec<Box<dyn Trivia>>,
}

#[node]
pub struct TypeId {
    pub colon_token: Token,
    pub type_name: Identifier,
    trivia: Vec<Box<dyn Trivia>>,
}

#[node]
pub struct EqualsValue {
    pub equals_token: Token,
    pub value: Box<dyn Expression>,
    trivia: Vec<Box<dyn Trivia>>,
}


#[node]
pub struct CompilationUnit {
    pub statements: Vec<Box<dyn Statement>>,
    pub trivia: Vec<Box<dyn Trivia>>,
}

// expressions

// statements

#[statement]
pub struct VariableDefinitionStatement {
    pub let_token: Token,
    pub mut_token: Option<Token>,
    pub identifier: Identifier,
    pub type_id: Option<TypeId>,
    pub initial_value: Option<EqualsValue>,
    pub semicolon_token: Token,
    trivia: Vec<Box<dyn Trivia>>,
}