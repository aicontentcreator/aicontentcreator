//extern crate utility;
/*
use utility::bytesfile;
use utility::bufferreader::BufferReader;
use utility::hashing::Hash;
use utility::hashing::compute_root;
use utility::utility::generate_random_number;
use std::error::Error;
use crate::mainblock::Mainblock;
use crate::mainheader::Mainheader;
use crate::mainheader;


use tx::tx::Transaction;
*/
//extern crate ctrlc;
//extern crate rand;
//extern crate chrono;
use thiserror::Error;

use utility::hash::hash::Hash;
use utility::hash::tree::compute_root;
use maintx::maintx::maintx::Maintx;

//use std::error::Error;
use crate::mainblock::mainblock::Mainblock;
use crate::mainheader::mainheader::Mainheader;
use crate::mainheader::mainheader::MainheaderError;

//use crate::mainheader;
use crate::maincore_inner::constants::mainblock_reward;
use crate::mainheader::mainheader::mine_mainheader_with_cpu;
use maintx::maintx::maintx::new_reward_transaction;



use utility::system::time::timestamp_now;
use utility::system::time::format_timestamp_to_gmt_string;
use std::process;
use utility::system::random::generate_random_number;


#[derive(Debug, Error)]
pub enum MinerError {
    //#[error("I/O error: {0}")]
    //IoError(#[from] io::Error),

    //#[error("Custom error: {0}")]
    //Custom(String),

    //#[error("BigInt error: {0}")]
    //BigIntError(String),

    //#[error("Chunk storage error: {0}")]
    //ChunkStorageError(String),

    //#[error("Header serialization error: {0}")]
    //SerializationError(String),
    //////////////////////////////////////////////
    /////////////////////////////////////////////
    //#[error("Buffer reader error: {0}")]
    //BufferReaderError(#[from] BufferReaderError),
    /*
    #[error("Storage directory error: {0}")]
    StorageDirectoryError(#[from] StorageDirectoryError),
    #[error("Mainblock error: {0}")]
    MainblockError(#[from] MainblockError),
    #[error("UnconfirmedMainblocksError error: {0}")]
    UnconfirmedMainblocksError(#[from] UnconfirmedMainblocksError),
    #[error("{0}")]
    OtherError(String),
    */
    #[error("Mainheader error: {0}")]
    MainheaderError(#[from] MainheaderError),
}

pub struct Miner {
    addresses_vector: Vec<Hash>,
}


impl Miner {
    pub fn new() -> Miner {
        Miner { addresses_vector: Vec::new() }
    }
    pub fn add_address(&mut self,tmp_address: Hash){
        self.addresses_vector.push(tmp_address);
    }
    //fn mine_mainblock_with_cpu(&self) -> Result<Mainblock, Box<dyn Error>> {
    // self.miner.mine_mainblock(tmp_last_inmem_header,tmpheight,tmp_newbits,tmp_prioritized_txs.clone(),total_fees) {
    pub fn mine_mainblock(&self,prev_mainheader: Mainheader,height:u32,newbits:u32,mut tmp_txs:Vec<Maintx>,total_fees:u64) -> Result<Mainblock,MinerError>{
        //let prev_mainheader=prev_mainblock.get_mainheader();
        println!("------> txs to be mined {:?}",tmp_txs);
        let num_addresses=self.addresses_vector.len();
        let random_number=generate_random_number(1,num_addresses).unwrap()-1;
        let newaddress=self.addresses_vector[random_number].clone();
        println!("prev_mainheader {:?}",prev_mainheader);

        let tmprewardvalue:u64;
        tmprewardvalue=mainblock_reward(height);//1000;
        
        let newtx = new_reward_transaction(height as u32,tmprewardvalue,total_fees,newaddress);

        //
        //let new_root_hash=newtx.compute_hash();
        let mut tmp_hashes=Vec::new();
        tmp_hashes.push(newtx.compute_hash());
        
        for tmp_tx in &tmp_txs {
           tmp_hashes.push(tmp_tx.compute_hash());
        }
        let new_root_hash=compute_root(tmp_hashes.as_slice());
        //let newtimestamp=utility::timestamp_now();//TOFINALIZE
        let newtimestamp=prev_mainheader.get_timestamp()+60;
        println!("/////timestamp delta time {}", newtimestamp-prev_mainheader.get_timestamp());
        /*
        if newtimestamp-prev_mainheader.get_timestamp()==0 {
            process::exit(1);
        }
        */
        
        
        let newverison=1;
        let result=mine_mainheader_with_cpu(newverison, prev_mainheader.get_hash(), new_root_hash.clone(), newtimestamp, newbits);

        match result {
            Ok(new_mainheader) => {
                    println!("Mined Mainheader is {:?}", new_mainheader);
                    if !new_mainheader.check_target() {
                        println!("Invalid Target of mainheader")
                    } else {
                        println!("Valid Target of mainheader")
                    }
                    new_mainheader.check_hash();
                    //
                    //let txs: Vec<tx::tx::Transaction> = vec![newtx];
                    //println!("******");
                    tmp_txs.insert(0, newtx);
                    let new_mainblock=Mainblock::new(new_mainheader.clone(),tmp_txs);
                    
                    println!("new_mainblock {:?}",new_mainblock);
                    Ok(new_mainblock)

                    //

                },
            Err(err) => {
                    println!("Mine Error: {}", err);
                    Err(MinerError::MainheaderError(err))
                    },
        }
    }

}
