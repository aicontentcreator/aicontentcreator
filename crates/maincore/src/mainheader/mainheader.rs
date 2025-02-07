use std::fmt;
//extern crate utility;
//extern crate maintx;

use thiserror::Error;

use utility::hash::hash::Hash;

use utility::buffer::buffer_writer::BufferWriter;
use utility::buffer::buffer_reader::BufferReader;

use utility::buffer::buffer_reader::BufferReaderError;
use utility::hash::bigint;
use std::cmp::Ordering;

use utility::system::random;

#[derive(Debug, Error)]
pub enum MainheaderError {
    //#[error("Failed to read transaction data: {0}")]
    //ReadError(#[from] std::io::Error),

    //#[error("Failed to deserialize transaction: {0}")]
    //DeserializeError(String),

    //#[error("Failed to unserialize transaction - BufferReaderError error: {0}")]
    //UnserializeError(#[from] BufferReaderError),
    #[error("Buffer reader error: {0}")]
    BufferReaderError(#[from] BufferReaderError),
    #[error("Mining unsuccessful")]
    MiningUnsuccessful,

}

#[derive(Clone)] 
pub struct Mainheader {
    version: u32,
    prev_hash: Hash,
    root_hash: Hash,
    timestamp: i64,
    bits: u32,
    nonce: u32,
    hash: Hash,
}

impl Mainheader {
    pub fn new(
        version: u32,
        prev_hash: Hash,
        root_hash: Hash,
        timestamp: i64,
        bits: u32,
        nonce: u32,
        hash: Hash,
    ) -> Mainheader {
        Mainheader {
            version,
            prev_hash,
            root_hash,
            timestamp,
            bits,
            nonce,
            hash,
        }
    }

    //
    //
    pub fn get_version(&self) -> u32 {
        self.version
    }
    pub fn get_prev_hash(&self) -> Hash {
        self.prev_hash.clone()
    }
    pub fn get_root_hash(&self) -> Hash {
        self.root_hash.clone()
    }
    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
    pub fn get_bits(&self) -> u32 {
        self.bits
    }
    pub fn get_nonce(&self) -> u32 {
        self.nonce
    }
    pub fn get_hash(&self) -> Hash {
        self.hash.clone()
    }
    //
    pub fn check_hash(&self) -> bool {
        println!("Mainheader check_hash              {:?}",self.hash.clone());
        println!("Mainheader check_hash compute_hash {:?}",self.compute_hash());
        let tmp_hash=self.compute_hash();
        self.hash.clone()==tmp_hash
    }
    //
    pub fn check_target(&self) -> bool {
        // compare hash with target
        let bitsbigint=bigint::biguint_from_compact(self.bits);
        let hashbigint=bigint::biguint_from_hash(&self.hash.clone());
        println!("Mainheader check_target bigint      {:?}",bitsbigint);
        println!("Mainheader check_target hash bigint {:?}",hashbigint);
        // Compare a and b
        let comparison = hashbigint.cmp(&bitsbigint);
        match comparison {
            Ordering::Less => return true,//println!("hashbigint is less than bitsbigint"),
            //Ordering::Equal => println!("hashbigint is equal to bitsbigint"),
            _ => return false,//println!("hashbigint is greater or equal to bitsbigint"),
        }
    }
    //
    pub fn compute_hash(&self)-> Hash {
        //a transaction hash is a signing hash, it does not include a signature
        //TODOLATER make it more efficient using pointer and not clone
        let mut bw = BufferWriter::new();
        bw.put_var_u32(self.version as u32);
        //bw.put_var_u64(tmptx.vin.len() as u64);
        bw.put_hash(self.prev_hash.clone());
        bw.put_hash(self.root_hash.clone());
        bw.put_u64(self.timestamp as u64);
        bw.put_u32(self.bits);
        bw.put_u32(self.nonce);
        let content = bw.get_bytes();

        let tmphash=Hash::compute_hash(content.as_slice());
        tmphash 
    }
    //
    pub fn serialize(&self) -> Vec<u8> {
        let tmpmh=self;
        let mut bw = BufferWriter::new();
        bw.put_var_u32(tmpmh.version as u32);
        bw.put_hash(tmpmh.prev_hash.clone());
        bw.put_hash(tmpmh.root_hash.clone());
        bw.put_u64(tmpmh.timestamp as u64);
        bw.put_u32(tmpmh.bits);
        bw.put_u32(tmpmh.nonce);
        bw.put_hash(tmpmh.hash.clone());
        let content = bw.get_bytes();
        content
    }    
    //
    ///////////////////

    
}

pub fn unserialize_mainheader(rawbytes: Vec<u8>) -> Result<Mainheader, MainheaderError> {
    let mut br = BufferReader::new(rawbytes);
    let tmpversion=br.get_var_u32()?;
    let tmpprev_hash=br.get_hash()?;
    let tmproot_hash=br.get_hash()?;
    let tmptimestamp=br.get_u64()?;
    let tmpbits=br.get_u32()?;
    let tmpnonce=br.get_u32()?;
    let tmphash=br.get_hash()?;
    let mut mh = Mainheader {
        version:tmpversion as u32,
        prev_hash:tmpprev_hash,
        root_hash:tmproot_hash,
        timestamp:tmptimestamp as i64,
        bits:tmpbits,
        nonce:tmpnonce,
        hash:tmphash,
    };
    Ok(mh)
}

impl fmt::Debug for Mainheader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mainheader")
            .field("version", &self.version)
            .field("prev_hash", &format!("{:?}", &self.prev_hash))
            .field("root_hash", &format!("{:?}", &self.root_hash))
            .field("timestamp", &self.timestamp)
            .field("bits", &self.bits)
            .field("nonce", &self.nonce)
            .field("hash", &format!("{:?}", &self.hash))
            .finish()
    }
}

