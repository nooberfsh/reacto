use std::ops::Deref;
use std::sync::Arc;

use crate::span::Span;

#[derive(Clone, Debug)]
pub struct Chars(Arc<Vec<char>>);

impl Chars {
    pub fn new(s: &str) -> Self {
        let chars: Vec<_> = s.chars().collect();
        Chars(Arc::new(chars))
    }

    pub fn get_string(&self, span: Span) -> Option<String> {
        if span.end() > self.len() {
            None
        } else {
            let s = &self.0[span.start()..span.end()];
            Some(s.iter().collect())
        }
    }
}

impl Deref for Chars {
    type Target = [char];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<Chars> for Chars {
    fn eq(&self, other: &Chars) -> bool {
        self.0 == other.0
    }
}

impl Eq for Chars {}

impl PartialEq<String> for Chars {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<str> for Chars {
    fn eq(&self, other: &str) -> bool {
        // TODO: avoid allocation
        let other = Chars::new(other);
        self == &other
    }
}

impl PartialEq<&str> for Chars {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let a = Chars::new("123");
        assert_eq!(a, a.clone());
        assert_eq!(a, "123");
        assert_eq!(a, *"123");
        assert_eq!(a, "123".to_string());
    }

    #[test]
    fn test_get_string() {
        let a = Chars::new("a+");
        assert_eq!(a.get_string(Span::new(0, 1)).unwrap(), "a");
        assert_eq!(a.get_string(Span::new(0, 2)).unwrap(), "a+");
        assert_eq!(a.get_string(Span::new(0, 3)), None);
        assert_eq!(a.get_string(Span::new(2, 3)), None);
    }
}
