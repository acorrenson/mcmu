use std::collections::{HashMap, HashSet};

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
        buff.expect_list(Sexpr::is_symb, Sexpr::get_symb)
    }

    fn expect_num_list(buff: &mut Buff<Sexpr>) -> Option<Vec<u32>> {
        buff.expect_list(Sexpr::is_num, Sexpr::get_num)
    }

    fn to_command(s: Sexpr) -> Option<Instr> {
        if !s.is_list() {
            return None;
        }
        let list = s.get_list();
        let mut buff = Buff::new(list);
        let cmd = buff.expect_cond(Sexpr::is_symb)?.get_symb();
        if cmd == "label".to_string() {
            let n = buff.expect_cond(Sexpr::is_num)?.get_num();
            let label = Self::expect_symb_list(&mut buff)?;
            Some(Instr::Label(n, label))
        } else if cmd == "props".to_string() {
            let label = Self::expect_symb_list(&mut buff)?;
            Some(Instr::SetProps(label))
        } else if cmd == "init".to_string() {
            let init = Self::expect_num_list(&mut buff)?;
            Some(Instr::SetInit(init))
        } else if cmd == "actions".to_string() {
            let actions = Self::expect_symb_list(&mut buff)?;
            Some(Instr::SetActions(actions))
        } else if cmd == "trans".to_string() {
            let state1 = buff.expect_cond(Sexpr::is_num)?.get_num();
            let action = buff.expect_cond(Sexpr::is_symb)?.get_symb();
            let state2 = buff.expect_cond(Sexpr::is_num)?.get_num();
            Some(Instr::Trans(state1, action, state2))
        } else if cmd == "loop".to_string() {
            let state1 = buff.expect_cond(Sexpr::is_num)?.get_num();
            let action = buff.expect_cond(Sexpr::is_symb)?.get_symb();
            Some(Instr::Loop(state1, action))
        } else {
            None
        }
    }

    pub fn check(&self, props: &[String], actions: &[String]) -> Result<(), String> {
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

// impl Prog {
//     pub fn check(&self) -> Result<(), String> {
//         for i in &self.instructions {
//             i.check(&self.props, &self.actions)?;
//         }
//         Ok(())
//     }

//     pub fn parse(string: String) -> Option<Self> {
//         let buff = &mut Buff::new(string.chars().collect());
//         let mut instructions = vec![];
//         let mut actions = vec![];
//         let mut init = vec![];
//         let mut spec = vec![];
//         let mut props = vec![];
//         while let Some(i) = Instr::parse(buff) {
//             match i {
//                 Instr::SetProps(new_props) => props.append(&mut new_props.clone()),
//                 Instr::SetActions(new_actions) => actions.append(&mut new_actions.clone()),
//                 Instr::SetInit(new_init) => init.append(&mut new_init.clone()),
//                 Instr::SetSpec(new_spec) => spec.push(new_spec),
//                 _ => instructions.push(i),
//             }
//         }
//         Some(Prog {
//             spec,
//             init,
//             props,
//             actions,
//             instructions,
//         })
//     }

//     pub fn compile(self) -> Ts<String, String> {
//         let mut ts = Ts::<String, String>::new(vec![], self.init, vec![], vec![]);
//         for instr in &self.instructions {
//             match instr {
//                 Instr::Label(state, label) => {
//                     ts.labels
//                         .insert(*state, HashSet::from_iter(label.iter().cloned()));
//                 }
//                 Instr::Trans(state1, action, state2) => {
//                     if !ts.states.contains(state1) {
//                         ts.states.insert(*state1);
//                     }
//                     if !ts.states.contains(state2) {
//                         ts.states.insert(*state2);
//                     }
//                     if let Some(post) = ts.transitions.get_mut(state1) {
//                         post.insert(action.clone(), *state2);
//                     } else {
//                         ts.transitions.insert(
//                             *state1,
//                             HashMap::from_iter(vec![(action.clone(), *state2)].into_iter()),
//                         );
//                     }
//                 }
//                 Instr::Loop(state, action) => {
//                     if !ts.states.contains(state) {
//                         ts.states.insert(*state);
//                     }
//                     if let Some(post) = ts.transitions.get_mut(state) {
//                         post.insert(action.clone(), *state);
//                     } else {
//                         ts.transitions.insert(
//                             *state,
//                             HashMap::from_iter(vec![(action.clone(), *state)].into_iter()),
//                         );
//                     }
//                 }
//                 _ => (),
//             }
//         }
//         ts
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::Instr;
//     use crate::{buff::Buff, lang::Prog, ts::Ts};

//     #[test]
//     fn test_1() {
//         let mut buff = Buff::new("(label 1 P Q)".chars().collect());
//         assert_eq!(
//             Instr::parse(&mut buff).unwrap(),
//             Instr::Label(1, vec!["P".to_string(), "Q".to_string()])
//         );
//     }

//     #[test]
//     fn test_2() {
//         let mut buff = Buff::new("(trans 1 act1 2)".chars().collect());
//         assert_eq!(
//             Instr::parse(&mut buff).unwrap(),
//             Instr::Trans(1, "act1".to_string(), 2)
//         );
//     }

//     #[test]
//     fn test_3() {
//         let mut buff = Buff::new("(loop 1 act1)".chars().collect());
//         assert_eq!(
//             Instr::parse(&mut buff).unwrap(),
//             Instr::Loop(1, "act1".to_string())
//         );
//     }

//     #[test]
//     fn test_4() {
//         let prog = "
//           (props P Q R)
//           (actions act1)
//           (init 1)
//           (label 1 P)
//           (label 2 Q)
//           (trans 1 act1 2)
//           (loop 2 act2)
//         ";
//         assert_eq!(
//             Prog::parse(prog.to_string()).unwrap(),
//             Prog {
//                 spec: vec![],
//                 props: vec!["P".to_string(), "Q".to_string(), "R".to_string()],
//                 actions: vec!["act1".to_string()],
//                 instructions: vec![
//                     Instr::Label(1, vec!["P".to_string()]),
//                     Instr::Label(2, vec!["Q".to_string()]),
//                     Instr::Trans(1, "act1".to_string(), 2),
//                     Instr::Loop(2, "act2".to_string()),
//                 ],
//                 init: vec![1]
//             }
//         )
//     }

//     #[test]
//     fn test_5() {
//         let prog = "
//         (props P Q R)
//         (actions act1)
//         (label 1 P)
//         (label 2 Q)
//         (trans 1 act1 2)
//         (loop 2 act2)";
//         assert_eq!(
//             Prog::parse(prog.to_string()).unwrap().compile(),
//             Ts::new(
//                 vec![1, 2],
//                 vec![],
//                 vec![(1, vec!["P".to_string()]), (2, vec!["Q".to_string()])],
//                 vec![
//                     (1, vec![("act1".to_string(), 2)]),
//                     (2, vec![("act2".to_string(), 2)])
//                 ]
//             )
//         )
//     }
// }
