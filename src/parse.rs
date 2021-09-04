use crate::ast::N;
use crate::chars::Chars;
use crate::node_id::IdGen;
use crate::span::{Span, S};

#[derive(Clone, Debug)]
pub struct ParseCtx<T> {
    chars: Chars,
    tokens: Vec<S<T>>,
    id_gen: IdGen,
    // state
    call_stack: Vec<usize>,
    cursor: usize,
}

impl<T> ParseCtx<T> {
    pub fn new(chars: Chars, tokens: Vec<S<T>>) -> Self {
        ParseCtx {
            chars,
            tokens,
            id_gen: IdGen::new(),
            call_stack: vec![],
            cursor: 0,
        }
    }
}

#[macro_export]
macro_rules! parse_some {
    ($parser:expr, $f:ident, $sep:expr) => {{
        let head = $parser.$f()?;
        let mut ret = vec![head];
        while $parser.advance_cmp($sep) {
            let d = $parser.$f()?;
            ret.push(d);
        }
        ret
    }};
}

#[macro_export]
macro_rules! parse_some_l1 {
    ($parser:expr, $f:ident, $l:expr) => {{
        let head = $parser.$f()?;
        let mut ret = vec![head];
        while $parser.sat($l).is_ok() {
            let d = $parser.$f()?;
            ret.push(d);
        }
        ret
    }};
}

#[macro_export]
macro_rules! expect_one_of {
    ($parser:expr, $($l:path => $r:expr),*) => {{
        let d = $parser.expect_one_of(&[$($l),*])?;
        match d.tok {
            $($l => $r,)*
            _ => unreachable!(),
        }
    }}
}

#[macro_export]
macro_rules! sat_one_of {
    ($parser:expr, $($l:path => $r:expr),*) => {{
        let d = $parser.sat_one_of(&[$($l),*])?;
        match d.tok {
            $($l => $r,)*
            _ => unreachable!(),
        }
    }}
}

#[macro_export]
macro_rules! parse_many_l1 {
    ($parser:expr, $f:ident, $l:expr) => {{
        let mut ret = vec![];
        while $parser.sat($l).is_ok() {
            let d = $parser.$f()?;
            ret.push(d);
        }
        ret
    }};
}

pub trait Parse {
    type Error;
    type Token;

    // required

    fn ctx(&self) -> &ParseCtx<Self::Token>;
    fn ctx_mut(&mut self) -> &mut ParseCtx<Self::Token>;
    fn expect_err(&self, expected: Self::Token, found: Option<S<Self::Token>>) -> Self::Error;
    fn expect_one_of_err(
        &self,
        expected: &[Self::Token],
        found: Option<S<Self::Token>>,
    ) -> Self::Error;

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // parsing

