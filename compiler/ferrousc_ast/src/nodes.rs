use ferrousc_ast_proc_macros::*;

use ferrousc_lexer::Token;

pub trait Trivia {}

pub struct WhitespaceTrivia {
    pub whitespace_token: Token,
}
impl Trivia for WhitespaceTrivia {}

pub struct NewlineTrivia {
    pub newline_token: Token,
}
impl Trivia for NewlineTrivia {}

macro_rules! trivia_fn {
    ($name:ident) => { 
        fn get_trivia(&self) -> &Vec<Box<dyn Trivia>> { 
            &self.trivia 
        }
    }
}

macro_rules! node {
    ($name:ident) => { 
        impl SyntaxNode for $name { 
            trivia_fn!($name);
        } 
    }
}

pub trait SyntaxNode {
    fn get_trivia(&self) -> &Vec<Box<dyn Trivia>>;
}

#[node]  // TODO: build accesors for fields and ::new() method
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
    pub trailin_trivia: Vec<Box<dyn Trivia>>,
    pub statements: Vec<Box<dyn Statement>>,
    pub trivia: Vec<Box<dyn Trivia>>,
}

// expressions

pub trait Expression : SyntaxNode {}

macro_rules! expr {
    ($name:ident) => { 
        impl Expression for $name {} 
    }
}

// statements

pub trait Statement : SyntaxNode  {}

macro_rules! stat {
    ($name:ident) => { 
        impl Statement for $name {} 
    }
}


#[stat]
pub struct VariableDefinitionStatement {
    pub let_token: Token,
    pub mut_token: Option<Token>,
    pub identifier: Identifier,
    pub type_id: Option<TypeId>,
    pub initial_value: Option<EqualsValue>,
    pub semicolon_token: Token,
    trivia: Vec<Box<dyn Trivia>>,
}