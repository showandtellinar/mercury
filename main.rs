extern crate mercury;

use mercury::{parse, utils};
use std::io::File;
use std::os;

fn main() {
    let path = Path::new(os::args()[1].clone());
    let mut file = (File::open(&path)).unwrap();

    utils::print_block(&parse::parse_block(&mut file, false));
}

