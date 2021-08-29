use std::fmt;
use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};

use crate::node_id::NodeId;
use crate::span::Span;

#[derive(Clone)]
pub struct N<T> {
    pub span: Span,
    pub id: NodeId,
    pub data: T,
}

impl<T: fmt::Debug> fmt::Debug for N<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}

impl<T> Deref for N<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for N<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: PartialEq> PartialEq for N<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: Eq> Eq for N<T> {}

impl<T: Hash> Hash for N<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state)
    }
}
