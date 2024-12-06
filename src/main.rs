use std::fs;
use std::process::exit;

fn main() {
    let file_path = "./docs.gl/gl4/glClear.xhtml";
    // returns result -> result has unwrap or else function so throw custom error
    let content = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("ERROR: could not read file {file_path}: {err}");
        exit(1)
    });
    println!("{content}");
}
