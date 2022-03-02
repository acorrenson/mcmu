use std::{fmt::Display, str::FromStr};

use crate::{buff::Buff, sexpr::Sexpr};

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
                let mu = Self::parse_disj(buff)?;
                buff.expect(')')?;
                Some(mu)
            }
            '⟨' => {
                buff.pop();
                let act = Self::parse_act(buff)?;
                buff.expect('⟩')?;
                Self::parse_atom(buff).map(|lhs| Mu::Ex(act, Box::new(lhs)))
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
                let lhs = Self::parse_disj(buff)?;
                Some(Mu::Lfp(x.to_string(), Box::new(lhs)))
            }
            'ν' => {
                buff.pop();
                let x = Self::parse_var(buff)?;
                buff.expect('.')?;
                let lhs = Self::parse_disj(buff)?;
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

impl Mu<String, String> {
    fn parse_binop(buff: &mut Buff<Sexpr>, is_or: bool) -> Option<Self> {
        let args = buff.convert_list(Self::from_sexpr)?;
        buff.expect_end()?;
        if is_or {
            args.into_iter()
                .reduce(|lhs, rhs| Mu::Or(Box::new(lhs), Box::new(rhs)))
        } else {
            args.into_iter()
                .reduce(|lhs, rhs| Mu::And(Box::new(lhs), Box::new(rhs)))
        }
    }

    fn parse_neg(buff: &mut Buff<Sexpr>) -> Option<Self> {
        let mu = buff.next().and_then(Self::from_sexpr)?;
        buff.expect_end()?;
        Some(Mu::Neg(Box::new(mu)))
    }

    fn lit_to_var(self, var: &String) -> Self {
        match self {
            Mu::Lit(x) => {
                if x == *var {
                    Mu::Var(x)
                } else {
                    Mu::Lit(x)
                }
            }
            Mu::Neg(lhs) => Mu::Neg(Box::new(lhs.lit_to_var(var))),
            Mu::And(lhs, rhs) => {
                Mu::And(Box::new(lhs.lit_to_var(var)), Box::new(rhs.lit_to_var(var)))
            }
            Mu::Or(lhs, rhs) => {
                Mu::Or(Box::new(lhs.lit_to_var(var)), Box::new(rhs.lit_to_var(var)))
            }
            Mu::Gfp(x, lhs) => Mu::Gfp(x.clone(), Box::new(lhs.lit_to_var(var))),
            Mu::All(a, lhs) => Mu::All(a.clone(), Box::new(lhs.lit_to_var(var))),
            Mu::Lfp(x, lhs) => Mu::Lfp(x.clone(), Box::new(lhs.lit_to_var(var))),
            Mu::Ex(a, lhs) => Mu::Ex(a.clone(), Box::new(lhs.lit_to_var(var))),
            Mu::Var(_) => self,
        }
    }

    fn parse_fixpoint(buff: &mut Buff<Sexpr>, is_lfp: bool) -> Option<Self> {
        let var = buff.next()?.get_singleton_opt()?.get_symb_opt()?;
        let mu = buff.next().and_then(Self::from_sexpr)?.lit_to_var(&var);
        if is_lfp {
            Some(Mu::Lfp(var, Box::new(mu)))
        } else {
            Some(Mu::Gfp(var, Box::new(mu)))
        }
    }

    fn parse_quantifier(buff: &mut Buff<Sexpr>, is_any: bool) -> Option<Self> {
        let action = buff.next()?.get_singleton_opt()?.get_symb_opt()?;
        let mu = buff.next().and_then(Self::from_sexpr)?;
        buff.expect_end()?;
        if is_any {
            Some(Mu::Ex(action, Box::new(mu)))
        } else {
            Some(Mu::All(action, Box::new(mu)))
        }
    }

    pub fn from_sexpr(sexpr: Sexpr) -> Option<Self> {
        match sexpr {
            Sexpr::Sym(s) => Some(Mu::Lit(s)),
            Sexpr::Num(_) => None,
            Sexpr::List(list) => {
                let mut buff = Buff::new(list);
                let op = buff.expect_cond(Sexpr::is_symb)?.get_symb();
                if op == *"any" || op == *"any" {
                    Self::parse_quantifier(&mut buff, op == *"any")
                } else if op == *"lfp" || op == "gfp" {
                    Self::parse_fixpoint(&mut buff, op == *"lfp")
                } else if op == *"or" || op == *"and" {
                    Self::parse_binop(&mut buff, op == *"or")
                } else if op == *"not" {
                    Self::parse_neg(&mut buff)
                } else {
                    None
                }
            }
        }
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
    use super::Mu::*;
    use super::Sexpr::*;
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Mu::from_str("μx.[a]x ∨ x").unwrap(),
            Lfp(
                "x".to_string(),
                Box::new(Or(
                    Box::new(All('a', Box::new(Var("x".to_string())))),
                    Box::new(Var("x".to_string()))
                ))
            )
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Mu::from_str("μx.⟨a⟩x ∨ x").unwrap(),
            Lfp(
                "x".to_string(),
                Box::new(Or(
                    Box::new(Ex('a', Box::new(Var("x".to_string())))),
                    Box::new(Var("x".to_string()))
                ))
            )
        )
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Mu::from_str("μx.νy.x ∧ y").unwrap(),
            Lfp(
                "x".to_string(),
                Box::new(Gfp(
                    "y".to_string(),
                    Box::new(And(
                        Box::new(Var("x".to_string())),
                        Box::new(Var("y".to_string()))
                    )),
                ))
            )
        )
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Mu::from_str("(μx.x) ∧ y").unwrap(),
            And(
                Box::new(Lfp("x".to_string(), Box::new(Var("x".to_string())))),
                Box::new(Var("y".to_string()))
            )
        )
    }

    #[test]
    fn test_5() {
        let sexpr = List(vec![
            Sym("lfp".to_string()),
            List(vec![Sym("x".to_string())]),
            Sym("x".to_string()),
        ]);
        assert_eq!(
            Mu::from_sexpr(sexpr).unwrap(),
            Lfp("x".to_string(), Box::new(Var("x".to_string()))),
        )
    }

    #[test]
    fn test_6() {
        let sexpr = List(vec![
            Sym("gfp".to_string()),
            List(vec![Sym("x".to_string())]),
            Sym("x".to_string()),
        ]);
        assert_eq!(
            Mu::from_sexpr(sexpr).unwrap(),
            Gfp("x".to_string(), Box::new(Var("x".to_string()))),
        )
    }

    #[test]
    fn test_7() {
        let sexpr = List(vec![
            Sym("and".to_string()),
            Sym("x".to_string()),
            Sym("y".to_string()),
        ]);
        assert_eq!(
            Mu::from_sexpr(sexpr).unwrap(),
            And(
                Box::new(Lit("x".to_string())),
                Box::new(Lit("y".to_string()))
            ),
        )
    }

    #[test]
    fn test_8() {
        let sexpr = List(vec![
            Sym("and".to_string()),
            List(vec![
                Sym("lfp".to_string()),
                List(vec![Sym("x".to_string())]),
                Sym("x".to_string()),
            ]),
            Sym("x".to_string()),
        ]);
        assert_eq!(
            Mu::from_sexpr(sexpr).unwrap(),
            And(
                Box::new(Lfp("x".to_string(), Box::new(Var("x".to_string())))),
                Box::new(Lit("x".to_string()))
            ),
        )
    }
}
