use reacto::lex::{Lex, LexCtx};
use reacto::span::Span;

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
                    return Err("sting not closed".to_string());
                }
                Token::LitString
            }
            c if is_letter(c) => {
                self.advance_while(is_digit_letter);
                Token::Ident
            }
            _ => return Err("unknown char".to_string()),
        };
        Ok(Some(ty))
    }
}

pub fn new_lexer(s: &str) -> Lexer {
    let ctx = LexCtx::new(s);
    Lexer { ctx }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// helper functions
pub fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

pub fn is_letter(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_')
}

pub fn is_digit_letter(c: char) -> bool {
    is_digit(c) || is_letter(c)
}
