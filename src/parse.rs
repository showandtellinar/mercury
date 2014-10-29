pub use self::parse::*;

pub mod parse {
    use types;
    use std::io::File;

    macro_rules! unwrap(
        ($inp: expr) => (
            match $inp {
                Ok(r) => r,
                Err(e) => fail!("{}",e)
            }
        );
    )

    macro_rules! vprintln(
        ($flag: ident, $msg: expr, $($args:expr)*) => (
            if $flag { println!($msg, $($args)*); }
        );
    )

    #[allow(dead_code)]
    fn string_of_hex(hex: & Vec<u8>) -> Vec<String>{
        hex.iter().map(|h| {
            let base16 = ['0', '1', '2', '3', 
                          '4', '5', '6', '7', 
                          '8', '9', 'A', 'B', 
                          'C', 'D', 'E', 'F'];
            let mut s = String::new();
            s.push(base16[(h/16) as uint]);
            s.push(base16[(h%16) as uint]);
            s
        }).collect()
    }

    #[allow(unused_variable)]
    pub fn parse_block(file: &mut File, verbose: bool) -> types::Block {
        verify_block(file);

        let header = unwrap!(file.read_le_u32());
        let version = unwrap!(file.read_le_u32());
        let sha = unwrap!(file.read_exact(32));
        let merkle = unwrap!(file.read_exact(32));
        let timestamp = unwrap!(file.read_le_u32());
        let difficulty = unwrap!(file.read_le_u32());
        let nonce = unwrap!(file.read_le_u32());
        let transaction_count = read_vli(file);

        let mut txns = Vec::new();
        for _ in range(0, transaction_count) {
            let transaction_version = unwrap!(file.read_le_u32());

            let input_count = read_vli(file);
            let mut txns_in = Vec::new();
            for _ in range(0, input_count) {
                let transaction_hash = unwrap!(file.read_exact(32));
                let transaction_index = unwrap!(file.read_le_u32());
                let script_length = read_vli(file);
                let script = unwrap!(file.read_exact(script_length as uint));
                let sequence_number = unwrap!(file.read_le_u32());
                assert!(sequence_number == 0xFFFFFFFFu32);

                txns_in.push( types::TxIn {
                    previous_output : types::OutPoint { 
                        hash : transaction_hash, 
                        index : transaction_index },
                    script_length : script_length,
                    signature_script : script,
                    sequence : sequence_number
                });
            }

            let output_count = read_vli(file);
            let mut txns_out = Vec::new();
            for _ in range(0, output_count) {
                let value = unwrap!(file.read_le_i64());
                let script_length = read_vli(file);
                let script = unwrap!(file.read_exact(script_length as uint));
                txns_out.push( types::TxOut {
                    value : value,
                    pk_script_length : script_length,
                    pk_script : script
                });
            }

            let lock_time = unwrap!(file.read_le_u32());

            txns.push( types::Transaction {
                version : transaction_version,
                tx_in_count : input_count,
                tx_in : txns_in,
                tx_out_count : output_count,
                tx_out : txns_out,
                lock_time : lock_time
            });
        }

        types::Block {
            version : version,
            prev_block : sha, 
            merkle_root : merkle,
            timestamp : timestamp,
            bits : difficulty,
            nonce : nonce,
            txn_count : transaction_count,
            txns : txns
        }
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
        let magic_uint : u32 = 0xD9B4BEF9;
        let file_uint = unwrap!(file.read_le_u32());
        assert!(magic_uint == file_uint);
    }
}
