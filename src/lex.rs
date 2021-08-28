use crate::chars::Chars;

pub trait Lex {
    fn chars(&self) -> &Chars;
    fn cursor(&self) -> usize;
    fn start(&self) -> usize;
    fn inc_cursor(&mut self);

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // provided

    fn eof(&self) -> bool {
        self.cursor() == self.chars().len()
    }

    fn peek(&self) -> Option<char> {
        if self.eof() {
            None
        } else {
            Some(self.chars()[self.cursor()])
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.eof() {
            None
        } else {
            let c = self.chars()[self.cursor()];
            self.inc_cursor();
            Some(c)
        }
    }

    fn advance_cmp(&mut self, c: char) -> bool {
        self.advance_if(|x| x == c)
    }

    fn advance_if(&mut self, p: impl Fn(char) -> bool) -> bool {
        if let Some(c) = self.peek() {
            if p(c) {
                self.inc_cursor();
                return true;
            }
        }
        false
    }

    fn advance_while(&mut self, p: impl Fn(char) -> bool) -> usize {
        let mut num = 0;
        while let Some(c) = self.peek() {
            if !p(c) {
                break;
            }
            self.inc_cursor();
            num += 1;
        }
        num
    }
}
