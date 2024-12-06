use std::fs::File;
use std::process::exit;
use xml::reader::EventReader;

fn main() {
    let file_path = "./docs.gl/gl4/glClear.xhtml";

    // returns result -> result has unwrap or else function so throw custom error
    let file = File::open("./docs.gl/gl4/glClear.xhtml").unwrap_or_else(|err| {
        eprintln!("ERROR: could not read file {file_path}: {err}");
        exit(1)
    });
    let er = EventReader::new(file);

    for event in er.into_iter() {
        println!("{event:?}")
    }
}
