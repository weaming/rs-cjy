extern crate formats;

use formats::*;
use std::process;

static STDIN: &'static str = "/dev/stdin";

fn main() {
    let text = read_file(STDIN).unwrap();
    let data = match io_json::parse_json(&text) {
        Ok(r) => r,
        Err(_) => match io_csv::parse_csv(&text) {
            Ok(r) => r,
            Err(e) => {
                println!("{:?}", e);
                process::exit(1)
            }
        },
    };
    let data = io_json::parse_json(&text);
    // let data = io_csv::read_csv(STDIN);
    println!("{:?}", data);
}
