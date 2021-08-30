use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct NodeId(pub(crate) usize);

#[derive(Clone, Debug)]
pub(crate) struct IdGen {
    id: Arc<AtomicUsize>,
}

impl IdGen {
    pub(crate) fn new() -> Self {
        IdGen {
            id: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub(crate) fn next(&self) -> NodeId {
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        NodeId(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let gen = IdGen::new();
        let a = gen.next();
        assert_eq!(a.0, 0);
        let a = gen.next();
        assert_eq!(a.0, 1);
        let a = gen.next();
        assert_eq!(a.0, 2);
    }
}
