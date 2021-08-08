use ferrousc_lexer::tokenize;

pub fn run() {
    for token in tokenize(" **\t         * / */ ++ += - == % |||") {
        println!("{:?}", token);
    }
}