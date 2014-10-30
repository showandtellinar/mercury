extern crate mercury;

use mercury::{parse};
use std::io::File;
use std::os;

fn main() {
    let path = Path::new(os::args()[1].clone());
    let mut file = (File::open(&path)).unwrap();

    parse::parse_file(&mut file, false);
}

