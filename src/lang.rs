use std::collections::{HashMap, HashSet};

use crate::{buff::Buff, mu::Mu, ts::Ts};

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

enum Sexpr {
    Sym(String),
    Num(u32),
    List(Vec<Sexpr>),
}

impl Sexpr {
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

impl Instr {
    fn parse_symb_list(buff: &mut Buff<char>) -> Option<Vec<String>> {
        let mut symbols = vec![buff.expect_symb()?];
        buff.save();
        while let Some(l) = buff.expect_symb() {
            symbols.push(l);
            buff.update_save();
        }
        buff.restore();
        Some(symbols)
    }

    fn parse_num_list(buff: &mut Buff<char>) -> Option<Vec<u32>> {
        let mut nums = vec![buff.expect_u32()?];
        buff.save();
        while let Some(l) = buff.expect_u32() {
            nums.push(l);
            buff.update_save();
        }
        buff.restore();
        Some(nums)
    }

    fn parse_label(buff: &mut Buff<char>) -> Option<Self> {
        buff.trim();
        buff.expect('(')?;
        buff.expect_token("label".to_string())?;
        let state = buff.expect_u32()?;
        buff.expect_blank()?;
        let label = Self::parse_symb_list(buff)?;
        buff.trim();
        buff.expect(')')?;
        Some(Instr::Label(state, label))
    }

    fn parse_trans(buff: &mut Buff<char>) -> Option<Self> {
        buff.trim();
        buff.expect('(')?;
        buff.expect_token("trans".to_string())?;
        let state1 = buff.expect_u32()?;
        buff.expect_blank()?;
        let action = buff.expect_symb()?;
        let state2 = buff.expect_u32()?;
        buff.trim();
        buff.expect(')')?;
        Some(Instr::Trans(state1, action, state2))
    }

    fn parse_actions_decl(buff: &mut Buff<char>) -> Option<Self> {
        buff.trim();
        buff.expect('(')?;
        buff.expect_token("actions".to_string())?;
        let actions = Self::parse_symb_list(buff)?;
        buff.trim();
        buff.expect(')')?;
        Some(Instr::SetActions(actions))
    }

    fn parse_props_decl(buff: &mut Buff<char>) -> Option<Self> {
        buff.trim();
        buff.expect('(')?;
        buff.expect_token("props".to_string())?;
        let props = Self::parse_symb_list(buff)?;
        buff.trim();
        buff.expect(')')?;
        Some(Instr::SetProps(props))
    }

    fn parse_init_decl(buff: &mut Buff<char>) -> Option<Self> {
        buff.trim();
        buff.expect('(')?;
        buff.expect_token("init".to_string())?;
        let initials = Self::parse_num_list(buff)?;
        buff.trim();
        buff.expect(')')?;
        Some(Instr::SetInit(initials))
    }

    fn parse_loop(buff: &mut Buff<char>) -> Option<Self> {
        buff.trim();
        buff.expect('(')?;
        buff.expect_token("loop".to_string())?;
        let state = buff.expect_u32()?;
        buff.expect_blank()?;
        let action = buff.expect_symb()?;
        buff.trim();
        buff.expect(')')?;
        Some(Instr::Loop(state, action))
    }

    fn parse(buff: &mut Buff<char>) -> Option<Self> {
        buff.save();
        if let i @ Some(_) = Self::parse_trans(buff) {
            return i;
        }
        buff.restore_save();
        if let i @ Some(_) = Self::parse_label(buff) {
            return i;
        }
        buff.restore_save();
        if let i @ Some(_) = Self::parse_props_decl(buff) {
            return i;
        }
        buff.restore_save();
        if let i @ Some(_) = Self::parse_actions_decl(buff) {
            return i;
        }
        buff.restore_save();
        if let i @ Some(_) = Self::parse_init_decl(buff) {
            return i;
        }
        buff.restore_save();
        Self::parse_loop(buff)
    }

