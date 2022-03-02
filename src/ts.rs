use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

use crate::mu::Mu;

#[derive(PartialEq, Eq, Debug)]
pub struct Ts<A, P>
where
    A: Display + Eq + Clone + Hash,
    P: Eq + Display + Clone + Hash,
{
    pub(crate) states: HashSet<u32>,
    pub(crate) initial: HashSet<u32>,
    pub(crate) labels: HashMap<u32, HashSet<P>>,
    pub(crate) transitions: HashMap<u32, HashMap<A, u32>>,
    pub(crate) spec: Vec<Mu<A, P>>,
}

impl<A, P> Ts<A, P>
where
    A: Display + Eq + Clone + Hash,
    P: Eq + Display + Clone + Hash,
{
    pub fn new(
        states: Vec<u32>,
        initials: Vec<u32>,
        labels: Vec<(u32, Vec<P>)>,
        transitions: Vec<(u32, Vec<(A, u32)>)>,
        spec: Vec<Mu<A, P>>,
    ) -> Self {
        Ts {
            states: states.into_iter().collect(),
            initial: initials.into_iter().collect(),
            labels: labels
                .into_iter()
                .map(|(s, labels)| (s, HashSet::from_iter(labels.into_iter())))
                .collect(),
            transitions: transitions
                .into_iter()
                .map(|(s, post)| (s, HashMap::from_iter(post.into_iter())))
                .collect(),
            spec,
        }
    }

    pub fn label(&self, x: &u32) -> HashSet<P> {
        self.labels.get(x).unwrap_or(&HashSet::new()).clone()
    }

    pub fn succ(&self, x: &u32, act: &A) -> Option<&u32> {
        self.transitions.get(x).and_then(|succ| succ.get(act))
    }

    pub fn check(&self) -> bool {
        self.spec.iter().all(|form| {
            let sat = self.sat(form, HashMap::new());
            self.initial.iter().all(|s| sat.contains(s))
        })
    }

    pub fn sat(&self, spec: &Mu<A, P>, env: HashMap<String, HashSet<u32>>) -> HashSet<u32> {
        match spec {
            Mu::Lit(p) => {
                let s = self
                    .states
                    .clone()
                    .into_iter()
                    .filter(|x| self.label(x).contains(p))
                    .collect();
                self.states.intersection(&s).cloned().collect()
            }
            Mu::Neg(a) => {
                let s2 = self.sat(a, env).iter().cloned().collect();
                self.states.difference(&s2).cloned().collect()
            }
            Mu::And(a, b) => {
                let sat_a = self.sat(a, env.clone());
                let sat_b = self.sat(b, env);
                sat_a.intersection(&sat_b).cloned().collect()
            }
            Mu::Or(a, b) => {
                let sat_a = self.sat(a, env.clone());
                let sat_b = self.sat(b, env);
                sat_a.union(&sat_b).cloned().collect()
            }
            Mu::Gfp(x, a) => {
                let mut sat = self.states.clone();
                loop {
                    let mut env_next = env.clone();
                    env_next.insert(x.clone(), sat.clone());
                    let sat_next = self.sat(a, env_next);
                    if sat_next == sat {
                        break;
                    }
                    sat = sat_next;
                }
                sat
            }
            Mu::All(act, a) => {
                let sat_a = self.sat(a, env);
                let mut sat_all = HashSet::<u32>::new();
                for s1 in &self.states {
                    if self.succ(s1, act).iter().all(|s2| sat_a.contains(s2)) {
                        sat_all.insert(*s1);
                    }
                }
                sat_all
            }
            Mu::Lfp(x, a) => {
                let mut sat = HashSet::new();
                loop {
                    let mut env_next = env.clone();
                    env_next.insert(x.clone(), sat.clone());
                    let sat_next = self.sat(a, env_next);
                    if sat_next == sat {
                        break;
                    }
                    sat = sat_next;
                }
                sat
            }
            Mu::Ex(act, a) => {
                let sat_a = self.sat(a, env);
                let mut sat_ex = HashSet::new();
                for s1 in &self.states {
                    if self.succ(s1, act).iter().any(|s2| sat_a.contains(s2)) {
                        sat_ex.insert(*s1);
                    }
                }
                sat_ex
            }
            Mu::Var(x) => env.get(x).unwrap().clone(),
        }
    }
}

