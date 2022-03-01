use std::{env::args, fs};

use mu_calc::{lang::Prog, mu::Mu};

fn run(input: String, spec: &Mu<String, String>) -> Result<bool, String> {
    // let prog = Prog::parse(input).ok_or("Syntax error while parsing the input file")?;
    // prog.check()?;
    // let ts = prog.compile();
    // Ok(ts.check(spec))
    Ok(true)
}

fn main() {
    let file = args().collect::<Vec<String>>()[1].clone();
    let res = fs::read_to_string(file.as_str())
        .map_err(|err| format!("{}", err))
        .and_then(|input| run(input, &Mu::Lit("A".to_string())));
    match res {
        Ok(b) => println!("Result of the verification: {}", b),
        Err(err) => eprintln!("Verification failed: {}", err),
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use mu_calc::ts::Ts;

    use super::*;

    fn test_spec(ts: &Ts<char, char>, spec: &Mu<char, char>, expect: bool) {
        println!("system:\n{}", ts);
        println!("spec:\n{}", spec);
        let res = ts.check(spec);
        println!("correct:{}", res);
        assert_eq!(res, expect)
    }

    #[test]
    fn test_1() {
        let ts = Ts::new(
            vec![1, 2],
            vec![1],
            vec![(2, vec!['A'])],
            vec![(1, vec![('a', 2)])],
        );
        let spec = Mu::All('b', Box::new(Mu::Lit('A')));
        test_spec(&ts, &spec, true)
    }

    #[test]
    fn test_2() {
        let ts = Ts::new(
            vec![1, 2],
            vec![1],
            vec![(2, vec!['A'])],
            vec![(1, vec![('a', 2)])],
        );
        let spec = Mu::All('a', Box::new(Mu::Lit('A')));
        test_spec(&ts, &spec, true)
    }

    #[test]
    fn test_3() {
        let ts = Ts::new(
            vec![1, 2],
            vec![1],
            vec![(1, vec!['A'])],
            vec![(1, vec![('a', 2)])],
        );
        let spec = Mu::All('a', Box::new(Mu::Lit('A')));
        test_spec(&ts, &spec, false)
    }

    #[test]
    fn test_4() {
        let ts = Ts::new(
            vec![1, 2],
            vec![1],
            vec![(1, vec!['A'])],
            vec![(1, vec![('a', 2)])],
        );
        let spec = Mu::Ex('a', Box::new(Mu::Lit('A')));
        test_spec(&ts, &spec, false)
    }

    #[test]
    fn test_5() {
        let ts = Ts::new(
            vec![1, 2],
            vec![1],
            vec![(1, vec!['A'])],
            vec![(1, vec![('a', 2)])],
        );
        let spec = Mu::Ex('b', Box::new(Mu::Lit('A')));
        test_spec(&ts, &spec, false)
    }

    #[test]
    fn test_6() {
        let ts = Ts::new(
            vec![1, 2, 3],
            vec![1],
            vec![(3, vec!['A'])],
            vec![(1, vec![('a', 2), ('b', 3)])],
        );
        let spec = Mu::Ex('b', Box::new(Mu::Lit('A')));
        test_spec(&ts, &spec, true)
    }

    #[test]
    fn test_7() {
        let ts = Ts::new(
            vec![1, 2, 3],
            vec![1],
            vec![(1, vec!['A']), (2, vec!['B']), (3, vec!['C'])],
            vec![
                (1, vec![('a', 2)]),
                (2, vec![('a', 3)]),
                (3, vec![('a', 1)]),
            ],
        );
        let phi = Mu::Or(Box::new(Mu::Lit('B')), Box::new(Mu::Lit('C')));
        let phi = Mu::Or(Box::new(Mu::Lit('A')), Box::new(phi));
        let phi = Mu::And(Box::new(Mu::Var("X".to_string())), Box::new(phi));
        let spec = Mu::Gfp("X".to_string(), Box::new(phi));
        println!("{:?}", ts.sat(&spec, HashMap::new()));
        test_spec(&ts, &spec, true);
    }

    #[test]
    fn test_8() {
        let ts = Ts::new(
            vec![1, 2, 3],
            vec![1],
            vec![(1, vec!['A']), (2, vec!['B']), (3, vec!['C'])],
            vec![
                (1, vec![('a', 2)]),
                (2, vec![('a', 3)]),
                (3, vec![('a', 1)]),
            ],
        );
        let phi = Mu::Or(Box::new(Mu::Lit('B')), Box::new(Mu::Lit('C')));
        let phi = Mu::Or(Box::new(Mu::Lit('A')), Box::new(phi));
        let phi = Mu::And(Box::new(Mu::Var("X".to_string())), Box::new(phi));
        let spec = Mu::Gfp("X".to_string(), Box::new(phi));
        println!("{:?}", ts.sat(&spec, HashMap::new()));
        test_spec(&ts, &spec, true);
    }
}
