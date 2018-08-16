extern crate clap;

use clap::{Arg, App};
use std::vec::Vec;

mod convert;
mod io;

fn main() {
    let matches = App::new("Subtitles converter")
                          .version("0.01")
                          .arg(Arg::with_name("encoding")
                               .short("e")
                               .long("encoding")
                               .value_name("ENCODING")
                               .help("Generated subtitles encoding")
                               .takes_value(true))
                          .arg(Arg::with_name("files")
                               .required(true)
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .get_matches();
    let files: Vec<_> = matches.values_of("files").unwrap().collect();
    let encoding = matches.value_of("encoding").unwrap_or("utf-8");

    for file in files {
        println!("Processing {}", file);
        let lines = match io::get_lines(file, encoding) {
            Ok(res) => res,
            Err(error) => {
                println!("Failed to read {}: {}", file, error);
                continue;
            }
        };
        let new_lines = convert::convert(lines);
        let new_file = io::get_target_name(file);
        println!("Saving {} as {}", file, new_file);
        match io::save_lines(&new_file, new_lines) {
            Ok(_) => println!("Save succeeded!"),
            Err(e) => println!("Save failed: {}", e),
        };
    }
}
