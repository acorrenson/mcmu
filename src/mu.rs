use std::{fmt::Display, str::FromStr};

use crate::buff::Buff;

#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Mu<char, u32> {
    fn parse_var(buff: &mut Buff<char>) -> Option<char> {
        let c = buff.next()?;
        match c {
            'a'..='z' | 'A'..='Z' => Some(c),
            _ => None,
        }
    }

    fn parse_act(buff: &mut Buff<char>) -> Option<char> {
        buff.next()
    }

    fn parse_atom(buff: &mut Buff<char>) -> Option<Self> {
        let c = buff.top()?;
        match c {
            '0'..='9' => {
                let n = buff.expect_u32()?;
                Some(Mu::Lit(n))
            }
            'a'..='z' | 'A'..='Z' => {
                buff.pop();
                Some(Mu::Var(c.to_string()))
            }
            '(' => {
                buff.pop();
                let mu = Self::parse(buff)?;
                buff.expect(')');
                Some(mu)
            }
            '⟨' => {
                buff.pop();
                let act = Self::parse_act(buff)?;
                buff.expect('⟩')?;
                Self::parse_atom(buff).map(|lhs| Mu::All(act, Box::new(lhs)))
            }
            '[' => {
                buff.pop();
                let act = Self::parse_act(buff)?;
                buff.expect(']')?;
                Self::parse_atom(buff).map(|lhs| Mu::All(act, Box::new(lhs)))
            }
            '¬' => {
                buff.pop();
                Self::parse_atom(buff).map(|lhs| Mu::Neg(Box::new(lhs)))
            }
            'μ' => {
                buff.pop();
                let x = Self::parse_var(buff)?;
                buff.expect('.')?;
                let lhs = Self::parse(buff)?;
                Some(Mu::Lfp(x.to_string(), Box::new(lhs)))
            }
            'ν' => {
                buff.pop();
                let x = Self::parse_var(buff)?;
                buff.expect('.')?;
                let lhs = Self::parse(buff)?;
                Some(Mu::Gfp(x.to_string(), Box::new(lhs)))
            }
            _ => None,
        }
    }

    fn parse_disj(buff: &mut Buff<char>) -> Option<Self> {
        let mut lhs = Self::parse_conj(buff)?;
        while let Some('∨') = buff.top() {
            buff.pop();
            let rhs = Self::parse_conj(buff)?;
            lhs = Mu::Or(Box::new(lhs), Box::new(rhs));
        }
        Some(lhs)
    }

    fn parse_conj(buff: &mut Buff<char>) -> Option<Self> {
        let mut lhs = Self::parse_atom(buff)?;
        while let Some('∧') = buff.top() {
            buff.pop();
            let rhs = Self::parse_atom(buff)?;
            lhs = Mu::And(Box::new(lhs), Box::new(rhs));
        }
        Some(lhs)
    }

    fn parse(buff: &mut Buff<char>) -> Option<Self> {
        let res = Self::parse_disj(buff);
        if buff.next().is_some() {
            None
        } else {
            res
        }
    }
}

impl FromStr for Mu<char, u32> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buff = Buff::new(s.chars().filter(|c| *c != ' ').collect());
        Self::parse(&mut buff).ok_or_else(|| "Error while parsing Mu formula".to_string())
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
        println!("{}", Mu::from_str("μx.[a]x ∨ x").unwrap())
    }
}
