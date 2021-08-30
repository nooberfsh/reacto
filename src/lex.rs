use crate::chars::Chars;
use crate::span::{Span, S};

#[derive(Debug, Clone)]
pub struct LexCtx {
    chars: Chars,
    cursor: usize,
    start: usize,
}

impl LexCtx {
    pub fn new(input: &str) -> LexCtx {
        let chars = Chars::new(input);
        LexCtx {
            chars,
            cursor: 0,
            start: 0,
        }
    }
}

pub trait Lex {
    type Token;
    type Error;

    // required

    fn ctx(&self) -> &LexCtx;
    fn ctx_mut(&mut self) -> &mut LexCtx;
    fn next(&mut self) -> Result<Option<Self::Token>, Self::Error>;

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // provided

    fn next_s(&mut self) -> Result<Option<S<Self::Token>>, Self::Error> {
        if let Some(tok) = self.next()? {
            let span = self.span();
            let tok = S { span, tok };
            self.ctx_mut().sync();
            Ok(Some(tok))
        } else {
            Ok(None)
        }
    }

    fn tokens(&mut self) -> Result<Vec<S<Self::Token>>, Self::Error> {
        let mut ret = vec![];
        while let Some(tok) = self.next_s()? {
            ret.push(tok);
        }
        Ok(ret)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // provided, delegate to LexCtx

    fn eof(&self) -> bool {
        self.ctx().eof()
    }

    fn peek(&self) -> Option<char> {
        self.ctx().peek()
    }

    fn advance(&mut self) -> Option<char> {
        self.ctx_mut().advance()
    }

    fn advance_cmp(&mut self, c: char) -> bool {
        self.ctx_mut().advance_cmp(c)
    }

    fn advance_if(&mut self, p: impl Fn(char) -> bool) -> bool {
        self.ctx_mut().advance_if(p)
    }

    fn advance_while(&mut self, p: impl Fn(char) -> bool) -> usize {
        self.ctx_mut().advance_while(p)
    }

    fn span(&self) -> Span {
        self.ctx().span()
    }

    fn chars(&self) -> &Chars {
        self.ctx().chars()
    }
}

impl LexCtx {
    fn eof(&self) -> bool {
        self.cursor == self.chars.len()
    }

    fn peek(&self) -> Option<char> {
        if self.eof() {
            None
        } else {
            Some(self.chars[self.cursor])
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.eof() {
            None
        } else {
            let c = self.chars[self.cursor];
            self.cursor += 1;
            Some(c)
        }
    }

    fn advance_cmp(&mut self, c: char) -> bool {
        self.advance_if(|x| x == c)
    }

    fn advance_if(&mut self, p: impl Fn(char) -> bool) -> bool {
        if let Some(c) = self.peek() {
            if p(c) {
                self.cursor += 1;
                return true;
            }
        }
        false
    }

    fn advance_while(&mut self, p: impl Fn(char) -> bool) -> usize {
        let mut num = 0;
        while let Some(c) = self.peek() {
            if !p(c) {
                break;
            }
            self.cursor += 1;
            num += 1;
        }
        num
    }

    fn span(&self) -> Span {
        let start = self.start;
        let end = self.cursor;
        Span::new(start, end)
    }

    fn sync(&mut self) {
        self.start = self.cursor
    }

    fn chars(&self) -> &Chars {
        &self.chars
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    pub struct LexError {
        span: Span,
    }

    #[derive(Clone, Debug)]
    pub struct Lexer {
        ctx: LexCtx,
    }

    #[derive(Clone, Debug, Copy, Eq, PartialEq)]
    pub enum Token {
        Plus,
        Whitespace,
        Ident,
        LitString,
    }

    impl Lex for Lexer {
        type Token = Token;
        type Error = String;

        fn ctx(&self) -> &LexCtx {
            &self.ctx
        }

        fn ctx_mut(&mut self) -> &mut LexCtx {
            &mut self.ctx
        }

        fn next(&mut self) -> Result<Option<Token>, String> {
            let c = match self.advance() {
                Some(d) => d,
                None => return Ok(None),
            };

            let ty = match c {
                '+' => Token::Plus,
                ' ' => Token::Whitespace,
                '"' => {
                    self.advance_while(|c| c != '"');
                    if !self.advance_cmp('"') {
                        return Err("sting not closed".to_string())
                    }
                    Token::LitString
                }
                c if is_letter(c) => {
                    self.advance_while(is_digit_letter);
                    Token::Ident
                }
                _ => return Err("unknown char".to_string())
            };
            Ok(Some(ty))
        }
    }

    fn new_lexer(s: &str) -> Lexer {
        let ctx = LexCtx::new(s);
        Lexer {ctx}
    }

    #[test]
    fn test_move() {
        let mut a = new_lexer("ab");
        assert_eq!(a.eof(), false);
        assert_eq!(a.peek().unwrap(), 'a');
        assert_eq!(a.advance().unwrap(), 'a');
        assert_eq!(a.peek().unwrap(), 'b');
        assert_eq!(a.advance().unwrap(), 'b');
        assert_eq!(a.eof(), true);
        assert_eq!(a.peek(), None);
        assert_eq!(a.advance(), None);
    }

    #[test]
    fn test_advance_cmp() {
        let mut a = new_lexer("ab");
        assert_eq!(a.advance_cmp('b'), false);
        assert_eq!(a.peek().unwrap(), 'a');

        assert_eq!(a.advance_cmp('a'), true);
        assert_eq!(a.peek().unwrap(), 'b');

        assert_eq!(a.advance_cmp('b'), true);
        assert_eq!(a.peek(), None);

        assert_eq!(a.advance_cmp('b'), false);
        assert_eq!(a.peek(), None);
    }

    #[test]
    fn test_advance_if() {
        let mut a = new_lexer("ab");
        assert_eq!(a.advance_if(|c| c == 'b'), false);
        assert_eq!(a.peek().unwrap(), 'a');

        assert_eq!(a.advance_if(|c| c == 'a'), true);
        assert_eq!(a.peek().unwrap(), 'b');

        assert_eq!(a.advance_if(|c| c == 'b'), true);
        assert_eq!(a.peek(), None);

        assert_eq!(a.advance_if(|c| c == 'b'), false);
        assert_eq!(a.peek(), None);
    }

    #[test]
    fn test_advance_while() {
        let mut a = new_lexer("123ab");
        assert_eq!(a.advance_while(|c| is_digit(c)), 3);
        assert_eq!(a.advance_while(|c| is_letter(c)), 2);
        assert_eq!(a.advance_while(|c| is_letter(c)), 0);
    }

    #[test]
    fn test_span() {
        let mut a = new_lexer("+ ab");
        assert_eq!(Span::new(0,0), a.span());
        let res = a.next_s().unwrap().unwrap();
        assert_eq!(res.span, Span::new(0, 1));
        assert_eq!(res.tok, Token::Plus);

        let res = a.next_s().unwrap().unwrap();
        assert_eq!(res.span, Span::new(1, 2));
        assert_eq!(res.tok, Token::Whitespace);

        let res = a.next_s().unwrap().unwrap();
        assert_eq!(res.span, Span::new(2, 4));
        assert_eq!(res.tok, Token::Ident);

        let res = a.next_s().unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn test_chars() {
        let a = new_lexer("+ ab");
        assert_eq!(a.chars(), "+ ab")
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////
    // helper functions
    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_letter(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_')
    }

    fn is_digit_letter(c: char) -> bool {
        is_digit(c) || is_letter(c)
    }
}
