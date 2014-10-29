extern crate mercury;

use mercury::{parse, utils};
use std::io::File;

fn main() {
    let path = Path::new("/home/alex/.bitcoin/blocks/blk00002.dat");
    let mut file = (File::open(&path)).unwrap();

    utils::print_block(&parse::parse_block(&mut file, false));
}

