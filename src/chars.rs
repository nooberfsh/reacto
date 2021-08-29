use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Chars(Arc<Vec<char>>);

impl Chars {
    pub fn new(s: &str) -> Self {
        let chars: Vec<_> = s.chars().collect();
        Chars(Arc::new(chars))
    }
}

impl Deref for Chars {
    type Target = [char];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
