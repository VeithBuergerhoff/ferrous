use expect_test::{Expect, expect};

use super::*;

fn check_lexing(src: &str, expect: Expect) {
    let actual: String = tokenize(src).map(|token| format!("{:?}\n", token)).collect();
    expect.assert_eq(&actual)
}

#[test]
fn smoke_test() {
    check_lexing(
        r"
// This is some test code
/*
    It contains stuff
*/
fn maybe_main(i: int) -> bool {
    while true 
        if is_nice {
            return true;
        }
        else {
            return false;
        }
}
",
        expect![[r#"
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: LineComment, value: "// This is some test code", len: 25 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: MultilineComment { terminated: true }, value: "/*\n    It contains stuff\n*/", len: 27 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: FunctionKeyword, value: "fn", len: 2 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: Identifier, value: "maybe_main", len: 10 }
            Token { kind: LParen, value: "(", len: 1 }
            Token { kind: Identifier, value: "i", len: 1 }
            Token { kind: Colon, value: ":", len: 1 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: Identifier, value: "int", len: 3 }
            Token { kind: RParen, value: ")", len: 1 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: MinusGreater, value: "->", len: 2 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: Identifier, value: "bool", len: 4 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: LBrace, value: "{", len: 1 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: Whitespace, value: "    ", len: 4 }
            Token { kind: WhileKeyword, value: "while", len: 5 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: TrueKeyword, value: "true", len: 4 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: Whitespace, value: "        ", len: 8 }
            Token { kind: IfKeyword, value: "if", len: 2 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: Identifier, value: "is_nice", len: 7 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: LBrace, value: "{", len: 1 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: Whitespace, value: "            ", len: 12 }
            Token { kind: ReturnKeyword, value: "return", len: 6 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: TrueKeyword, value: "true", len: 4 }
            Token { kind: Semicolon, value: ";", len: 1 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: Whitespace, value: "        ", len: 8 }
            Token { kind: RBrace, value: "}", len: 1 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: Whitespace, value: "        ", len: 8 }
            Token { kind: ElseKeyword, value: "else", len: 4 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: LBrace, value: "{", len: 1 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: Whitespace, value: "            ", len: 12 }
            Token { kind: ReturnKeyword, value: "return", len: 6 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: FalseKeyword, value: "false", len: 5 }
            Token { kind: Semicolon, value: ";", len: 1 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: Whitespace, value: "        ", len: 8 }
            Token { kind: RBrace, value: "}", len: 1 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: RBrace, value: "}", len: 1 }
            Token { kind: Newline, value: "\n", len: 1 }
        "#]],
    )
}

#[test]
fn operators() {
    check_lexing(
        "()[]+++---!%&|<",
        expect![[r#"
            Token { kind: LParen, value: "(", len: 1 }
            Token { kind: RParen, value: ")", len: 1 }
            Token { kind: LBracket, value: "[", len: 1 }
            Token { kind: RBracket, value: "]", len: 1 }
            Token { kind: PlusPlus, value: "++", len: 2 }
            Token { kind: Plus, value: "+", len: 1 }
            Token { kind: MinusMinus, value: "--", len: 2 }
            Token { kind: Minus, value: "-", len: 1 }
            Token { kind: Bang, value: "!", len: 1 }
            Token { kind: Percent, value: "%", len: 1 }
            Token { kind: Amp, value: "&", len: 1 }
            Token { kind: Bar, value: "|", len: 1 }
            Token { kind: Less, value: "<", len: 1 }
        "#]],
    )
}

#[test]
fn numbers() {
    check_lexing(
        r#"
123 5. .5 1232.25 0b10_14 0o17_29 0xaf_Fah 0x 0x_ 0b 0b_ 0o 0o_ 0b_1 0b1_ 0..5
"#,
        expect![[r#"
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: NumberLiteral { base: Decimal, has_digits: true }, value: "123", len: 3 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: NumberLiteral { base: Decimal, has_digits: true }, value: "5", len: 1 }
            Token { kind: Dot, value: ".", len: 1 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: Dot, value: ".", len: 1 }
            Token { kind: NumberLiteral { base: Decimal, has_digits: true }, value: "5", len: 1 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: NumberLiteral { base: Decimal, has_digits: true }, value: "1232.25", len: 7 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: NumberLiteral { base: Binary, has_digits: true }, value: "0b10_1", len: 6 }
            Token { kind: NumberLiteral { base: Decimal, has_digits: true }, value: "4", len: 1 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: NumberLiteral { base: Octal, has_digits: true }, value: "0o17_2", len: 6 }
            Token { kind: NumberLiteral { base: Decimal, has_digits: true }, value: "9", len: 1 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: NumberLiteral { base: Hexadecimal, has_digits: true }, value: "0xaf_Fa", len: 7 }
            Token { kind: Identifier, value: "h", len: 1 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: NumberLiteral { base: Hexadecimal, has_digits: false }, value: "0x", len: 2 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: NumberLiteral { base: Hexadecimal, has_digits: false }, value: "0x_", len: 3 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: NumberLiteral { base: Binary, has_digits: false }, value: "0b", len: 2 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: NumberLiteral { base: Binary, has_digits: false }, value: "0b_", len: 3 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: NumberLiteral { base: Octal, has_digits: false }, value: "0o", len: 2 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: NumberLiteral { base: Octal, has_digits: false }, value: "0o_", len: 3 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: NumberLiteral { base: Binary, has_digits: true }, value: "0b_1", len: 4 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: NumberLiteral { base: Binary, has_digits: true }, value: "0b1_", len: 4 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: NumberLiteral { base: Decimal, has_digits: true }, value: "0", len: 1 }
            Token { kind: DotDot, value: "..", len: 2 }
            Token { kind: NumberLiteral { base: Decimal, has_digits: true }, value: "5", len: 1 }
            Token { kind: Newline, value: "\n", len: 1 }
        "#]],
    )
}

#[test]
fn strings() {
    check_lexing(
        r#"
"string" "string with spaces" "string with a '\"'" "string with a newline
ends here" "testäù 幸験test"
"#,
        expect![[r#"
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: StringLiteral { terminated: true }, value: "\"string\"", len: 8 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: StringLiteral { terminated: true }, value: "\"string with spaces\"", len: 20 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: StringLiteral { terminated: true }, value: "\"string with a '\\\"'\"", len: 20 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: StringLiteral { terminated: true }, value: "\"string with a newline\nends here\"", len: 33 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: StringLiteral { terminated: true }, value: "\"testäù 幸験test\"", len: 14 }
            Token { kind: Newline, value: "\n", len: 1 }
        "#]],
    )
}

#[test]
fn chars() {
    check_lexing(
        r#"
's' '\n' '\x' 'too long' '\'' '"'
"#,
        expect![[r#"
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: CharLiteral { terminated: true }, value: "'s'", len: 3 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: CharLiteral { terminated: true }, value: "'\n'", len: 4 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: CharLiteral { terminated: true }, value: "'\x'", len: 4 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: CharLiteral { terminated: false }, value: "'too long'", len: 10 }
            Token { kind: CharLiteral { terminated: true }, value: "'\\''", len: 4 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: CharLiteral { terminated: true }, value: "'\"'", len: 3 }
            Token { kind: Newline, value: "\n", len: 1 }
        "#]],
    )
}

#[test]
fn identifier() {
    check_lexing(
        r"
_test test 1 _1 __1 a-ha a_ha äòtest testäù 幸験test TEST TestTest
",
        expect![[r#"
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: Identifier, value: "_test", len: 5 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: Identifier, value: "test", len: 4 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: NumberLiteral { base: Decimal, has_digits: true }, value: "1", len: 1 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: Identifier, value: "_1", len: 2 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: Identifier, value: "__1", len: 3 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: Identifier, value: "a", len: 1 }
            Token { kind: Minus, value: "-", len: 1 }
            Token { kind: Identifier, value: "ha", len: 2 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: Identifier, value: "a_ha", len: 4 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: Identifier, value: "äòtest", len: 6 }
            Token { kind: Identifier, value: "testäù", len: 6 }
            Token { kind: Identifier, value: "幸験test", len: 6 }
            Token { kind: Identifier, value: "st", len: 2 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: Identifier, value: "TEST", len: 4 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: Identifier, value: "TestTest", len: 8 }
            Token { kind: Newline, value: "\n", len: 1 }
        "#]],
    )
}

#[test]
fn comments() {
    check_lexing(
        r"
// line
/// line too
//// line also
//! still a line
//* also a line
//* not affected by */
// also not affected by */
/* block */
/** also block */
/*** also also block */
/*** also also also block ***/
/*! also also also block */
/**/
/***/
/* /* can also be nested */ */
/* 
    block multiline
*/
/* not terminated! // is a line comment
",
        expect![[r#"
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: LineComment, value: "// line", len: 7 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: LineComment, value: "/// line too", len: 12 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: LineComment, value: "//// line also", len: 14 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: LineComment, value: "//! still a line", len: 16 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: LineComment, value: "//* also a line", len: 15 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: LineComment, value: "//* not affected by */", len: 22 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: LineComment, value: "// also not affected by */", len: 26 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: MultilineComment { terminated: true }, value: "/* block */", len: 11 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: MultilineComment { terminated: true }, value: "/** also block */", len: 17 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: MultilineComment { terminated: true }, value: "/*** also also block */", len: 23 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: MultilineComment { terminated: true }, value: "/*** also also also block ***/", len: 30 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: MultilineComment { terminated: true }, value: "/*! also also also block */", len: 27 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: MultilineComment { terminated: true }, value: "/**/", len: 4 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: MultilineComment { terminated: true }, value: "/***/", len: 5 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: MultilineComment { terminated: true }, value: "/* /* can also be nested */", len: 27 }
            Token { kind: Whitespace, value: " ", len: 1 }
            Token { kind: Star, value: "*", len: 1 }
            Token { kind: Slash, value: "/", len: 1 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: MultilineComment { terminated: true }, value: "/* \n    block multiline\n*/", len: 26 }
            Token { kind: Newline, value: "\n", len: 1 }
            Token { kind: MultilineComment { terminated: false }, value: "/* not terminated! // is a line comment\n", len: 40 }
        "#]],
    )
}