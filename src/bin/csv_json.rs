extern crate csv;
extern crate formats;

use formats::*;

static STDIN: &'static str = "/dev/stdin";

fn main() {
    // let data = io_csv::read_csv(STDIN.to_owned());
    let json = io_json::read_json(STDIN.to_owned());
    // print!("{:?}", data);
    print!("{:?}", json);
}
