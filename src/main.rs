mod ast;
mod eval;
mod parser;

use std::io::prelude::*;

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut s = String::new();
    loop {
        stdout.write("λ ".as_bytes()).unwrap();
        stdout.flush().unwrap();
        stdin.read_line(&mut s).unwrap();
        let t = match parser::parse_Term(s.trim()) {
            Ok(t) => t,
            Err(e) => { println!("Parse error: {:?}", e); continue }
        };
        println!("big_eval = {}, small_eval = {}",
                 match eval::big_eval(&*t) { Ok(o) => format!("{}", o), Err(e) => format!("{}", e) },
                 eval::small_eval(&*t));
        s.clear();
    }
}