impl<A, P> Display for Ts<A, P>
where
    A: Display,
    A: Display,
    A: Eq,
    A: Clone,
    A: Hash,
    P: Eq,
    P: Display,
    P: Clone,
    P: Hash,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "digraph {{")?;
        writeln!(f, "  node [shape=circle]")?;
        for x in self.states.iter() {
            if self.initial.contains(x) {
                writeln!(
                    f,
                    "  {} [shape=doublecircle, label=\"{{{}}}\"]",
                    x,
                    self.label(x)
                        .iter()
                        .map(|p| format!("{}", p))
                        .collect::<Vec<String>>()
                        .join(", ")
                )?
            }
        }
        for (x, post) in self.transitions.iter() {
            for (a, y) in post {
                writeln!(f, "  {} -> {} [label=\" {}\"];", x, y, a)?
            }
        }
        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_1() {
        let ts = Ts::new(
            vec![1, 2],
            vec![1],
            vec![(2, vec!['A'])],
            vec![(1, vec![('a', 2)])],
            vec![Mu::All('b', Box::new(Mu::Lit('A')))],
        );
        assert!(ts.check())
    }

    #[test]
    fn test_2() {
        let ts = Ts::new(
            vec![1, 2],
            vec![1],
            vec![(2, vec!['A'])],
            vec![(1, vec![('a', 2)])],
            vec![Mu::All('a', Box::new(Mu::Lit('A')))],
        );
        assert!(ts.check())
    }

    #[test]
    fn test_3() {
        let ts = Ts::new(
            vec![1, 2],
            vec![1],
            vec![(1, vec!['A'])],
            vec![(1, vec![('a', 2)])],
            vec![Mu::All('a', Box::new(Mu::Lit('A')))],
        );
        assert!(!ts.check())
    }

    #[test]
    fn test_4() {
        let ts = Ts::new(
            vec![1, 2],
            vec![1],
            vec![(1, vec!['A'])],
            vec![(1, vec![('a', 2)])],
            vec![Mu::Ex('a', Box::new(Mu::Lit('A')))],
        );
        assert!(!ts.check())
    }

    #[test]
    fn test_5() {
        let ts = Ts::new(
            vec![1, 2],
            vec![1],
            vec![(1, vec!['A'])],
            vec![(1, vec![('a', 2)])],
            vec![Mu::Ex('b', Box::new(Mu::Lit('A')))],
        );
        assert!(!ts.check())
    }

    #[test]
    fn test_6() {
        let ts = Ts::new(
            vec![1, 2, 3],
            vec![1],
            vec![(3, vec!['A'])],
            vec![(1, vec![('a', 2), ('b', 3)])],
            vec![Mu::Ex('b', Box::new(Mu::Lit('A')))],
        );
        assert!(ts.check())
    }

    #[test]
    fn test_7() {
        let phi = Mu::Or(Box::new(Mu::Lit('B')), Box::new(Mu::Lit('C')));
        let phi = Mu::Or(Box::new(Mu::Lit('A')), Box::new(phi));
        let phi = Mu::And(Box::new(Mu::Var("X".to_string())), Box::new(phi));
        let spec = Mu::Gfp("X".to_string(), Box::new(phi));
        let ts = Ts::new(
            vec![1, 2, 3],
            vec![1],
            vec![(1, vec!['A']), (2, vec!['B']), (3, vec!['C'])],
            vec![
                (1, vec![('a', 2)]),
                (2, vec![('a', 3)]),
                (3, vec![('a', 1)]),
            ],
            vec![spec],
        );
        assert!(ts.check());
    }

    #[test]
    fn test_8() {
        let phi = Mu::Or(Box::new(Mu::Lit('B')), Box::new(Mu::Lit('C')));
        let phi = Mu::Or(Box::new(Mu::Lit('A')), Box::new(phi));
        let phi = Mu::And(Box::new(Mu::Var("X".to_string())), Box::new(phi));
        let spec = Mu::Gfp("X".to_string(), Box::new(phi));
        let ts = Ts::new(
            vec![1, 2, 3],
            vec![1],
            vec![(1, vec!['A']), (2, vec!['B']), (3, vec!['C'])],
            vec![
                (1, vec![('a', 2)]),
                (2, vec![('a', 3)]),
                (3, vec![('a', 1)]),
            ],
            vec![spec],
        );
        assert!(ts.check());
    }
}
