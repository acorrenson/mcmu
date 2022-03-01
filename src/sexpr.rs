use std::fmt::Display;

use crate::buff::Buff;

#[derive(Clone, PartialEq, Eq)]
pub enum Sexpr {
    Sym(String),
    Num(u32),
    List(Vec<Sexpr>),
}

impl Sexpr {
    pub fn is_symb(&self) -> bool {
        if let Sexpr::Sym(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_num(&self) -> bool {
        if let Sexpr::Num(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_list(&self) -> bool {
        if let Sexpr::List(_) = self {
            true
        } else {
            false
        }
    }

    pub fn get_symb(self) -> String {
        if let Sexpr::Sym(s) = self {
            s
        } else {
            panic!()
        }
    }

    pub fn get_num(self) -> u32 {
        if let Sexpr::Num(s) = self {
            s
        } else {
            panic!()
        }
    }

    pub fn get_list(self) -> Vec<Self> {
        if let Sexpr::List(l) = self {
            l
        } else {
            panic!()
        }
    }

    pub fn parse_list(buff: &mut Buff<char>) -> Option<Vec<Self>> {
        let mut sexps = vec![Self::parse(buff)?];
        buff.save();
        while let Some(l) = Self::parse(buff) {
            sexps.push(l);
            buff.update_save();
        }
        buff.restore();
        Some(sexps)
    }

    pub fn parse(buff: &mut Buff<char>) -> Option<Self> {
        buff.trim();
        match buff.top()? {
            '0'..='9' => buff.expect_u32().map(Sexpr::Num),
            '(' => {
                let list = Self::parse_list(buff)?;
                buff.trim();
                buff.expect(')')?;
                Some(Sexpr::List(list))
            }
            _ => buff.expect_symb().map(Sexpr::Sym),
        }
    }
}

impl Display for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sexpr::Sym(x) => write!(f, "{}", x),
            Sexpr::Num(n) => write!(f, "{}", n),
            Sexpr::List(l) => {
                let ls = l
                    .iter()
                    .map(|s| format!("{}", s))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "({})", ls)
            }
        }
    }
}
