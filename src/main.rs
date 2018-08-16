extern crate clap;

use clap::{Arg, App};
use std::vec::Vec;

mod convert;
mod io;

fn main() {
    let matches = App::new("Subtitles converter")
        .version("0.1")
        .arg(Arg::with_name("encoding")
           .short("e")
           .long("encoding")
           .help("Source subtitles encoding")
           .takes_value(true))
        .arg(Arg::with_name("fps")
           .long("fps")
           .help("Frame rate of the subtitles, will be deducted from file if possible")
           .takes_value(true))
        .arg(Arg::with_name("files")
           .required(true)
           .multiple(true)
           .help("Files to convert"))
        .get_matches();
    let files: Vec<_> = matches.values_of("files").unwrap().collect();
    let encoding = matches.value_of("encoding").unwrap_or("utf-8");
    let fps: f64 = matches.value_of("fps").unwrap_or("23.375")
        .parse().expect("Invalid fps argument");

    for file in files {
        println!("Processing {}", file);
        let lines = match io::get_lines(file, encoding) {
            Ok(res) => res,
            Err(error) => {
                println!("Failed to read {}: {}", file, error);
                continue;
            }
        };
        let new_lines = convert::convert(lines, fps);
        let new_file = io::get_target_name(file);
        println!("Saving {} as {}", file, new_file);
        match io::save_lines(&new_file, new_lines) {
            Ok(_) => println!("Save succeeded!"),
            Err(e) => println!("Save failed: {}", e),
        };
    }
}
