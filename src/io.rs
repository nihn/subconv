extern crate encoding;

use std::io::{self, Read};
use std::io::prelude::*;
use std::iter::Iterator;
use std::fs::File;
use std::path::PathBuf;
use std::vec::Vec;

use self::encoding::DecoderTrap;
use self::encoding::label::encoding_from_whatwg_label;


fn decode_file_content(mut file: File, encoding: &str) -> Result<String, io::Error> {
    let mut content = String::new();

    if encoding == "utf-8" {
        let mut buf = io::BufReader::new(file);
        buf.read_to_string(&mut content)?;
        return Ok(content);
    }
    let encoder = encoding_from_whatwg_label(encoding).expect("Invalid encoding!");
    let mut bytes: Vec<u8> = vec!();
    file.read_to_end(&mut bytes).unwrap();
    encoder.decode_to(&bytes, DecoderTrap::Replace, &mut content)
        .expect("Failed to decode file content");
    Ok(content.replace("\r", ""))
}


pub fn get_lines(file_name: &str, encoding: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_name)?;
    let content = decode_file_content(file, encoding)?;
    let lines = content.split("\n");
    let lines: Vec<String> = lines.map(|l|l.to_string()).collect();
    Ok(lines)
}


pub fn get_target_name(file_name: &str) -> String {
    let mut i = 0;
    let mut file = PathBuf::from(file_name);
    file.set_extension("srt");
    let orig_name: String;
    {
        let name = file.file_stem();
        orig_name = String::from(name.unwrap().to_str().unwrap());
    };

    while file.exists() {
        let new_name = format!("{}_{}.srt", orig_name, i);
        i += 1;
        file.set_file_name(new_name);
    }
    String::from(file.to_str().unwrap())
}


pub fn save_lines(file_name: &str, lines: String) -> io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(lines.as_bytes())?;
    file.sync_all()?;
    Ok(())
}
