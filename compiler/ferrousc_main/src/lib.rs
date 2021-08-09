use ferrousc_lexer::tokenize;

pub fn run() {
    for token in tokenize(r#" ** _  632.36.123 _hallo_welt "\" Test String!234632"       * / */ ++ += - == % |||"#) {
        println!("{:?}", token);
    }
}