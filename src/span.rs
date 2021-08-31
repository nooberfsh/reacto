#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Span {
    start: usize,
    end: usize,
}

#[derive(Clone, Debug, Copy)]
pub struct S<T> {
    pub span: Span,
    pub tok: T,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start < end, "start must less then end");
        Span { start, end }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn merge(&self, other: Span) -> Span {
        let start = self.start.min(other.start);
        let end = self.end.max(other.end);
        Self::new(start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        Span::new(0, 1);
    }

    #[test]
    #[should_panic]
    fn test_new_panic() {
        Span::new(1, 0);
    }

    #[test]
    #[should_panic]
    fn test_new_panic2() {
        Span::new(1, 1);
    }

    #[test]
    fn test_start_end() {
        let a = Span::new(1, 2);
        assert_eq!(a.start(), 1);
        assert_eq!(a.end(), 2);
    }

    #[test]
    fn test_len() {
        let a = Span::new(1, 2);
        assert_eq!(a.len(), 1);
    }

    #[test]
    fn test_merge() {
        let a = Span::new(1, 3);
        let res = a.merge(a);
        assert_eq!(res, a);

        let b = Span::new(0, 2);
        let res = a.merge(b);
        assert_eq!(res, Span::new(0, 3));

        let b = Span::new(0, 1);
        let res = a.merge(b);
        assert_eq!(res, Span::new(0, 3));

        let b = Span::new(0, 4);
        let res = a.merge(b);
        assert_eq!(res, Span::new(0, 4));

        let b = Span::new(1, 4);
        let res = a.merge(b);
        assert_eq!(res, Span::new(1, 4));

        let b = Span::new(3, 4);
        let res = a.merge(b);
        assert_eq!(res, Span::new(1, 4));
    }
}
