#![feature(macro_rules)]
#![allow(unused_variable)]
use std::io::{File};

macro_rules! unwrap(
    ($inp: expr) => (
        match $inp {
            Ok(r) => r,
            Err(e) => fail!("{}",e)
        }
    );
)

fn main() {
    let path = Path::new("/home/alex/.bitcoin/blocks/blk00000.dat");
    let mut file = unwrap!(File::open(&path));

    verify_block(&mut file);
    parse_block(&mut file);
}

fn parse_block(file: &mut File) {
    let header = unwrap!(file.read_le_u32());
    //println!("header={}",header);
    
    let version = unwrap!(file.read_le_u32());
    println!("version={}",version);

    let sha = unwrap!(file.read_exact(32));
    //println!("sha={}",sha);
    
    let merkle = unwrap!(file.read_exact(32));
    //println!("merkle={}",merkle);

    let timestamp = unwrap!(file.read_le_u32());
    println!("timestamp={}",timestamp);

    let difficulty = unwrap!(file.read_le_u32());
    //println!("difficulty={}",difficulty);

    let nonce = unwrap!(file.read_le_u32());
    println!("nonce={}",nonce);

    let transaction_count = read_vli(file);
    println!("transaction_count={}",transaction_count);

    for _ in range(0, transaction_count) {
        let transaction_version = unwrap!(file.read_le_u32());

        let input_count = read_vli(file);
        println!("{} inputs", input_count);
        for _ in range(0, input_count) {
            let transaction_hash = unwrap!(file.read_exact(32));
            println!("tx hash={}",transaction_hash);
            let transaction_index = unwrap!(file.read_le_u32());
            let script_length = read_vli(file) as uint; // is this okay?
            let script = unwrap!(file.read_exact(script_length));
            let sequence_number = unwrap!(file.read_le_u32());
            assert!(sequence_number == 0xFFFFFFFFu32);
        }

        let output_count = read_vli(file);
        println!("{} outputs", output_count);
        for _ in range(0, output_count) {
            let value = unwrap!(file.read_le_u64());
            println!("value={}", value);
            let script_length = read_vli(file) as uint;
            let script = unwrap!(file.read_exact(script_length));
        }

        let lock_time = unwrap!(file.read_le_u32());
        assert!(lock_time == 0u32);
    }

    // Now, we should be done with the block.
    assert!(file.eof());
}

fn read_vli(file: &mut File) -> u64 {
    let flag = unwrap!(file.read_u8());
    if flag < 0xfd {
        flag as u64
    } else if flag == 0xfd {
        (unwrap!(file.read_le_u16())) as u64
    } else if flag == 0xfe {
        (unwrap!(file.read_le_u32())) as u64
    } else {
        unwrap!(file.read_le_u64())
    }
}

fn verify_block(file: &mut File) {
    print!("verifying block...");

    let magic_uint : u32 = 0xD9B4BEF9;
    let file_uint = unwrap!(file.read_le_u32());
    assert!(magic_uint == file_uint);

    println!("verify passed...");
}