    fn parse<T>(
        &mut self,
        f: impl Fn(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<T, Self::Error> {
        let f = |parser: &mut Self| {
            parser.ctx_mut().push_stack();

            let ret = f(parser);

            parser.ctx_mut().pop_stack();
            ret
        };
        self.parse_roll_back(f)
    }

    fn parse_n<T>(
        &mut self,
        f: impl Fn(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<N<T>, Self::Error> {
        let f = |parser: &mut Self| match f(parser) {
            Ok(d) => Ok(parser.ctx().make_node(d)),
            Err(e) => Err(e),
        };
        self.parse(f)
    }

    fn parse_roll_back<T>(
        &mut self,
        f: impl Fn(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<T, Self::Error> {
        let cursor = self.ctx().cursor;
        match f(self) {
            Ok(d) => Ok(d),
            Err(e) => {
                self.ctx_mut().cursor = cursor;
                Err(e)
            }
        }
    }

    fn parse_roll_back_opt<T>(
        &mut self,
        f: impl Fn(&mut Self) -> Result<Option<T>, Self::Error>,
    ) -> Result<Option<T>, Self::Error> {
        let cursor = self.ctx().cursor;
        match f(self) {
            Ok(Some(d)) => Ok(Some(d)),
            d => {
                self.ctx_mut().cursor = cursor;
                d
            }
        }
    }

    fn parse_l1<T>(
        &mut self,
        tok: Self::Token,
        f: impl Fn(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<Option<T>, Self::Error>
    where
        Self::Token: Clone + Eq,
    {
        self.parse_l1_if(|t| t == &tok, f)
    }

    fn parse_l1_adv<T>(
        &mut self,
        tok: Self::Token,
        f: impl Fn(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<Option<T>, Self::Error>
        where
            Self::Token: Clone + Eq,
    {
        let f = |p: &mut Self| {
            p.parse_roll_back(|p: &mut Self| {
                p.advance();
                f(p)
            })
        };
        self.parse_l1(tok, f)
    }

    fn parse_l1_if<T>(
        &mut self,
        cond: impl Fn(&Self::Token) -> bool,
        f: impl Fn(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<Option<T>, Self::Error>
    where
        Self::Token: Clone,
    {
        let current = match self.peek() {
            Some(d) => d.tok,
            None => return Ok(None),
        };
        if cond(&current) {
            let t = f(self)?;
            Ok(Some(t))
        } else {
            Ok(None)
        }
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // movement

    fn advance(&mut self) -> Option<S<Self::Token>>
    where
        Self::Token: Clone,
    {
        self.ctx_mut().advance()
    }

    fn advance_cmp(&mut self, tok: Self::Token) -> bool
    where
        Self::Token: Clone + Eq,
    {
        self.ctx_mut().advance_cmp(tok)
    }

    fn peek(&self) -> Option<S<Self::Token>>
    where
        Self::Token: Clone,
    {
        self.ctx().peek()
    }

    fn eof(&self) -> bool {
        self.ctx().eof()
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // expect

    fn expect(&mut self, expected: Self::Token) -> Result<S<Self::Token>, Self::Error>
    where
        Self::Token: Eq + Clone,
    {
        let ret = self.sat(expected)?;
        self.advance();
        Ok(ret)
    }

    fn sat(&self, expected: Self::Token) -> Result<S<Self::Token>, Self::Error>
    where
        Self::Token: Eq + Clone,
    {
        let d = match self.peek() {
            Some(d) => d,
            None => return Err(self.expect_err(expected, None)),
        };
        if d.tok == expected {
            Ok(d)
        } else {
            Err(self.expect_err(expected, Some(d)))
        }
    }

    fn expect_one_of(&mut self, expected: &[Self::Token]) -> Result<S<Self::Token>, Self::Error>
    where
        Self::Token: Eq + Clone,
    {
        let ret = self.sat_one_of(expected)?;
        self.advance();
        Ok(ret)
    }

    fn sat_one_of(&self, expected: &[Self::Token]) -> Result<S<Self::Token>, Self::Error>
    where
        Self::Token: Eq + Clone,
    {
        let d = match self.peek() {
            Some(d) => d,
            None => return Err(self.expect_one_of_err(expected, None)),
        };

        if expected.iter().find(|e| *e == &d.tok).is_some() {
            Ok(d)
        } else {
            Err(self.expect_one_of_err(expected, Some(d)))
        }
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // meta

    fn span(&self) -> Option<Span> {
        self.ctx().span()
    }

    fn make_node<A>(&self, data: A) -> N<A> {
        self.ctx().make_node(data)
    }

    fn get_string(&self) -> Option<String> {
        let span = self.span()?;
        self.chars().get_string(span)
    }

    fn chars(&self) -> &Chars {
        self.ctx().chars()
    }

    // only for test
    fn cursor(&self) -> usize {
        self.ctx().cursor
    }
}

impl<T> ParseCtx<T> {
    fn span(&self) -> Option<Span> {
        let start = *self.call_stack.last()?;
        debug_assert!(start <= self.cursor);
        if start == self.cursor {
            None
        } else {
            let start = self.tokens[start].span;
            let end = self.tokens[self.cursor - 1].span;
            Some(start.merge(end))
        }
    }

    fn push_stack(&mut self) {
        self.call_stack.push(self.cursor)
    }

    fn pop_stack(&mut self) -> Option<usize> {
        self.call_stack.pop()
    }

    fn make_node<A>(&self, data: A) -> N<A> {
        let id = self.id_gen.next();
        let span = self.span().expect("not in parsing context");
        N { id, span, data }
    }

    fn chars(&self) -> &Chars {
        &self.chars
    }

    fn eof(&self) -> bool {
        self.cursor == self.tokens.len()
    }
}

impl<T: Clone> ParseCtx<T> {
    fn advance_if(&mut self, p: impl Fn(T) -> bool) -> bool {
        if let Some(c) = self.peek() {
            if p(c.tok) {
                self.cursor += 1;
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> Option<S<T>> {
        if self.eof() {
            None
        } else {
            let c = self.tokens[self.cursor].clone();
            self.cursor += 1;
            Some(c)
        }
    }

    fn peek(&self) -> Option<S<T>> {
        if self.eof() {
            None
        } else {
            let c = self.tokens[self.cursor].clone();
            Some(c)
        }
    }
}

impl<T: Clone + Eq> ParseCtx<T> {
    fn advance_cmp(&mut self, tok: T) -> bool {
        self.advance_if(|x| x == tok)
    }
}
