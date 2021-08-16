use ferrousc_lexer::tokenize;
use ferrousc_parser::generate_ast;

pub fn run() {
    let test_src = r#"
        let mut test: aha;
    "#;

    for token in tokenize(test_src) {
        println!("{:?}", token);
    }

    println!("{:?}", generate_ast(tokenize(test_src)));
}