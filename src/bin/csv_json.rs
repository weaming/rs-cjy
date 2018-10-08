extern crate csv;
extern crate formats;

use formats::*;

static STDIN: &'static str = "/dev/stdin";

fn main() {
    let data = io_csv::read_csv(STDIN.to_owned());
    print!("{:?}", data);
}
