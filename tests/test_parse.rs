mod lex_parse;

use lex_parse::lexer::*;
use lex_parse::parser::*;
use reacto::*;
use reacto::parse::Parse;
use reacto::span::{Span, S};

#[test]
fn test_move() {
    let mut a = new_parser("a+");
    assert_eq!(a.eof(), false);
    assert_eq!(a.peek().unwrap().tok, Token::Ident);
    assert_eq!(a.advance().unwrap().tok, Token::Ident);
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
            assert_eq!(expected, Token::Plus);
            assert_eq!(found.unwrap().span, Span::new(2, 3));
            assert_eq!(found.unwrap().tok, Token::Ident)
        }
        _ => panic!("invalid error"),
    }

    let s = a.expect(Token::Ident).unwrap();
    assert_eq!(s.span, Span::new(2, 3));
    assert_eq!(s.tok, Token::Ident);

    // eof
    match a.expect(Token::Plus).unwrap_err() {
        ParseError::Expect(expected, found) => {
            assert_eq!(expected, Token::Plus);
            assert!(found.is_none());
        }
        _ => panic!("invalid error"),
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
            assert_eq!(expected, Token::Plus);
            assert_eq!(found.unwrap().span, Span::new(0, 1));
            assert_eq!(found.unwrap().tok, Token::Ident)
        }
        _ => panic!("invalid error"),
    }

    let s = a.sat(Token::Ident).unwrap();
    assert_eq!(s.span, Span::new(0, 1));
    assert_eq!(s.tok, Token::Ident);

    // eof
    let a = new_parser("");
    match a.sat(Token::Plus).unwrap_err() {
        ParseError::Expect(expected, found) => {
            assert_eq!(expected, Token::Plus);
            assert!(found.is_none());
        }
        _ => panic!("invalid error"),
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
    match a
        .expect_one_of(&[Token::Plus, Token::Whitespace])
        .unwrap_err()
    {
        ParseError::ExpectMulti(expected, found) => {
            assert_eq!(expected, vec![Token::Plus, Token::Whitespace]);
            assert_eq!(found.unwrap().span, Span::new(2, 3));
            assert_eq!(found.unwrap().tok, Token::Ident)
        }
        _ => panic!("invalid error"),
    }

    let s = a.expect_one_of(&[Token::Ident, Token::Plus]).unwrap();
    assert_eq!(s.span, Span::new(2, 3));
    assert_eq!(s.tok, Token::Ident);

    // eof
    match a
        .expect_one_of(&[Token::Plus, Token::Whitespace])
        .unwrap_err()
    {
        ParseError::ExpectMulti(expected, found) => {
            assert_eq!(expected, vec![Token::Plus, Token::Whitespace]);
            assert!(found.is_none());
        }
        _ => panic!("invalid error"),
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
            assert_eq!(expected, vec![Token::Plus, Token::Whitespace]);
            assert_eq!(found.unwrap().span, Span::new(0, 1));
            assert_eq!(found.unwrap().tok, Token::Ident)
        }
        _ => panic!("invalid error"),
    }

    let s = a.sat_one_of(&[Token::Ident, Token::Plus]).unwrap();
    assert_eq!(s.span, Span::new(0, 1));
    assert_eq!(s.tok, Token::Ident);

    // eof
    let a = new_parser("");
    match a.sat_one_of(&[Token::Plus, Token::Whitespace]).unwrap_err() {
        ParseError::ExpectMulti(expected, found) => {
            assert_eq!(expected, vec![Token::Plus, Token::Whitespace]);
            assert!(found.is_none());
        }
        _ => panic!("invalid error"),
    }
}

#[test]
fn test_span() {
    let a = new_parser("a+");
    assert_eq!(a.span(), None);
}

#[test]
fn test_chars() {
    let a = new_parser("+ ab");
    assert_eq!(a.chars(), "+ ab")
}

