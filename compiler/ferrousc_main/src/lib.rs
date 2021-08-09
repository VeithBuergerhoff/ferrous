use ferrousc_lexer::tokenize;

pub fn run() {
    for token in tokenize(r#" ** _  6_32.36.123 _hallo_welt "\" Test String!234632"       * / */ ++ += - == % |||"#) {
        println!("{:?}", token);
    }
}