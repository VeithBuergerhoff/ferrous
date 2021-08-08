use std::str::Chars;

pub const EOF_CHAR: char = '\0';

pub(crate) struct Cursor<'a> {
    chars: Chars<'a>
}

impl Cursor<'_> {
    pub(crate) fn new(src: &'_ str) -> Cursor<'_> {
        Cursor { chars: src.chars() }
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub(crate) fn peek(&self) -> char {
        self.chars().nth(0).unwrap_or(EOF_CHAR)
    }

    pub(crate) fn peekn(&self, n: usize) -> char {
        self.chars().nth(n).unwrap_or(EOF_CHAR)
    }

    pub(crate) fn eat(&mut self) -> char {
        self.chars.next().unwrap_or(EOF_CHAR)
    }

    fn chars(&self) -> Chars<'_> {
        self.chars.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_eof() {
        let mut c = Cursor::new("1234\05");
        assert_eq!(c.eat(), '1');
        assert_eq!(c.eat(), '2');
        assert_eq!(c.eat(), '3');
        assert_eq!(c.eat(), '4');
        assert_eq!(c.eat(), EOF_CHAR);
        assert_eq!(c.is_eof(), false);
        assert_eq!(c.eat(), '5');
        assert_eq!(c.eat(), EOF_CHAR);
        assert_eq!(c.is_eof(), true);
        assert_eq!(c.eat(), EOF_CHAR);
        assert_eq!(c.is_eof(), true);
    }

    #[test]
    fn test_eat() {
        let mut c = Cursor::new("1234");
        assert_eq!(c.eat(), '1');
        assert_eq!(c.eat(), '2');
        assert_eq!(c.eat(), '3');
        assert_eq!(c.eat(), '4');
        assert_eq!(c.eat(), EOF_CHAR);
        assert_eq!(c.eat(), EOF_CHAR);
    }

    #[test]
    fn test_peek() {
        let c = Cursor::new("1234");
        assert_eq!(c.peek(), '1');
        assert_eq!(c.peek(), '1');
        let mut c = c;
        c.eat();
        let c = c;
        assert_eq!(c.peek(), '2');
        assert_eq!(c.peek(), '2');
    }

    #[test]
    fn test_peekn() {
        let c = Cursor::new("1234");
        assert_eq!(c.peekn(0), '1');
        assert_eq!(c.peekn(1), '2');
        let mut c = c;
        c.eat();
        let c = c;
        assert_eq!(c.peekn(1), '3');
        assert_eq!(c.peekn(2), '4');
        assert_eq!(c.peekn(3), EOF_CHAR);
    }

    #[test]
    fn test_peek_end() {
        let mut c = Cursor::new("1234");
        c.eat();
        c.eat();
        c.eat();
        c.eat();
        let c = c;
        assert_eq!(c.peek(), EOF_CHAR);
        assert_eq!(c.peek(), EOF_CHAR);
        let mut c = c;
        c.eat();
        let c = c;
        assert_eq!(c.peek(), EOF_CHAR);
    }
}