pub fn mine_mainheader_with_cpu(version: u32,prev_hash: Hash,root_hash: Hash,timestamp: i64,bits: u32) -> Result<Mainheader, MainheaderError> {
    let test: u64 =1;
    let mut bw = BufferWriter::new();
    bw.put_var_u32(version as u32);
    //bw.put_var_u64(tmptx.vin.len() as u64);
    bw.put_hash(prev_hash.clone());
    bw.put_hash(root_hash.clone());
    bw.put_u64(timestamp as u64);
    bw.put_u32(bits);
    
    let mut nonce:u32=1;
 
    for num in 1..=429490 {//max (too high) 4294967295//TODO optimize//TODO use search_iterations_count
        //println!("Current number: {}", num);
        //nonce+=1;
        nonce=random::generate_random_number(0, 4294967295).unwrap() as u32;
        let mut newbw=bw.clone();
        newbw.put_u32(nonce);
        let content = newbw.get_bytes();
        let tmphash=Hash::compute_hash(content.as_slice());
        
        // compare hash with target
        let bitsbigint=bigint::biguint_from_compact(bits.clone());
        let hashbigint=bigint::biguint_from_hash(&tmphash.clone());
        //println!("genesis_block bigint      {:?}",bitsbigint);
        //println!("genesis_block hash bigint {:?}",hashbigint);
        // Compare a and b
        
        let comparison = hashbigint.cmp(&bitsbigint);
        match comparison {
            Ordering::Less => { let tmpmainheader=Mainheader::new(version, prev_hash.clone(),root_hash.clone(), timestamp, bits, nonce,tmphash.clone());
                return Ok(tmpmainheader)},//{return true},//println!("hashbigint is less than bitsbigint"),
            _ => continue,//println!("hashbigint is greater or equal to bitsbigint"),
        }
        /////////////////////////////////////////
        //let tmpmainheader=Mainheader::new(version, prev_hash.clone(),root_hash.clone(), timestamp, bits, nonce,tmphash.clone());
        //return Ok(tmpmainheader)
        /////////////////////////////////////////
        
    }
    return Err(MainheaderError::MiningUnsuccessful)//Err("mining unsuccessful".into())    
}
