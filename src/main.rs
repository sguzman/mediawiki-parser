// src/main.rs
use crate::schema::data::Mediawiki;
use indicatif::{ProgressBar, ProgressStyle};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod schema {
    pub mod data;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "resources/example.xml";
    let file = File::open(file_path)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len();

    let pb = ProgressBar::new(file_size);
    let style = ProgressStyle::default_bar()
        .template("{msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-");
    pb.set_style(style);

    let reader = BufReader::new(file);
    let mut xml_reader = Reader::from_reader(reader);
    xml_reader.trim_text(true);

    let mut buf = Vec::new();
    let mut bytes_read = 0;

    loop {
        match xml_reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                // Handle start elements here
                if e.name() == b"page" {
                    // Add logic for handling `page` element if needed
                }
            }
            Ok(Event::End(ref e)) => {
                // Handle end elements here
                if e.name() == b"mediawiki" {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => (),
            Err(e) => {
                pb.println(format!(
                    "Error at position {}: {:?}",
                    xml_reader.buffer_position(),
                    e
                ));
                break;
            }
        }
        bytes_read = xml_reader.buffer_position() as u64;
        pb.set_position(bytes_read);
        buf.clear();
    }

    pb.finish_with_message("Processing complete");
    Ok(())
}
