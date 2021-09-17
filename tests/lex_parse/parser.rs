use reacto::lex::Lex;
use reacto::parse::{Parse, ParseCtx};
use reacto::span::S;

use super::lexer::*;

pub fn new_parser(s: &str) -> Parser {
    let lexer = new_lexer(s);
    Parser::new(lexer)
}

pub fn new_parser_wo_sp(s: &str) -> Parser {
    let lexer = new_lexer(s);
    Parser::new_wo_sp(lexer)
}

#[derive(Clone, Debug)]
pub enum ParseError {
    Expect(Token, Option<S<Token>>),
    ExpectMulti(Vec<Token>, Option<S<Token>>),
}

pub struct Parser {
    ctx: ParseCtx<Token>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let tokens = lexer.tokens().unwrap();
        let chars = lexer.chars();
        let ctx = ParseCtx::new(chars.clone(), tokens);
        Parser { ctx }
    }

    pub fn new_wo_sp(mut lexer: Lexer) -> Self {
        let tokens = lexer
            .tokens()
            .unwrap()
            .into_iter()
            .filter(|tok| tok.tok != Token::Whitespace)
            .collect();
        let chars = lexer.chars();
        let ctx = ParseCtx::new(chars.clone(), tokens);
        Parser { ctx }
    }

    pub fn parse_ident(&mut self) -> Result<(), ParseError> {
        self.expect(Token::Ident)?;
        Ok(())
    }
}

impl Parse for Parser {
    type Error = ParseError;
    type Token = Token;

    fn ctx(&self) -> &ParseCtx<Self::Token> {
        &self.ctx
    }

    fn ctx_mut(&mut self) -> &mut ParseCtx<Self::Token> {
        &mut self.ctx
    }

    fn expect_err(&self, expected: Self::Token, found: Option<S<Self::Token>>) -> Self::Error {
        ParseError::Expect(expected, found)
    }

    fn expect_one_of_err(
        &self,
        expected: &[Self::Token],
        found: Option<S<Self::Token>>,
    ) -> Self::Error {
        let expected = expected.iter().map(|t| t.clone()).collect();
        ParseError::ExpectMulti(expected, found)
    }
}
