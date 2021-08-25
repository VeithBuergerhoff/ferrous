use expect_test::{Expect, expect};

use super::*;

fn check_lexing(src: &str, expect: Expect) {
    let actual: String = tokenize(src).map(|token| format!("{:?}\n", token)).collect();
    expect.assert_eq(&actual)
}

#[test]
fn identifier() {
    check_lexing(
        r"
_test test 1 _1 __1 a-ha a_ha äòtest testäù 幸験test TEST TestTest
",
        expect![[r#"
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