    fn check(&self, props: &[String], actions: &[String]) -> Result<(), String> {
        match self {
            Instr::Label(_, label) => {
                for p in label {
                    if !props.contains(p) {
                        return Err(format!("undeclared proposition {}", p));
                    }
                }
                Ok(())
            }
            Instr::Trans(_, act, _) => {
                if actions.contains(act) {
                    Ok(())
                } else {
                    Err(format!("undeclared action {}", act))
                }
            }
            Instr::Loop(_, act) => {
                if actions.contains(act) {
                    Ok(())
                } else {
                    Err(format!("undeclared action {}", act))
                }
            }
            Instr::SetProps(_) => Ok(()),
            Instr::SetActions(_) => Ok(()),
            Instr::SetInit(_) => Ok(()),
            Instr::SetSpec(_) => Ok(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Prog {
    spec: Vec<Mu<String, String>>,
    props: Vec<String>,
    actions: Vec<String>,
    init: Vec<u32>,
    instructions: Vec<Instr>,
}

impl Prog {
    pub fn check(&self) -> Result<(), String> {
        for i in &self.instructions {
            i.check(&self.props, &self.actions)?;
        }
        Ok(())
    }

    pub fn parse(string: String) -> Option<Self> {
        let buff = &mut Buff::new(string.chars().collect());
        let mut instructions = vec![];
        let mut actions = vec![];
        let mut init = vec![];
        let mut spec = vec![];
        let mut props = vec![];
        while let Some(i) = Instr::parse(buff) {
            match i {
                Instr::SetProps(new_props) => props.append(&mut new_props.clone()),
                Instr::SetActions(new_actions) => actions.append(&mut new_actions.clone()),
                Instr::SetInit(new_init) => init.append(&mut new_init.clone()),
                Instr::SetSpec(new_spec) => spec.push(new_spec),
                _ => instructions.push(i),
            }
        }
        Some(Prog {
            spec,
            init,
            props,
            actions,
            instructions,
        })
    }

    pub fn compile(self) -> Ts<String, String> {
        let mut ts = Ts::<String, String>::new(vec![], self.init, vec![], vec![]);
        for instr in &self.instructions {
            match instr {
                Instr::Label(state, label) => {
                    ts.labels
                        .insert(*state, HashSet::from_iter(label.iter().cloned()));
                }
                Instr::Trans(state1, action, state2) => {
                    if !ts.states.contains(state1) {
                        ts.states.insert(*state1);
                    }
                    if !ts.states.contains(state2) {
                        ts.states.insert(*state2);
                    }
                    if let Some(post) = ts.transitions.get_mut(state1) {
                        post.insert(action.clone(), *state2);
                    } else {
                        ts.transitions.insert(
                            *state1,
                            HashMap::from_iter(vec![(action.clone(), *state2)].into_iter()),
                        );
                    }
                }
                Instr::Loop(state, action) => {
                    if !ts.states.contains(state) {
                        ts.states.insert(*state);
                    }
                    if let Some(post) = ts.transitions.get_mut(state) {
                        post.insert(action.clone(), *state);
                    } else {
                        ts.transitions.insert(
                            *state,
                            HashMap::from_iter(vec![(action.clone(), *state)].into_iter()),
                        );
                    }
                }
                _ => (),
            }
        }
        ts
    }
}

#[cfg(test)]
mod test {
    use super::Instr;
    use crate::{buff::Buff, lang::Prog, ts::Ts};

    #[test]
    fn test_1() {
        let mut buff = Buff::new("(label 1 P Q)".chars().collect());
        assert_eq!(
            Instr::parse(&mut buff).unwrap(),
            Instr::Label(1, vec!["P".to_string(), "Q".to_string()])
        );
    }

    #[test]
    fn test_2() {
        let mut buff = Buff::new("(trans 1 act1 2)".chars().collect());
        assert_eq!(
            Instr::parse(&mut buff).unwrap(),
            Instr::Trans(1, "act1".to_string(), 2)
        );
    }

    #[test]
    fn test_3() {
        let mut buff = Buff::new("(loop 1 act1)".chars().collect());
        assert_eq!(
            Instr::parse(&mut buff).unwrap(),
            Instr::Loop(1, "act1".to_string())
        );
    }

    #[test]
    fn test_4() {
        let prog = "
          (props P Q R)
          (actions act1)
          (init 1)
          (label 1 P)
          (label 2 Q)
          (trans 1 act1 2)
          (loop 2 act2)
        ";
        assert_eq!(
            Prog::parse(prog.to_string()).unwrap(),
            Prog {
                spec: vec![],
                props: vec!["P".to_string(), "Q".to_string(), "R".to_string()],
                actions: vec!["act1".to_string()],
                instructions: vec![
                    Instr::Label(1, vec!["P".to_string()]),
                    Instr::Label(2, vec!["Q".to_string()]),
                    Instr::Trans(1, "act1".to_string(), 2),
                    Instr::Loop(2, "act2".to_string()),
                ],
                init: vec![1]
            }
        )
    }

    #[test]
    fn test_5() {
        let prog = "
        (props P Q R)
        (actions act1)
        (label 1 P)
        (label 2 Q)
        (trans 1 act1 2)
        (loop 2 act2)";
        assert_eq!(
            Prog::parse(prog.to_string()).unwrap().compile(),
            Ts::new(
                vec![1, 2],
                vec![],
                vec![(1, vec!["P".to_string()]), (2, vec!["Q".to_string()])],
                vec![
                    (1, vec![("act1".to_string(), 2)]),
                    (2, vec![("act2".to_string(), 2)])
                ]
            )
        )
    }
}
