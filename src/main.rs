// src/main.rs
use crate::schema::data::Schema as top;
use serde_xml_rs::from_str;
use std::fs;
use std::path::Path;

mod schema {
    pub mod data;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resource_dir = Path::new("resources");
    for entry in fs::read_dir(resource_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let content = fs::read_to_string(&path)?;
            match from_str::<top>(&content) {
                Ok(mediawiki) => {
                    println!("Parsed successfully: {:#?}", mediawiki);
                    // Add your validation logic here
                }
                Err(e) => {
                    println!("Failed to parse {}: {:#?}", path.display(), e);
                }
            }
        }
    }
    Ok(())
}
