mod parser;

use ferrousc_ast::nodes::*;
use ferrousc_lexer::Token;
use parser::Parser;

pub fn generate_ast(tokens: impl Iterator<Item = Token>) -> CompilationUnit {
    let mut parser = Parser::new(tokens);
    parser.parse()
}