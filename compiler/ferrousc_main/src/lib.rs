use ferrousc_ast::nodes::{Expr, FunctionBody, Stat};
use ferrousc_lexer::tokenize;
use ferrousc_parser::generate_ast;

const TEST_CODE: &str = r#"
if 1 < 2 {
    test = 53;
}

if !true == false {

}

if "string".2 > 3 {

}

if "test"[5] < 8 {

}

for n in 0..5 {

}

let mut test1: aha = 525.52;
let test2 = "test string";
let test3 = 'c';
let test4 = false;
{}

for v in 25 {

}

fn test(a: bool, b: bool) {
    return 5;
}

fn test(a: bool, b: bool) => return 5;

fn are_equal(a: bool, b: bool) -> bool {
    return false;
}

while true {

}

if 2 {

}
else if 5 {
}
else {
    return;
    return 5;
    break;
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
        println!();
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
                println!("typename: {:?},", id.type_kind);
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
        },
        Stat::Break {
            break_token, 
            semicolon_token,
        } => {
            indent_n(tab_index);
            println!("Break Statement {{");
            indent_n(tab_index + 1);
            println!("break_token: {:?},", break_token);
            indent_n(tab_index + 1);
            println!("semicolon_token: {:?},", semicolon_token);
            indent_n(tab_index);
            println!("}}");
        },
        Stat::FunctionDefinition {
            fn_token, 
            identifier, 
            parameter_list,
            return_type,
            body,
        } => {
            indent_n(tab_index);
            println!("Function Definition Statement {{");
            indent_n(tab_index + 1);
            println!("fn_token: {:?},", fn_token);
            indent_n(tab_index + 1);
            println!("identifier: {:?},", identifier);

            indent_n(tab_index + 1);
            println!("parameter_list: {{");
            indent_n(tab_index + 2);
            println!("l_paran: {:?}", parameter_list.l_paran);
            
            for parameter in &parameter_list.parameters {
                indent_n(tab_index + 2);
                println!("parameter: {{");

                indent_n(tab_index + 3);
                println!("identifier: {:?}", parameter.identifier);
                indent_n(tab_index + 3);
                println!("type_id: {:?}", parameter.type_id);
                indent_n(tab_index + 3);
                println!("comma_token: {:?}", parameter.comma_token);

                indent_n(tab_index + 2);
                println!("}}");
            }
            
            println!("r_paran: {:?}", parameter_list.r_paran);
            
            indent_n(tab_index + 1);
            println!("}}");

            indent_n(tab_index + 1);
            if return_type.is_some() {
                let return_type = return_type.as_ref().unwrap();
                println!("return_type: {{");
            
                indent_n(tab_index + 2);
                println!("small_arrow_token: {:?}", return_type.small_arrow_token);

                indent_n(tab_index + 2);
                println!("type_kind: {:?}", return_type.type_kind);
    
                indent_n(tab_index + 1);
                println!("}}");
            }
            else {
                println!("return_type: none");
            }
           
            indent_n(tab_index + 1);
            println!("body: {{");
            match body.as_ref() {
                FunctionBody::BlockStatement{ block } => {
                    walk(block, tab_index + 2);
                },
                FunctionBody::ExpressionBody{ fat_arrow_token, statement } => {
                    indent_n(tab_index + 2);
                    println!("expression_body: {{");

                    indent_n(tab_index + 3);
                    println!("fat_arrow_token: {:?}", fat_arrow_token);

                    indent_n(tab_index + 3);
                    println!("statement: {{");

                    walk(statement, tab_index + 4);

                    indent_n(tab_index + 3);
                    println!("}}");

                    indent_n(tab_index + 2);
                    println!("}}");
                },
            }

            indent_n(tab_index + 1);
            println!("}}");

            indent_n(tab_index);
            println!("}}");
        },
        Stat::Return {
            return_token, 
            expression, 
            semicolon_token,
        } => {
            indent_n(tab_index);
            println!("Return Statement {{");
            indent_n(tab_index + 1);
            println!("return_token: {:?},", return_token);
            indent_n(tab_index + 1);
            if expression.is_some() {
                println!("expression: {{");
                print_expression(expression.as_ref().unwrap(), tab_index + 1);
                indent_n(tab_index + 1);
                println!("}}");
            }
            else {
                println!("expression: none,");
            }
            indent_n(tab_index + 1);
            println!("semicolon_token: {:?},", semicolon_token);
            indent_n(tab_index);
            println!("}}");
        },
        Stat::For{
            for_token, 
            identifier,
            in_token,
            range,
            statement,
        } => {
            indent_n(tab_index);
            println!("For Statement {{");

            indent_n(tab_index + 1);
            println!("for_token: {:?},", for_token);
            indent_n(tab_index + 1);
            println!("identifier: {:?},", identifier);
            indent_n(tab_index + 1);
            println!("in_token: {:?},", in_token);

            indent_n(tab_index + 1);
            println!("expression: {{");
            print_expression(range, tab_index + 2);
            indent_n(tab_index + 1);
            println!("}}");

            indent_n(tab_index + 1);
            println!("Statement: {{");
            walk(statement, tab_index + 2);
            indent_n(tab_index + 1);
            println!("}}");

            indent_n(tab_index);
            println!("}}");
        },
        Stat::While{
            while_token, 
            expression,
            statement,
        } => {
            indent_n(tab_index);
            println!("While Statement {{");
            indent_n(tab_index + 1);
            println!("while_token: {:?},", while_token);
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
            indent_n(tab_index);
            println!("}}");
        },
        Stat::Expr{
            expr, 
            semicolon_token,
        } => {
            indent_n(tab_index);
            println!("Expression Statement {{");

            indent_n(tab_index + 1);
            println!("expression: {{");
            print_expression(expr, tab_index + 2);
            indent_n(tab_index + 1);
            println!("}}");
            
            indent_n(tab_index + 1);
            println!("semicolon_token: {:?}", semicolon_token);
            indent_n(tab_index);
            println!("}}");
        },
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
                if let Stat::Else{else_token, statement} = else_statement.as_ref().unwrap().as_ref() {
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
        },
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