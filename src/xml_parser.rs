use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

pub fn parse_file(path: &str) {
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                println!("{}: {:?}", name.local_name, attributes);
            }
            Ok(XmlEvent::Characters(s)) => {
                println!("{}", s);
            }
            Ok(XmlEvent::EndElement { name }) => {
                println!("End: {}", name.local_name);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            _ => {}
        }
    }
}
