use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::path::Path;
use xml::reader::{EventReader, XmlEvent};

pub fn parse_lyr_file(path: &str) -> Result<Vec<String>, Error> {
    if !fs::metadata(path).is_ok() {
        return Err(Error::new(ErrorKind::NotFound, "File not found"));
    }

    let path_obj: &Path = Path::new(path);
    if path_obj.extension().and_then(|s: &OsStr| s.to_str()) != Some("lyr") {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid file extension",
        ));
    }

    let file: File = File::open(path)?;
    let file: BufReader<File> = BufReader::new(file);
    let parser: EventReader<BufReader<File>> = EventReader::new(file);
    let mut ids: Vec<String> = Vec::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name == "Object" {
                    for a in attributes {
                        if a.name.local_name == "Id" {
                            ids.push(a.value);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    return Ok(ids);
}
