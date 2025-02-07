use std::fmt;
use thiserror::Error;
//extern crate utility;
//extern crate maintx;
use utility;
//use std::error::Error;
use utility::hash::hash;
use utility::hash::hash::Hash;

use utility::buffer::buffer_writer::BufferWriter;
use utility::buffer::buffer_reader::BufferReader;
use utility::buffer::buffer_reader::BufferReaderError;

use utility::hash::bigint;
use std::cmp::Ordering;
use crate::mainheader::mainheader::Mainheader;
use crate::mainheader::mainheader::unserialize_mainheader;


use maintx::maintx::maintx::Maintx;
use maintx::maintx::maintx::unserialize_maintx;


use crate::mainheader::mainheader::MainheaderError;
use maintx::maintx::maintx::MaintxError;
//

#[derive(Debug, Error)]
pub enum MainblockError {
    //#[error("Failed to read transaction data: {0}")]
    //ReadError(#[from] std::io::Error),

    //#[error("Failed to deserialize transaction: {0}")]
    //DeserializeError(String),

    //#[error("Failed to unserialize transaction - BufferReaderError error: {0}")]
    //UnserializeError(#[from] BufferReaderError),
    #[error("Buffer reader error: {0}")]
    BufferReaderError(#[from] BufferReaderError),

    #[error("Mainheader error: {0}")]
    MainheaderError(#[from] MainheaderError),

    #[error("Maintx error: {0}")]
    MaintxError(#[from] MaintxError),

}

#[derive(Clone)] 
pub struct Mainblock {
    pub header: Mainheader,
    pub transactions: Vec<Maintx>,
}

impl Mainblock {
    pub fn new(header: Mainheader, transactions: Vec<Maintx>) -> Mainblock {
        Mainblock {
            header,
            transactions,
        }
    }
    /*
    pub fn check_hash(&self) -> bool {
        self.header.check_hash()
    }
    
    pub fn check_target(&self) -> bool {
        self.header.check_target()
        //
        //if !(self.header).check_target() {
        //    println!("Invalid Target of mainheader");
        //    return false
        //} else {
        //    println!("Valid Target of mainheader")
        //}
        //
        //return true
        //
    }*/
    //
    pub fn get_hash(&self) -> Hash {
        self.header.get_hash()
    }
    //
    pub fn get_mainheader(&self)-> Mainheader {
        self.header.clone()
    }
    //
    pub fn serialize(&self) -> Vec<u8> {
        let tmpmb=self;
        let mut bw = BufferWriter::new();
        let mainheader_rawbytes=tmpmb.header.serialize();
        //bw.put_var_u64(mainheader_rawbytes.len() as u64);
        //bw.put_bytes(mainheader_rawbytes);
        bw.put_var_bytes(&mainheader_rawbytes);

        bw.put_var_u64(tmpmb.transactions.len() as u64);
        for i in 0..tmpmb.transactions.len() { 
            let transaction_rawbytes=tmpmb.transactions[i].serialize();
            //bw.put_var_u64(transaction_rawbytes.len() as u64);
            //bw.put_bytes(&transaction_rawbytes);
            bw.put_var_bytes(&transaction_rawbytes);
        }
        let content = bw.get_bytes();
        content
    }
    //
}
//

pub fn unserialize_mainblock(rawbytes: Vec<u8>) -> Result<Mainblock, MainblockError> {
    let mut br = BufferReader::new(rawbytes);

    //let tmpmainheaderlen=br.get_var_u64()?;
    //let tmpmainheader_rawbytes=br.get_bytes(tmpmainheaderlen as u32)?;
    let tmpmainheader_rawbytes=br.get_var_bytes()?;
    let tmpheader=unserialize_mainheader(tmpmainheader_rawbytes)?;
    let mut mb = Mainblock {
        header:tmpheader,
        transactions: Vec::new(), 
    };

    let tmptransactionslen=br.get_var_u64()?;
    for i in 0..tmptransactionslen { 
        let tmptransactionlen=br.get_var_u64()?;
        let tmptransaction_rawbytes=br.get_bytes(tmptransactionlen as u32)?;
        let tmptransaction=unserialize_maintx(tmptransaction_rawbytes)?;
        mb.transactions.push(tmptransaction);
    }


    Ok(mb)
}

pub fn compute_hash_mainblock_info(version: u32,prev_hash: Hash,root_hash: Hash,timestamp: i64,bits: u32,nonce: u32) -> Hash{
    //a transaction hash is a signing hash, it does not include a signature
    //TODOLATER make it more efficient using pointer and not clone
    let mut bw = BufferWriter::new();
    bw.put_u32(version as u32);
    //bw.put_var_u64(tmptx.vin.len() as u64);
    bw.put_hash(prev_hash.clone());
    bw.put_hash(root_hash.clone());
    bw.put_u64(timestamp as u64);
    bw.put_u32(bits);
    bw.put_u32(nonce);
    let content = bw.get_bytes();

    let tmphash=Hash::compute_hash(content.as_slice());
    tmphash
}

impl fmt::Debug for Mainblock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mainblock")
            //.field("height", &self.height)
            .field("header", &format!("{:?}", &self.header))
            .field("transactions", &self.transactions)
            .finish()
    }
}


