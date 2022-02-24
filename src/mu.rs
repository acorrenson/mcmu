use std::fmt::Display;

#[derive(Clone)]
pub enum Mu<A, P>
where
    A: Display,
    A: Clone,
    P: Display,
{
    Lit(P),
    Neg(Box<Mu<A, P>>),
    And(Box<Mu<A, P>>, Box<Mu<A, P>>),
    Or(Box<Mu<A, P>>, Box<Mu<A, P>>),
    Gfp(String, Box<Mu<A, P>>),
    All(A, Box<Mu<A, P>>),
    Lfp(String, Box<Mu<A, P>>),
    Ex(A, Box<Mu<A, P>>),
    Var(String),
}

struct Buff {
    data: Vec<char>,
    pos: usize,
}

impl Buff {
    fn not_blank(c: &char) -> bool {
        !vec![' ', '\t', '\n'].contains(c)
    }

    fn new(input: String) -> Self {
        Buff {
            data: input.chars().filter(Self::not_blank).collect(),
            pos: 0,
        }
    }

    fn get(&self) -> Option<char> {
        self.data.get(self.pos).cloned()
    }

    fn get_u32(&mut self) -> Option<u32> {
        let mut num = 0;
        let mut c = self.get()?;
        while c.is_digit(10) {
            num = 10 * num + c.to_digit(10).unwrap();
            self.pop();
            if let Some(new_c) = self.get() {
                c = new_c;
            } else {
                break;
            }
        }
        Some(num)
    }

    fn pop(&mut self) {
        self.pos += 1;
    }

    fn next(&mut self) -> Option<char> {
        let c = self.get();
        self.pos += 1;
        c
    }
}

impl Mu<char, u32> {
    fn parse_var(buff: &mut Buff) -> Option<char> {
        let c = buff.next()?;
        match c {
            'a'..='z' | 'A'..='Z' => Some(c),
            _ => None,
        }
    }

    fn parse_act(buff: &mut Buff) -> Option<char> {
        buff.next()
    }

    fn parse_atom(buff: &mut Buff) -> Option<Self> {
        let c = buff.get()?;
        match c {
            '0'..='9' => {
                let n = buff.get_u32()?;
                Some(Mu::Lit(n))
            }
            'a'..='z' | 'A'..='Z' => {
                buff.pop();
                Some(Mu::Var(c.to_string()))
            }
            '(' => {
                buff.pop();
                let mu = Self::parse(buff)?;
                buff.next()
                    .and_then(|c| if c == ')' { Some(mu) } else { None })
            }
            '⟨' => {
                buff.pop();
                let act = Self::parse_act(buff)?;
                buff.next()
                    .and_then(|c| if c == '⟩' { Some(()) } else { None })?;
                Self::parse_atom(buff).map(|lhs| Mu::All(act, Box::new(lhs)))
            }
            '[' => {
                buff.pop();
                let act = Self::parse_act(buff)?;
                buff.next()
                    .and_then(|c| if c == ']' { Some(()) } else { None })?;
                Self::parse_atom(buff).map(|lhs| Mu::All(act, Box::new(lhs)))
            }
            '¬' => {
                buff.pop();
                Self::parse_atom(buff).map(|lhs| Mu::Neg(Box::new(lhs)))
            }
            'μ' => {
                buff.pop();
                let x = Self::parse_var(buff)?;
                buff.next()
                    .and_then(|c| if c == '.' { Some(()) } else { None })?;
                let lhs = Self::parse(buff)?;
                Some(Mu::Lfp(x.to_string(), Box::new(lhs)))
            }
            'ν' => {
                buff.pop();
                let x = Self::parse_var(buff)?;
                buff.next().and_then(|c| {
                    if c == '.' {
                        let lhs = Self::parse(buff)?;
                        Some(Mu::Gfp(x.to_string(), Box::new(lhs)))
                    } else {
                        None
                    }
                })
            }
            _ => None,
        }
    }

    fn parse_disj(buff: &mut Buff) -> Option<Self> {
        let mut lhs = Self::parse_conj(buff)?;
        while let Some('∨') = buff.get() {
            buff.pop();
            let rhs = Self::parse_conj(buff)?;
            lhs = Mu::Or(Box::new(lhs), Box::new(rhs));
        }
        Some(lhs)
    }

    fn parse_conj(buff: &mut Buff) -> Option<Self> {
        let mut lhs = Self::parse_atom(buff)?;
        while let Some('∧') = buff.get() {
            buff.pop();
            let rhs = Self::parse_atom(buff)?;
            lhs = Mu::And(Box::new(lhs), Box::new(rhs));
        }
        Some(lhs)
    }

    fn parse(buff: &mut Buff) -> Option<Self> {
        let res = Self::parse_disj(buff);
        if let Some(_) = buff.next() {
            None
        } else {
            res
        }
    }

    pub fn from_str(str: &str) -> Option<Self> {
        Self::parse(&mut Buff::new(str.to_string()))
    }
}

impl<A, P> Display for Mu<A, P>
where
    A: Display,
    A: Clone,
    P: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mu::Lit(p) => write!(f, "{}", p),
            Mu::Neg(a) => write!(f, "¬{}", a),
            Mu::And(a, b) => write!(f, "({} ∧ {})", a, b),
            Mu::Or(a, b) => write!(f, "({} ∨ {})", a, b),
            Mu::Gfp(x, a) => write!(f, "ν {}.({})", x, a),
            Mu::All(act, a) => write!(f, "([{}]{})", act, a),
            Mu::Lfp(x, a) => write!(f, "μ {}.({})", x, a),
            Mu::Ex(act, a) => write!(f, "(⟨{}⟩{})", act, a),
            Mu::Var(c) => write!(f, "{}", c),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        println!("{}", Mu::from_str("μx.[a]x ∨ μy.⟨b⟩y").unwrap())
    }
}
