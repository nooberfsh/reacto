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
