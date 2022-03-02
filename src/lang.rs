use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{buff::Buff, mu::Mu, sexpr::Sexpr, ts::Ts};

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
}

pub struct ProgEnv {
    props: HashSet<String>,
    actions: HashSet<String>,
    states: HashSet<u32>,
    spec: Vec<Mu<String, String>>,
    initial: HashSet<u32>,
    labels: HashMap<u32, HashSet<String>>,
    transitions: HashMap<u32, HashMap<String, u32>>,
}

impl ProgEnv {
    pub fn exec(&mut self, instr: Instr) -> Result<(), String> {
        match instr {
            Instr::SetProps(props) => {
                if self.props.is_empty() {
                    for prop in props {
                        self.props.insert(prop);
                    }
                    Ok(())
                } else {
                    Err("Ill-formed program: the proposition set is declared twice".to_string())
                }
            }
            Instr::SetActions(actions) => {
                if self.actions.is_empty() {
                    for action in actions {
                        self.actions.insert(action);
                    }
                    Ok(())
                } else {
                    Err("Ill-formed program: the action set is declared twice".to_string())
                }
            }
            Instr::SetInit(initial) => {
                if self.initial.is_empty() {
                    for init in initial {
                        self.initial.insert(init);
                    }
                    Ok(())
                } else {
                    Err("Ill-formed program: the initial states are declared twice".to_string())
                }
            }
            Instr::SetSpec(s) => Ok(self.spec.push(s)),
            Instr::Label(s, l) => {
                if self.labels.get(&s).is_some() {
                    Err(format!(
                        "Ill-formed program: the label for states {} is declared twice",
                        s
                    ))
                } else {
                    self.labels.insert(s, HashSet::from_iter(l.into_iter()));
                    Ok(())
                }
            }

            Instr::Trans(s1, a, s2) => {
                self.states.insert(s1);
                self.states.insert(s2);
                if let Some(h) = self.transitions.get_mut(&s1) {
                    if h.get(&a).is_some() {
                        Err(format!(
                            "Ill-formed program: the {}-transition for states {} is declared twice",
                            a, s1
                        ))
                    } else {
                        h.insert(a, s2);
                        Ok(())
                    }
                } else {
                    self.transitions.insert(s1, HashMap::from_iter([(a, s2)]));
                    Ok(())
                }
            }
            Instr::Loop(s, a) => {
                self.states.insert(s);
                if let Some(h) = self.transitions.get_mut(&s) {
                    if h.get(&a).is_some() {
                        Err(format!(
                            "Ill-formed program: the {}-transition for states {} is declared twice",
                            a, s
                        ))
                    } else {
                        h.insert(a, s);
                        Ok(())
                    }
                } else {
                    self.transitions.insert(s, HashMap::from_iter([(a, s)]));
                    Ok(())
                }
            }
        }
    }
}

pub struct Prog {
    instructions: Vec<Instr>,
}

impl Prog {
    pub fn compile(self) -> Result<Ts<String, String>, String> {
        let mut env = ProgEnv {
            props: HashSet::new(),
            actions: HashSet::new(),
            states: HashSet::new(),
            spec: vec![],
            initial: HashSet::new(),
            labels: HashMap::new(),
            transitions: HashMap::new(),
        };
        for instr in self.instructions {
            env.exec(instr)?;
        }
        Ok(Ts {
            states: env.states,
            initials: env.initial,
            labels: env.labels,
            transitions: env.transitions,
        })
    }
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
