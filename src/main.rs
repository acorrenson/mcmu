use std::{env::args, fs};

use mu_calc::lang::Prog;

fn run(filename: &str) -> Result<bool, String> {
    let input = fs::read_to_string(filename).map_err(|err| format!("{}", err))?;
    let prog = input.parse::<Prog>()?;
    let ts = prog.compile()?;
    Ok(ts.check())
}

fn main() {
    let file = args().collect::<Vec<String>>()[1].clone();
    match run(file.as_str()) {
        Ok(b) => println!("Result of the verification: {}", b),
        Err(err) => eprintln!("Verification failed: {}", err),
    }
}
