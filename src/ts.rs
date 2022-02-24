use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

use crate::mu::Mu;

pub struct Ts<A>
where
    A: Display,
{
    states: Vec<u32>,
    initials: Vec<u32>,
    transitions: HashMap<u32, HashMap<A, u32>>,
}

impl<A> Ts<A>
where
    A: Display,
    A: Eq,
    A: Clone,
    A: Hash,
{
    pub fn new(
        states: Vec<u32>,
        initials: Vec<u32>,
        transitions: Vec<(u32, Vec<(A, u32)>)>,
    ) -> Self {
        Ts {
            states,
            initials,
            transitions: transitions
                .into_iter()
                .map(|(s, post)| (s, HashMap::from_iter(post.into_iter())))
                .collect(),
        }
    }

    pub fn states(&self) -> HashSet<u32> {
        HashSet::from_iter(self.states.clone().into_iter())
    }

    pub fn succ(&self, x: &u32, act: &A) -> Option<&u32> {
        self.transitions.get(x).and_then(|succ| succ.get(act))
    }

    pub fn check(&self, spec: &Mu<A, u32>) -> bool {
        let sat = self.sat(spec, HashMap::new());
        self.initials.iter().all(|s| sat.contains(s))
    }

    pub fn sat(&self, spec: &Mu<A, u32>, env: HashMap<String, Vec<u32>>) -> Vec<u32> {
        match spec {
            Mu::Lit(p) => {
                let s = [*p].into_iter().collect();
                self.states().intersection(&s).cloned().collect()
            }
            Mu::Neg(a) => {
                let s1 = self.states();
                let s2 = self.sat(a, env).iter().cloned().collect::<HashSet<u32>>();
                s1.difference(&s2).cloned().collect()
            }
            Mu::And(a, b) => {
                let s1 = self
                    .sat(a, env.clone())
                    .iter()
                    .cloned()
                    .collect::<HashSet<u32>>();
                let s2 = self
                    .sat(b, env.clone())
                    .iter()
                    .cloned()
                    .collect::<HashSet<u32>>();
                s1.intersection(&s2).cloned().collect()
            }
            Mu::Or(a, b) => {
                let s1 = self
                    .sat(a, env.clone())
                    .iter()
                    .cloned()
                    .collect::<HashSet<u32>>();
                let s2 = self
                    .sat(b, env.clone())
                    .iter()
                    .cloned()
                    .collect::<HashSet<u32>>();
                s1.union(&s2).cloned().collect()
            }
            Mu::Gfp(_x, _a) => todo!(),
            Mu::All(act, a) => {
                let sa = self.sat(a, env.clone());
                let mut s = Vec::<u32>::new();
                for s1 in self.states() {
                    if self.succ(&s1, &act).iter().all(|s2| sa.contains(s2)) {
                        s.push(s1)
                    }
                }
                s
            }
            Mu::Lfp(_, _) => todo!(),
            Mu::Ex(act, a) => {
                let sa = self.sat(a, env.clone());
                let mut s = Vec::<u32>::new();
                for s1 in self.states() {
                    if self.succ(&s1, &act).iter().any(|s2| sa.contains(s2)) {
                        s.push(s1)
                    }
                }
                s
            }
            Mu::Var(x) => env.get(x).unwrap().clone(),
        }
    }
}

impl<A> Display for Ts<A>
where
    A: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "digraph {{")?;
        writeln!(f, "  node [shape=circle]")?;
        for x in self.states.iter() {
            if self.initials.contains(x) {
                writeln!(f, "  {} [shape=doublecircle]", x)?
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
