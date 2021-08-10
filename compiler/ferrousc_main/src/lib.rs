use ferrousc_lexer::tokenize;

pub fn run() {
    for token in tokenize(r#"
        fn test(nica_dicy: i32, _char: char) {
            // test
            call_function(52.2);
        }
    "#) {
        println!("{:?}", token);
    }
}