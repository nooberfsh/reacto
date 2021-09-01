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
            let span = self.span().unwrap();
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

    fn peek2(&self) -> (Option<char>, Option<char>) {
        self.ctx().peek2()
    }

    fn advance(&mut self) -> Option<char> {
        self.ctx_mut().advance()
    }

    fn advance_cmp(&mut self, c: char) -> bool {
        self.ctx_mut().advance_cmp(c)
    }

    fn advance_cmp2(&mut self, c1: char, c2: char) -> bool {
        self.ctx_mut().advance_cmp2(c1, c2)
    }

    fn advance_to(&mut self, c: char) -> bool {
        self.ctx_mut().advance_to(c)
    }

    fn advance_after(&mut self, c: char) -> bool {
        self.ctx_mut().advance_after(c)
    }

    fn advance_if(&mut self, p: impl Fn(char) -> bool) -> bool {
        self.ctx_mut().advance_if(p)
    }

    fn advance_while(&mut self, p: impl Fn(char) -> bool) -> usize {
        self.ctx_mut().advance_while(p)
    }

    fn span(&self) -> Option<Span> {
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

    fn peek2(&self) -> (Option<char>, Option<char>) {
        let a = self.peek();
        let b = if self.cursor < self.chars.len() - 1 {
            Some(self.chars[self.cursor + 1])
        } else {
            None
        };
        (a, b)
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

    fn advance_cmp2(&mut self, c1: char, c2: char) -> bool {
        let (a1, a2) = self.peek2();
        if a1 == Some(c1) && a2 == Some(c2) {
            self.advance();
            self.advance();
            true
        } else {
            false
        }
    }

    fn advance_to(&mut self, c: char) -> bool {
        while self.peek() != Some(c) {
            if self.eof() {
                return false
            }
            self.advance();
        }
        true
    }

    fn advance_after(&mut self, c: char) -> bool {
        if self.advance_to(c) {
            self.advance();
            true
        } else {
            false
        }
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

    fn span(&self) -> Option<Span> {
        let start = self.start;
        let end = self.cursor;
        if start == end {
            None
        } else {
            Some(Span::new(start, end))
        }
    }

    fn sync(&mut self) {
        self.start = self.cursor
    }

    fn chars(&self) -> &Chars {
        &self.chars
    }
}
