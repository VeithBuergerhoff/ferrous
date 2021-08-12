mod parser;
mod cursor;

use ferrousc_ast::nodes::*;
use ferrousc_lexer::Token;
use parser::Parser;

pub fn generate_ast(tokens: impl Iterator<Item = Token>) -> CompilationUnit {
    /*let let_token = tokens.next().unwrap();
    let mut_token = tokens.next();
    let identifier = tokens.next().unwrap();
    let identifier = Identifier{identifier_token: identifier};
    let type_id: Option<TypeId> = None;
    let initial_value: Option<EqualsValue> = None;
    let semicolon_token = tokens.next().unwrap();*/

    let mut parser = Parser::new(tokens);
    parser.parse()
   // VariableDefinitionStatement { let_token, mut_token, identifier, type_id, initial_value, semicolon_token}
}