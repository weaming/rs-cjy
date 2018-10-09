extern crate formats;

use formats::*;
use std::process;

static STDIN: &'static str = "/dev/stdin";
static STDOUT: &'static str = "/dev/stdout";

fn main() {
    let text = read_file(STDIN).unwrap();
    match io_json::parse_json(&text) {
        Ok(r) => {
            r.write_yaml(STDOUT);
        },
        Err(_) => match io_yaml::parse_yaml(&text) {
            Ok(r) => {
                r.write_json(STDOUT);
            },
            Err(e) => {
                println!("{:?}", e);
                process::exit(1)
            }
        },
    };
}