#[test]
fn test_parse_roll_back() {
    let mut a = new_parser("a+");
    let res = a.parse_roll_back(|p| p.expect(Token::Ident)).unwrap();
    assert_eq!(res.tok, Token::Ident);
    assert_eq!(a.cursor(), 1);

    let res = a.parse_roll_back(|p| p.expect(Token::Ident));
    assert!(res.is_err());
    assert_eq!(a.cursor(), 1);
}

#[test]
fn test_parse_roll_back_opt() {
    let mut a = new_parser("a+");
    let res = a
        .parse_roll_back_opt(|p| p.expect(Token::Ident).map(Some))
        .unwrap()
        .unwrap();
    assert_eq!(res.tok, Token::Ident);
    assert_eq!(a.cursor(), 1);

    let res = a.parse_roll_back_opt(|p| p.expect(Token::Whitespace).map(Some));
    assert!(res.is_err());
    assert_eq!(a.cursor(), 1);

    let res: Option<S<Token>> = a
        .parse_roll_back_opt(|p| p.expect(Token::Plus).map(|_| None))
        .unwrap();
    assert!(res.is_none());
    assert_eq!(a.cursor(), 1);
}

#[test]
fn test_parse_l1() {
    let mut a = new_parser("a+");
    let res = a
        .parse_l1(Token::Ident, |p| p.expect(Token::Ident))
        .unwrap()
        .unwrap();
    assert_eq!(res.tok, Token::Ident);
    assert_eq!(a.cursor(), 1);

    let res = a
        .parse_l1(Token::Ident, |p| p.expect(Token::Ident))
        .unwrap();
    assert!(res.is_none());
    assert_eq!(a.cursor(), 1);

    // error
    let res = a.parse_l1(Token::Plus, |p| p.expect(Token::Ident));
    assert!(res.is_err());
    assert_eq!(a.cursor(), 1);

    // eof
    a.advance();
    let res = a.parse_l1(Token::Plus, |p| p.expect(Token::Ident)).unwrap();
    assert!(res.is_none());
}

#[test]
fn test_parse_l1_not() {
    let mut a = new_parser("a+");
    let res = a
        .parse_l1_not(Token::Plus, |p| p.expect(Token::Ident))
        .unwrap()
        .unwrap();
    assert_eq!(res.tok, Token::Ident);
    assert_eq!(a.cursor(), 1);

    let res = a
        .parse_l1_not(Token::Plus, |p| p.expect(Token::Plus))
        .unwrap();
    assert!(res.is_none());

    // error
    let res = a.parse_l1_not(Token::Ident, |p| p.expect(Token::Ident));
    assert!(res.is_err());

    // eof
    a.advance();
    let res = a.parse_l1_not(Token::Ident, |p| p.expect(Token::Ident)).unwrap();
    assert!(res.is_none());
}

#[test]
fn test_parse_l1_adv() {
    let mut a = new_parser("a+a");
    let res = a
        .parse_l1_adv(Token::Ident, |p| p.expect(Token::Plus))
        .unwrap()
        .unwrap();
    assert_eq!(res.tok, Token::Plus);
    assert_eq!(a.cursor(), 2);

    let res = a
        .parse_l1_adv(Token::Plus, |p| p.expect(Token::Ident))
        .unwrap();
    assert!(res.is_none());
    assert_eq!(a.cursor(), 2);

    // error
    let res = a.parse_l1_adv(Token::Ident, |p| p.expect(Token::Ident));
    assert!(res.is_err());
    assert_eq!(a.cursor(), 2);

    // eof
    a.advance();
    let res = a.parse_l1_adv(Token::Plus, |p| p.expect(Token::Ident)).unwrap();
    assert!(res.is_none());
}

