extern crate mercury;

use mercury::parse;
use std::io::File;

fn main() {
    let path = Path::new("/home/alex/.bitcoin/blocks/blk00000.dat");
    let mut file = (File::open(&path)).unwrap();

    parse::parse_block(&mut file, false);
}

