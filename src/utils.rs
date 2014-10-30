pub use self::utils::*;

pub mod utils {
    use types;

    /*macro_rules! printvec(
        ($inp:expr, $n:expr) => (
            for (a,b) in $inp.iter().zip($inp
        );
    )*/

    pub fn print_block(block : &types::Block) {
        println!("version {}", block.version);
        //printvec!(string_of_hex(block.prev_block));
        println!("txn_count {}", block.txn_count);
        for txn in block.txns.iter() {
            print_transaction(txn);
        }
    }

    fn print_transaction(txn: &types::Transaction) {
        println!("transaction {} ({}/{})", 
                 txn.version, 
                 txn.tx_in_count, 
                 txn.tx_out_count);
        for tx_in in txn.tx_in.iter() {
            println!("-> {}", string_of_hex(&tx_in.signature_script));
        }
        for tx_out in txn.tx_out.iter() {
            println!("<- {}", string_of_hex(&tx_out.pk_script));
        }

    }
    
    pub fn string_of_hex(hex: & Vec<u8>) -> Vec<String> {
        let mut hex_vec = Vec::new();
        let base16 = ['0', '1', '2', '3', 
                      '4', '5', '6', '7', 
                      '8', '9', 'A', 'B', 
                      'C', 'D', 'E', 'F'];
        for h in hex.iter() {
            let mut hex_string = String::new();
            hex_string.push(base16[(h/16) as uint]);
            hex_string.push(base16[(h%16) as uint]);
            hex_vec.push(hex_string);
        }
        return hex_vec;
    }

}
