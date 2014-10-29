extern crate mercury;

use mercury::parse;

fn main() {
    let path = Path::new("/home/alex/.bitcoin/blocks/blk00000.dat");
    let mut file = unwrap!(File::open(&path));

    parse_block(&mut file, false);
}