#[test]
fn test_parse_l1_if() {
    let mut a = new_parser("a+");
    let res = a
        .parse_l1_if(|tok| tok == &Token::Ident, |p| p.expect(Token::Ident))
        .unwrap()
        .unwrap();
    assert_eq!(res.tok, Token::Ident);
    assert_eq!(a.cursor(), 1);

    let res = a
        .parse_l1_if(|tok| tok == &Token::Ident, |p| p.expect(Token::Ident))
        .unwrap();
    assert!(res.is_none());
    assert_eq!(a.cursor(), 1);

    // error
    let res = a.parse_l1_if(|tok| tok == &Token::Plus, |p| p.expect(Token::Ident));
    assert!(res.is_err());
    assert_eq!(a.cursor(), 1);

    // eof
    a.advance();
    let res = a
        .parse_l1_if(|tok| tok == &Token::Plus, |p| p.expect(Token::Ident))
        .unwrap();
    assert!(res.is_none());
}

#[test]
fn test_parse() {
    let mut a = new_parser("a+");
    let res = a.parse(|p| p.expect(Token::Ident)).unwrap();
    assert_eq!(res.tok, Token::Ident);
    assert_eq!(a.cursor(), 1);

    let res = a.parse(|p| p.expect(Token::Ident));
    assert!(res.is_err());
    assert_eq!(a.cursor(), 1);
}

#[test]
fn test_parse_n() {
    let mut a = new_parser("ab+");
    let res = a.parse_n(|p| p.expect(Token::Ident)).unwrap();
    assert_eq!(res.data.tok, Token::Ident);
    assert_eq!(res.span, Span::new(0, 2));
    assert_eq!(res.id.get(), 0);

    let res = a.parse_n(|p| p.expect(Token::Ident));
    assert!(res.is_err());

    let res = a.parse_n(|p| p.expect(Token::Plus)).unwrap();
    assert_eq!(res.data.tok, Token::Plus);
    assert_eq!(res.span, Span::new(2, 3));
    assert_eq!(res.id.get(), 1);

    let res = a.parse_n(|p| p.expect(Token::Ident));
    assert!(res.is_err());
}

////////////////////////////////////////////////////////////////////////////////////////////////
// macro tests

#[test]
fn test_parse_many_to() {
    let mut a = new_parser("a b c+");
    let mut f = || -> Result<Vec<()>, ParseError> {
        let d = parse_many_to!(&mut a, parse_ident, Token::Whitespace, Token::Plus);
        Ok(d)
    };
    assert_eq!(f().unwrap(), vec![(), (), ()]);
    assert!(a.sat(Token::Plus).is_ok());

    let mut a = new_parser("+");
    let mut f = || -> Result<Vec<()>, ParseError> {
        let d = parse_many_to!(&mut a, parse_ident, Token::Whitespace, Token::Plus);
        Ok(d)
    };
    assert_eq!(f().unwrap(), vec![]);
    assert!(a.sat(Token::Plus).is_ok());

    let mut a = new_parser("a");
    let mut f = || -> Result<Vec<()>, ParseError> {
        let d = parse_many_to!(&mut a, parse_ident, Token::Whitespace, Token::Plus);
        Ok(d)
    };
    assert!(f().is_err());
    assert!(a.eof());
}

#[test]
fn test_parse_many_after() {
    let mut a = new_parser("a b c+a");
    let mut f = || -> Result<Vec<()>, ParseError> {
        let d = parse_many_after!(&mut a, parse_ident, Token::Whitespace, Token::Plus);
        Ok(d)
    };
    assert_eq!(f().unwrap(), vec![(), (), ()]);
    assert!(a.sat(Token::Ident).is_ok());

    let mut a = new_parser("+a");
    let mut f = || -> Result<Vec<()>, ParseError> {
        let d = parse_many_after!(&mut a, parse_ident, Token::Whitespace, Token::Plus);
        Ok(d)
    };
    assert_eq!(f().unwrap(), vec![]);
    assert!(a.sat(Token::Ident).is_ok());

    let mut a = new_parser("a");
    let mut f = || -> Result<Vec<()>, ParseError> {
        let d = parse_many_after!(&mut a, parse_ident, Token::Whitespace, Token::Plus);
        Ok(d)
    };
    assert!(f().is_err());
    assert!(a.eof());
}
