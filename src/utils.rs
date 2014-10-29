pub use self::utils::print_block;

pub mod utils {
    use types;

    pub fn print_block(block : &types::Block) {
        println!("version {}", block.version);
        println!("{}", string_of_hex(&block.prev_block));
        println!("txn_count {}", block.txn_count);
    }
    
    fn string_of_hex(hex: & Vec<u8>) -> String {
        let mut hex_string = String::from_str("0x");
        let base16 = ['0', '1', '2', '3', 
                      '4', '5', '6', '7', 
                      '8', '9', 'A', 'B', 
                      'C', 'D', 'E', 'F'];
        for h in hex.iter() {
            hex_string.push(base16[(h/16) as uint]);
            hex_string.push(base16[(h%16) as uint]);
        }
        return hex_string;
    }

}
