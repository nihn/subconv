extern crate regex;

use self::regex::Regex;
use std::iter::Iterator;
use std::vec::Vec;


pub fn convert(lines: Vec<String>) -> String {
    let re = Regex::new(r"\{(\d+)\}\{(\d+)\}(.*)").unwrap();
    let mut new_lines = String::new();
    let mut rate = 23.375;

    for (i, line) in lines.iter().enumerate() {
        let line_str = &line.to_string();
        let cap = match re.captures(line_str) {
            Some(cap) => cap,
            None => continue,
        };
        if i == 0 {
            rate = match cap[3].parse::<f64>() {
                Ok(rate) => {
                    println!("Frame rate detected: {}", rate);
                    rate
                },
                Err(_) => rate,
            };
        }
        let start: u16 = cap[1].parse().unwrap();
        let end: u16 = cap[2].parse().unwrap();
        let text = &cap[3];
        new_lines.push_str(&(i + 1).to_string());
        new_lines.push_str("\n");
        new_lines.push_str(&format_time(start, end, rate));
        new_lines.push_str("\n");
        new_lines.push_str(&text.to_string());
        new_lines.push_str("\n\n");
    }
    new_lines
}


fn frame_to_time(timestamp: u16, rate: f64) -> String {
    let seconds = timestamp as f64 / rate;
    let full_seconds = seconds as u16;
    let hours = full_seconds / 3600;
    let minutes = (full_seconds - hours * 60) / 60;
    let seconds = seconds - (minutes * 60 + hours * 3600) as f64;
    format!("{:02}:{:02}:{:02.3}", hours, minutes, seconds).replace(".", ",")
}


fn format_time(start: u16, end: u16, rate: f64) -> String {
    let start = frame_to_time(start, rate);
    let end = frame_to_time(end, rate);
    let formatted = format!("{} --> {}", start, end);
    formatted
}
