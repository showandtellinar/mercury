pub use self::types::*;

pub mod types { 
    #[allow(dead_code)]
    pub struct Block {
        pub version : u32,
        pub prev_block : Vec<u8>,
        pub merkle_root : Vec<u8>,
        pub timestamp : u32,
        pub bits : u32,
        pub nonce : u32,
        pub txn_count : u64,
        pub txns : Vec<Transaction>
    }

    #[allow(dead_code)]
    pub struct Transaction {
        pub version : u32,
        pub tx_in_count : u64,
        pub tx_in : Vec<TxIn>,
        pub tx_out_count : u64,
        pub tx_out : Vec<TxOut>,
        pub lock_time : u32
    }

    #[allow(dead_code)]
    pub struct TxIn {
        pub previous_output : OutPoint,
        pub script_length : u64,
        pub signature_script : Vec<u8>,
        pub sequence : u32
    }

    #[allow(dead_code)]
    pub struct TxOut {
        pub value : i64,
        pub pk_script_length : u64,
        pub pk_script : Vec<u8>
    }

    #[allow(dead_code)]
    pub struct OutPoint {
        pub hash : Vec<u8>,
        pub index : u32
    }
}
