mod lex_parse;

use reacto::parse::Parse;
use lex_parse::parser::*;
use lex_parse::lexer::*;
use reacto::span::Span;

#[test]
fn test_move() {
    let mut a = new_parser("a+");
    assert_eq!(a.eof(), false);
     assert_eq!(a.peek().unwrap().tok, Token::Ident);
    assert_eq!(a.advance().unwrap().tok,Token::Ident);
    assert_eq!(a.peek().unwrap().tok, Token::Plus);
    assert_eq!(a.advance().unwrap().tok, Token::Plus);
    assert_eq!(a.eof(), true);
    assert!(a.peek().is_none());
    assert!(a.advance().is_none());
}


#[test]
fn test_advance_cmp() {
    let mut a = new_parser("a+");
    assert_eq!(a.advance_cmp(Token::Plus), false);
    assert_eq!(a.peek().unwrap().tok, Token::Ident);

    assert_eq!(a.advance_cmp(Token::Ident), true);
    assert_eq!(a.peek().unwrap().tok, Token::Plus);

    assert_eq!(a.advance_cmp(Token::Plus), true);
    assert!(a.peek().is_none());

    assert_eq!(a.advance_cmp(Token::Plus), false);
    assert!(a.peek().is_none());
}

#[test]
fn test_expect() {
    let mut a = new_parser("a+b");
    // success
    let s = a.expect(Token::Ident).unwrap();
    assert_eq!(s.span, Span::new(0, 1));
    assert_eq!(s.tok, Token::Ident);

    let s = a.expect(Token::Plus).unwrap();
    assert_eq!(s.span, Span::new(1, 2));
    assert_eq!(s.tok, Token::Plus);

    // failed
    match a.expect(Token::Plus).unwrap_err() {
        ParseError::Expect(expected, found) => {
            assert_eq!(expected, Token::Plus)           ;
            assert_eq!(found.unwrap().span, Span::new(2, 3));
            assert_eq!(found.unwrap().tok, Token::Ident)
        }
        _ => panic!("invalid error")
    }

    let s = a.expect(Token::Ident).unwrap();
    assert_eq!(s.span, Span::new(2, 3));
    assert_eq!(s.tok, Token::Ident);

    // eof
    match a.expect(Token::Plus).unwrap_err() {
        ParseError::Expect(expected, found) => {
            assert_eq!(expected, Token::Plus)           ;
            assert!(found.is_none());
        }
        _ => panic!("invalid error")
    }
}

#[test]
fn test_sat() {
    let a = new_parser("a");
    // success
    let s = a.sat(Token::Ident).unwrap();
    assert_eq!(s.span, Span::new(0, 1));
    assert_eq!(s.tok, Token::Ident);

    let s = a.sat(Token::Ident).unwrap();
    assert_eq!(s.span, Span::new(0, 1));
    assert_eq!(s.tok, Token::Ident);

    // failed
    match a.sat(Token::Plus).unwrap_err() {
        ParseError::Expect(expected, found) => {
            assert_eq!(expected, Token::Plus)           ;
            assert_eq!(found.unwrap().span, Span::new(0, 1));
            assert_eq!(found.unwrap().tok, Token::Ident)
        }
        _ => panic!("invalid error")
    }

    let s = a.sat(Token::Ident).unwrap();
    assert_eq!(s.span, Span::new(0, 1));
    assert_eq!(s.tok, Token::Ident);

    // eof
    let a = new_parser("");
    match a.sat(Token::Plus).unwrap_err() {
        ParseError::Expect(expected, found) => {
            assert_eq!(expected, Token::Plus)           ;
            assert!(found.is_none());
        }
        _ => panic!("invalid error")
    }
}

#[test]
fn test_expect_one_of() {
    let mut a = new_parser("a+b");
    // success
    let s = a.expect_one_of(&[Token::Ident, Token::Plus]).unwrap();
    assert_eq!(s.span, Span::new(0, 1));
    assert_eq!(s.tok, Token::Ident);

    let s = a.expect_one_of(&[Token::Ident, Token::Plus]).unwrap();
    assert_eq!(s.span, Span::new(1, 2));
    assert_eq!(s.tok, Token::Plus);

    // failed
    match a.expect_one_of(&[Token::Plus, Token::Whitespace]).unwrap_err() {
        ParseError::ExpectMulti(expected, found) => {
            assert_eq!(expected, vec![Token::Plus, Token::Whitespace])           ;
            assert_eq!(found.unwrap().span, Span::new(2, 3));
            assert_eq!(found.unwrap().tok, Token::Ident)
        }
        _ => panic!("invalid error")
    }

    let s = a.expect_one_of(&[Token::Ident, Token::Plus]).unwrap();
    assert_eq!(s.span, Span::new(2, 3));
    assert_eq!(s.tok, Token::Ident);

    // eof
    match a.expect_one_of(&[Token::Plus, Token::Whitespace]).unwrap_err() {
        ParseError::ExpectMulti(expected, found) => {
            assert_eq!(expected, vec![Token::Plus, Token::Whitespace])           ;
            assert!(found.is_none());
        }
        _ => panic!("invalid error")
    }
}

#[test]
fn test_sat_one_of() {
    let a = new_parser("a");
    // success
     let s = a.sat_one_of(&[Token::Ident, Token::Plus]).unwrap();
    assert_eq!(s.span, Span::new(0, 1));
    assert_eq!(s.tok, Token::Ident);

    let s = a.sat_one_of(&[Token::Ident, Token::Plus]).unwrap();
    assert_eq!(s.span, Span::new(0, 1));
    assert_eq!(s.tok, Token::Ident);

    // failed
    match a.sat_one_of(&[Token::Plus, Token::Whitespace]).unwrap_err() {
        ParseError::ExpectMulti(expected, found) => {
            assert_eq!(expected, vec![Token::Plus, Token::Whitespace])           ;
            assert_eq!(found.unwrap().span, Span::new(0, 1));
            assert_eq!(found.unwrap().tok, Token::Ident)
        }
        _ => panic!("invalid error")
    }

    let s = a.sat_one_of(&[Token::Ident, Token::Plus]).unwrap();
    assert_eq!(s.span, Span::new(0, 1));
    assert_eq!(s.tok, Token::Ident);

    // eof
    let a = new_parser("");
    match a.sat_one_of(&[Token::Plus, Token::Whitespace]).unwrap_err() {
        ParseError::ExpectMulti(expected, found) => {
            assert_eq!(expected, vec![Token::Plus, Token::Whitespace])           ;
            assert!(found.is_none());
        }
        _ => panic!("invalid error")
    }
}

#[test]
fn test_span() {
    let a = new_parser("a+");
    assert_eq!(Span::new(0, 0), a.span());
}

#[test]
fn test_get_string() {
    let a = new_parser("a+");
    assert_eq!(a.get_string(Span::new(0, 0)).unwrap(), "");
    assert_eq!(a.get_string(Span::new(0, 1)).unwrap(), "a");
    assert_eq!(a.get_string(Span::new(0, 2)).unwrap(), "a+");
    assert_eq!(a.get_string(Span::new(0, 3)), None);
    assert_eq!(a.get_string(Span::new(2, 3)), None);
}

#[test]
fn test_chars() {
    let a = new_parser("+ ab");
    assert_eq!(a.chars(), "+ ab")
}

#[test]
fn test_parse_roll_back() {
}