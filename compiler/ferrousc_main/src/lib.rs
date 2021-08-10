use ferrousc_lexer::tokenize;

pub fn run() {
    for token in tokenize(r#" ** _ '' 'd' '\'' '\t' '  6_32.36.123 _hallo_welt 0b01014 0xaAfe2 0o259 "\" Test String!234632"       * / */ ++ += - == % |||"#) {
        println!("{:?}", token);
    }
}