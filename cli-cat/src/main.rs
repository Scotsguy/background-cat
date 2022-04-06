use std::path::Path;

use background_cat::common_mistakes;

fn main() {
    let arg: String = std::env::args().nth(1).unwrap();

    let log = std::fs::read_to_string(Path::new(&arg)).unwrap();

    println!("{:?}", common_mistakes(&log));
}
