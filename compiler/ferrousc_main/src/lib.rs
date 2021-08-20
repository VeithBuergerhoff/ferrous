use ferrousc_ast::nodes::{Expr, Stat, SyntaxToken};
use ferrousc_lexer::tokenize;
use ferrousc_parser::generate_ast;

const TEST_CODE: &str = r#"
let mut test: aha = 525.52;
let test2: aha;
{}

if 2 {

}
else {

}

{
    let test = 2;
    {
        let test = 56;
    }
}
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
        walk(st, 0);
    })
}

fn walk(st: &Stat, tab_index: i32) {
    match &*st {
        Stat::VarDefinition{ 
            let_token, 
            mut_token, 
            identifier, 
            type_id, 
            initial_value, 
            semicolon_token
        } => {
            indent_n(tab_index);
            println!("Var Definition {{");
            indent_n(tab_index + 1);
            println!("let: {:?},", let_token);
            indent_n(tab_index + 1);
            println!("mut: {:?},", mut_token);
            indent_n(tab_index + 1);
            println!("identifier: {:?},", identifier);
            if type_id.is_some() {
                indent_n(tab_index + 1);
                println!("type_id: {{");
                let id = type_id.as_ref().unwrap();
                indent_n(tab_index + 2);
                println!("colon: {:?},", id.colon_token);
                indent_n(tab_index + 2);
                println!("typename: {:?},", id.type_name);
                indent_n(tab_index + 1);
                println!("}}");
            }
            else {
                indent_n(tab_index + 1);
                println!("type_id: none");
            }
            indent_n(tab_index + 1);
            println!("initial_value: {:?},", initial_value);
            indent_n(tab_index + 1);
            println!("semicolon: {:?}", semicolon_token);
            indent_n(tab_index);
            println!("}}");
        },
        Stat::Block{
            l_brace, 
            statements, 
            r_brace
        } => {
            indent_n(tab_index);
            println!("Block Statement {{");
            indent_n(tab_index + 1);
            println!("l_brace: {:?},", l_brace);
            for st in statements {
                walk(st, tab_index + 1);
            }
            indent_n(tab_index + 1);
            println!("r_brace: {:?},", r_brace);
            indent_n(tab_index);
            println!("}}");
        }
        Stat::If{
            if_token, 
            expression,
            statement, 
            else_statement
        } => {
            indent_n(tab_index);
            println!("If Statement {{");
            indent_n(tab_index + 1);
            println!("if_token: {:?},", if_token);
            indent_n(tab_index + 1);
            println!("expression: {{");
            print_expression(expression, tab_index + 2);
            indent_n(tab_index + 1);
            println!("}}");
            indent_n(tab_index + 1);
            println!("Statement: {{");
            walk(statement, tab_index + 2);
            indent_n(tab_index + 1);
            println!("}}");

            if else_statement.is_some() {
                if let Stat::Else{else_token, statement} = else_statement.as_ref().as_ref().unwrap() {
                    indent_n(tab_index + 1);
                    println!("else: {{");
                    indent_n(tab_index + 2);
                    println!("else_token: {:?},", else_token);
                    indent_n(tab_index + 2);
                    println!("Statement: {{");
                    walk(statement.as_ref(), tab_index + 3);
                    indent_n(tab_index + 2);
                    println!("}}");
                }
            }
            else {
                indent_n(tab_index + 1);
                println!("else: none");
            }

            indent_n(tab_index + 1);
            println!("}}");

            indent_n(tab_index);
            println!("}}");
        }
        #[allow(unreachable_patterns)]
        _ => println!("unknown statement!"),
    }
}

fn print_expression(expr: &Expr, tab_index: i32) {
    indent_n(tab_index);
    println!("{:?}", expr);
}

fn indent_n(tab_index: i32) {
    for _ in 0..tab_index {
        indent();
    }
}

fn indent() {
    print!("  ");
}