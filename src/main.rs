mod ast;
mod eval;
mod parser;

fn main() {
    println!("{:?}", eval::big_eval(&*parser::parse_Term("iszero 5").unwrap()));
    println!("{:?}", eval::small_eval(&*parser::parse_Term("iszero 5").unwrap()));
}
