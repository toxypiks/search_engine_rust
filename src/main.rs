use std::fs::File;
use std::io;
use std::process::exit;
use xml::reader::{EventReader, XmlEvent};

fn read_entire_xml_file(file_path: &str) -> io::Result<String> {
    // returns result ? unwrapps result
    let file = File::open(file_path)?;
    let er = EventReader::new(file);

    let mut content = String::new();
    for event in er.into_iter() {
        if let XmlEvent::Characters(text) = event.expect("TODO") {
            content.push_str(&text);
        }
    }
    Ok(content)
}

fn main() {
    let dir_path = "./docs.gl/gl4";
    /*println!(
        "{content}",
        content = read_entire_xml_file(dir_path).expect("TODO")
    );*/
}
