use std::{env, fs};

fn main() {
    if env::args().len() <= 1 {
        eprintln!("rdf - read a file");
        eprintln!("usage: rdf <filename>");
        return;
    }

    let filename = env::args().nth(1).unwrap();
    let content: String = fs::read_to_string(filename).unwrap();

    println!("{}", content);
}
