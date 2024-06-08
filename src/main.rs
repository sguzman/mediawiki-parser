use crate::schema::data::Mediawiki;
use indicatif::{ProgressBar, ProgressStyle};
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;
use serde_xml_rs::from_str;
use std::fs::File;
use std::io::BufReader;

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
    let mut xml_content = String::new();

    loop {
        match xml_reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                // Handle start elements here
                if e.name() == QName(b"page") {
                    // Add logic for handling `page` element if needed
                }
            }
            Ok(Event::Text(e)) => {
                if let Ok(text) = e.unescape() {
                    xml_content.push_str(&text);
                }
            }
            Ok(Event::End(ref e)) => {
                // Handle end elements here
                if e.name() == QName(b"mediawiki") {
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
        pb.set_position(xml_reader.buffer_position() as u64);
        buf.clear();
    }

    pb.finish_with_message("Reading complete, starting validation");

    // Deserialize the collected XML content into the Mediawiki struct
    match from_str::<Mediawiki>(&xml_content) {
        Ok(mediawiki) => {
            // Perform validation here
            println!("Parsed successfully: {:?}", mediawiki);
            // Add your validation logic here
            for page in mediawiki.page {
                // Example validation: check if the page title is empty
                if page.title.is_empty() {
                    println!("Validation error: Page title is empty.");
                }
                // Add more validation rules as needed
            }
        }
        Err(e) => {
            println!("Failed to parse XML: {:#?}", e);
        }
    }

    pb.finish_with_message("Processing complete");
    Ok(())
}
