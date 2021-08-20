use ferrousc_ast::nodes::Stat;
use ferrousc_lexer::tokenize;
use ferrousc_parser::generate_ast;

const TEST_CODE: &str = r#"
let mut test: aha;
"#;


pub fn run() {
    generate_ast(tokenize(TEST_CODE));
}

pub fn print() {
    println!();

    for token in tokenize(TEST_CODE) {
        println!("{:?}", token);
    }
    println!();
    
    let ast = generate_ast(tokenize(TEST_CODE));

    println!("{:?}", ast);
    println!();

    ast.walk(|st|{
        match &*st {
            Stat::VarDefinition{ 
                let_token, 
                mut_token, 
                identifier, 
                type_id, 
                initial_value, 
                semicolon_token
            } => {
                println!("Var Definition {{");
                println!("\tlet: {:?},", let_token);
                println!("\tmut: {:?},", mut_token);
                println!("\tidentifier: {:?},", identifier);
                if type_id.is_some() {
                    println!("\ttype_id: {{");
                    let id = type_id.as_ref().unwrap();
                    println!("\t\tcolon: {:?},", id.colon_token);
                    println!("\t\ttypename: {:?},", id.type_name);
                    println!("\t}}");
                }
                else {
                    println!("\ttype_id: none");
                }
                println!("\tinitial_value: {:?},", initial_value);
                println!("\tsemicolon: {:?}", semicolon_token);
            },
            _ => println!("unknown statement!"),
        }
    })
}