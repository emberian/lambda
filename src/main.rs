mod ast;
mod eval;
mod parser;

use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let t = match parser::parse_Term(line.trim()) {
            Ok(t) => t,
            Err(e) => { println!("Parse error: {:?}", e); continue }
        };
        println!("big_eval = {}, small_eval = {}",
                 match eval::big_eval(&*t) { Ok(o) => format!("{}", o), Err(e) => format!("{}", e) },
                 eval::small_eval(&*t));
    }
}
