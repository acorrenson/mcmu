use std::fmt::Display;

/// Buffer data structure for parsing
pub struct Buff<T> {
    data: Vec<T>,
    pos: usize,
    stack: Vec<usize>,
}

impl<T> Buff<T>
where
    T: Display,
    T: Eq,
    T: Clone,
{
    /// Create a new buffer
    pub fn new(data: Vec<T>) -> Self {
        Buff {
            data,
            pos: 0,
            stack: vec![0],
        }
    }

    pub fn save(&mut self) {
        self.stack.push(self.pos);
    }

    pub fn restore(&mut self) {
        self.pos = self.stack.pop().unwrap();
    }

    pub fn restore_save(&mut self) {
        self.restore();
        self.save();
    }

    pub fn update_save(&mut self) {
        self.stack.pop();
        self.stack.push(self.pos);
    }

    /// Get the first element of the buffer
    /// and returns `None` if the buffer is empty
    pub fn top(&self) -> Option<T> {
        self.data.get(self.pos).cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.data.len()
    }

    /// Drop the first element of the buffer
    /// and returns `None` if the buffer is empty
    pub fn pop(&mut self) -> Option<()> {
        if self.is_empty() {
            None
        } else {
            self.pos += 1;
            Some(())
        }
    }

    /// Get the first element of the buffer
    /// and drops it.
    /// Returns `None` if the buffer is empty
    pub fn next(&mut self) -> Option<T> {
        let x = self.top()?;
        self.pop()?;
        Some(x)
    }

    /// Compare a given element with the first
    /// element of the buffer and drops it if they
    /// are equals.
    /// Returns `None` if the buffer is empty
    pub fn expect(&mut self, x: T) -> Option<()> {
        let y = self.next()?;
        if x == y {
            Some(())
        } else {
            None
        }
    }

    pub fn expect_cond<P>(&mut self, cond: P) -> Option<T>
    where
        P: FnOnce(&T) -> bool,
    {
        let y = self.next()?;
        if cond(&y) {
            Some(y)
        } else {
            None
        }
    }

    pub fn expect_list<U>(&mut self, pre: fn(&T) -> bool, conv: fn(T) -> U) -> Option<Vec<U>> {
        let mut list = vec![conv(self.expect_cond(pre)?)];
        self.save();
        while let Some(l) = self.expect_cond(pre) {
            list.push(conv(l));
            self.update_save();
        }
        self.restore();
        Some(list)
    }

    pub fn expect_one_of(&mut self, alt: Vec<T>) -> Option<()> {
        let y = self.next()?;
        if alt.contains(&y) {
            Some(())
        } else {
            None
        }
    }
}

impl Buff<char> {
    fn top_is_space(&self) -> bool {
        if let Some(c) = self.top() {
            vec!['\t', '\n', ' '].contains(&c)
        } else {
            false
        }
    }

    pub fn trim(&mut self) {
        while !self.is_empty() && self.top_is_space() {
            self.pos += 1;
        }
    }

    pub fn expect_u32(&mut self) -> Option<u32> {
        self.trim();
        let mut num = self.expect_digit()?.to_digit(10).unwrap();
        while let Some(c) = self.top() {
            if c.is_digit(10) {
                self.pop();
                num += 10 * num + c.to_digit(10).unwrap();
            } else {
                break;
            }
        }
        Some(num)
    }

    pub fn expect_alpha(&mut self) -> Option<char> {
        let c = self.next()?;
        if c.is_ascii_alphabetic() {
            Some(c)
        } else {
            None
        }
    }

    pub fn expect_digit(&mut self) -> Option<char> {
        let c = self.next()?;
        if c.is_digit(10) {
            Some(c)
        } else {
            None
        }
    }

    pub fn expect_symb(&mut self) -> Option<String> {
        self.trim();
        let mut symb = String::new();
        symb.push(self.expect_alpha()?);
        while let Some(c) = self.top() {
            if c.is_alphanumeric() {
                self.pop();
                symb.push(c);
            } else {
                break;
            }
        }
        Some(symb)
    }

    pub fn expect_blank(&mut self) -> Option<()> {
        self.expect_one_of(vec![' ', '\t', '\n'])
    }

    pub fn expect_token(&mut self, tok: String) -> Option<()> {
        let symb = self.expect_symb()?;
        if symb == tok {
            Some(())
        } else {
            None
        }
    }
}
