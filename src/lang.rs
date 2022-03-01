use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{buff::Buff, mu::Mu, sexpr::Sexpr};

#[derive(Debug, PartialEq, Eq)]
pub enum Instr {
    SetProps(Vec<String>),
    SetActions(Vec<String>),
    SetInit(Vec<u32>),
    SetSpec(Mu<String, String>),
    Label(u32, Vec<String>),
    Trans(u32, String, u32),
    Loop(u32, String),
}

impl Instr {
    fn expect_symb_list(buff: &mut Buff<Sexpr>) -> Option<Vec<String>> {
        buff.convert_while(Sexpr::is_symb, Sexpr::get_symb)
    }

    fn expect_num_list(buff: &mut Buff<Sexpr>) -> Option<Vec<u32>> {
        buff.convert_while(Sexpr::is_num, Sexpr::get_num)
    }

    fn from_sexpr(s: Sexpr) -> Option<Instr> {
        if !s.is_list() {
            return None;
        }
        let list = s.get_list();
        let mut buff = Buff::new(list);
        let cmd = buff.expect_cond(Sexpr::is_symb)?.get_symb();
        if cmd == *"label" {
            let n = buff.expect_cond(Sexpr::is_num)?.get_num();
            let label = Self::expect_symb_list(&mut buff)?;
            Some(Instr::Label(n, label))
        } else if cmd == *"props" {
            let label = Self::expect_symb_list(&mut buff)?;
            Some(Instr::SetProps(label))
        } else if cmd == *"init" {
            let init = Self::expect_num_list(&mut buff)?;
            Some(Instr::SetInit(init))
        } else if cmd == *"actions" {
            let actions = Self::expect_symb_list(&mut buff)?;
            Some(Instr::SetActions(actions))
        } else if cmd == *"trans" {
            let state1 = buff.expect_cond(Sexpr::is_num)?.get_num();
            let action = buff.expect_cond(Sexpr::is_symb)?.get_symb();
            let state2 = buff.expect_cond(Sexpr::is_num)?.get_num();
            Some(Instr::Trans(state1, action, state2))
        } else if cmd == *"loop" {
            let state1 = buff.expect_cond(Sexpr::is_num)?.get_num();
            let action = buff.expect_cond(Sexpr::is_symb)?.get_symb();
            Some(Instr::Loop(state1, action))
        } else {
            None
        }
    }

    fn parse(buff: &mut Buff<char>) -> Option<Self> {
        Sexpr::parse(buff).and_then(Instr::from_sexpr)
    }

    pub fn well_formed(&self, env: ProgEnv) -> Result<(), String> {
        match self {
            Instr::SetProps(_) | Instr::SetActions(_) | Instr::SetInit(_) | Instr::SetSpec(_) => {
                Ok(())
            }
            Instr::Label(state, _) => {
                if let Some(l) = env.labels.get(state) {
                    Err(format!(
                        "at instruction {:?}: Label for state {} is already defined to be {:?}",
                        self, state, l,
                    ))
                } else if !env.states.contains(state) {
                    Err(format!(
                        "at instruction {:?}: state {} is undefined. Defined states are {:?}",
                        self, state, env.states
                    ))
                } else {
                    Ok(())
                }
            }
            Instr::Trans(state1, action, state2) => todo!(),
            Instr::Loop(_, _) => todo!(),
        }
    }
}

pub struct ProgEnv {
    props: HashSet<u32>,
    states: HashSet<u32>,
    init: HashSet<u32>,
    labels: HashMap<u32, HashSet<String>>,
}

pub struct Prog {
    instructions: Vec<Instr>,
}

impl FromStr for Prog {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buff = Buff::new(s.chars().collect());
        buff.expect_list(Instr::parse)
            .ok_or_else(|| "Prog: parse error".to_string())
            .map(|instructions| Prog { instructions })
    }
}
