mod lex_parse;

use lex_parse::lexer::*;
use reacto::lex::Lex;
use reacto::span::Span;

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
    assert_eq!(Span::new(0, 0), a.span());
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
