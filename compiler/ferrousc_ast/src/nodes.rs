use ferrousc_ast_proc_macros::*;

use ferrousc_lexer::Token;
use std::fmt;

pub trait Node {}

pub trait Trivia : Node {
    fn get_value(&self) -> &Token;
}

impl fmt::Debug for dyn Trivia {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Trivia {{ token: {:?} }}", self.get_value())
    }
}

pub trait SyntaxNode : Node {}

pub trait Expression : SyntaxNode {}

impl fmt::Debug for dyn Expression {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Expression")
    }
}

pub trait Statement : SyntaxNode  {}

impl fmt::Debug for dyn Statement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Statement")
    }
}

#[node]
#[triviated]
#[derive(Debug)]
pub struct SyntaxToken {
    pub token: Token,
    pub trivia: Vec<Box<dyn Trivia>>,
}

#[trivia]
#[derive(Debug)]
pub struct WhitespaceTrivia {
    pub trivia_token: Token,
}

#[trivia]
#[derive(Debug)]
pub struct NewlineTrivia {
    pub trivia_token: Token,
}

#[trivia]
#[derive(Debug)]
pub struct LineCommentTrivia {
    pub trivia_token: Token,
}

#[trivia]
#[derive(Debug)]
pub struct MultilineCommentTrivia {
    pub trivia_token: Token,
}

#[node]
#[derive(Debug)]
pub struct Identifier {
    pub identifier_token: SyntaxToken,
}

#[node]
#[derive(Debug)]
pub struct TypeId {
    pub colon_token: SyntaxToken,
    pub type_name: Identifier,
}

#[node]
#[derive(Debug)]
pub struct EqualsValue {
    pub equals_token: SyntaxToken,
    pub expression: Box<dyn Expression>,
}

#[node]
#[triviated]
#[derive(Debug)]
pub struct CompilationUnit {
    pub statements: Vec<Box<dyn Statement>>,
    pub trivia: Vec<Box<dyn Trivia>>,
}

// expressions

#[node]
#[expression]
pub struct NumberLiteralExpression {
    pub literal_token: SyntaxToken,
}

// statements

#[statement]
#[derive(Debug)]
pub struct VariableDefinitionStatement {
    pub let_token: SyntaxToken,
    pub mut_token: Option<SyntaxToken>,
    pub identifier: Identifier,
    pub type_id: Option<TypeId>,
    pub initial_value: Option<EqualsValue>,
    pub semicolon_token: SyntaxToken,
}