use crate::buff::Buff;

#[derive(Debug, PartialEq, Eq)]
pub enum Instr {
    Label(u32, Vec<String>),
    Trans(u32, String, u32),
    Loop(u32, String),
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

    fn parse_actions_decl(buff: &mut Buff<char>) -> Option<Vec<String>> {
        buff.trim();
        buff.expect('(')?;
        buff.expect_token("actions".to_string())?;
        let actions = Self::parse_symb_list(buff)?;
        buff.trim();
        buff.expect(')')?;
        Some(actions)
    }

    fn parse_props_decl(buff: &mut Buff<char>) -> Option<Vec<String>> {
        buff.trim();
        buff.expect('(')?;
        buff.expect_token("props".to_string())?;
        let actions = Self::parse_symb_list(buff)?;
        buff.trim();
        buff.expect(')')?;
        Some(actions)
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
        Self::parse_loop(buff)
    }

    fn check(&self, props: &Vec<String>, actions: &Vec<String>) -> Result<(), String> {
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
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Prog {
    props: Vec<String>,
    actions: Vec<String>,
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
        let props = Instr::parse_props_decl(buff)?;
        let actions = Instr::parse_actions_decl(buff)?;
        while let Some(i) = Instr::parse(buff) {
            instructions.push(i);
        }
        Some(Prog {
            props,
            actions,
            instructions,
        })
    }
}

#[cfg(test)]
mod test {
    use super::Instr;
    use crate::{buff::Buff, lang::Prog};

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
          (label 1 P)
          (label 2 Q)
          (trans 1 act1 2)
          (loop 2 act2)
        ";
        assert_eq!(
            Prog::parse(prog.to_string()).unwrap(),
            Prog {
                props: vec!["P".to_string(), "Q".to_string(), "R".to_string()],
                actions: vec!["act1".to_string()],
                instructions: vec![
                    Instr::Label(1, vec!["P".to_string()]),
                    Instr::Label(2, vec!["Q".to_string()]),
                    Instr::Trans(1, "act1".to_string(), 2),
                    Instr::Loop(2, "act2".to_string()),
                ]
            }
        )
    }
}
