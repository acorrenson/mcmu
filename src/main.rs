use mu_calc::{mu::Mu, ts::Ts};

fn main() {
    let ts = Ts::new(vec![1, 2], vec![1], vec![(1, vec![('a', 2)])]);
    let spec = Mu::All('b', Box::new(Mu::Lit(2)));

    println!("system:\n{}", ts);
    println!("spec:\n{}", spec);
    println!("correct:{}", ts.check(&spec))
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_spec(ts: &Ts<char>, spec: &Mu<char, u32>, expect: bool) {
        println!("system:\n{}", ts);
        println!("spec:\n{}", spec);
        let res = ts.check(spec);
        println!("correct:{}", res);
        assert_eq!(res, expect)
    }

    #[test]
    fn test_1() {
        let ts = Ts::new(vec![1, 2], vec![1], vec![(1, vec![('a', 2)])]);
        let spec = Mu::All('b', Box::new(Mu::Lit(2)));
        test_spec(&ts, &spec, true)
    }

    #[test]
    fn test_2() {
        let ts = Ts::new(vec![1, 2], vec![1], vec![(1, vec![('a', 2)])]);
        let spec = Mu::All('a', Box::new(Mu::Lit(2)));
        test_spec(&ts, &spec, true)
    }

    #[test]
    fn test_3() {
        let ts = Ts::new(vec![1, 2], vec![1], vec![(1, vec![('a', 2)])]);
        let spec = Mu::All('a', Box::new(Mu::Lit(1)));
        test_spec(&ts, &spec, false)
    }

    #[test]
    fn test_4() {
        let ts = Ts::new(vec![1, 2], vec![1], vec![(1, vec![('a', 2)])]);
        let spec = Mu::Ex('a', Box::new(Mu::Lit(1)));
        test_spec(&ts, &spec, false)
    }

    #[test]
    fn test_5() {
        let ts = Ts::new(vec![1, 2], vec![1], vec![(1, vec![('a', 2)])]);
        let spec = Mu::Ex('b', Box::new(Mu::Lit(1)));
        test_spec(&ts, &spec, false)
    }

    #[test]
    fn test_6() {
        let ts = Ts::new(vec![1, 2, 3], vec![1], vec![(1, vec![('a', 2), ('b', 3)])]);
        let spec = Mu::Ex('b', Box::new(Mu::Lit(3)));
        test_spec(&ts, &spec, true)
    }
}
