use utility::hash::hash::Hash;
use utility::buffer::buffer_writer::BufferWriter;
use utility::buffer::buffer_reader::BufferReader;
use utility::buffer::buffer_reader::BufferReaderError;
//use std::error::Error;
use thiserror::Error;

use utility::statedb::statedb::Error as DbError;

//use std::io::ErrorKind;

use super::mainstate::Mainstate;

//
use super::mainstate::MAINTX_STATE_KEY_IDENTIFIER;
use super::mainstate::MAINTX_OUT_STATE_KEY_IDENTIFIER;
//
use super::mainstate::MAINTX_OUT_STATE_VALUE_IDENTIFIER_UNSPENT;
use super::mainstate::MAINTX_OUT_STATE_VALUE_IDENTIFIER_SPENT;

use maintx::maintx::maintx::Maintx;

/*



use utility::hash::tree::compute_root;


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
*/
#[derive(Debug, Error)]
pub enum MaintxStateError {
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
    #[error("Buffer reader error: {0}")]
    BufferReaderError(#[from] BufferReaderError),
    /*
    #[error("Storage directory error: {0}")]
    StorageDirectoryError(#[from] StorageDirectoryError),
    #[error("Mainblock error: {0}")]
    MainblockError(#[from] MainblockError),
    #[error("UnconfirmedMainblocksError error: {0}")]
    UnconfirmedMainblocksError(#[from] UnconfirmedMainblocksError),

    */
    //#[error("Mainheader error: {0}")]
    //MainheaderError(#[from] MainheaderError),
    //MaintxStateError
    #[error("Db error: {0}")]
    DbError(#[from] DbError),
    #[error("{0}")]
    OtherError(String),
}


impl Mainstate {
    pub async fn insert_maintx_state(& mut self,hash:Hash,maintx_mainblock_height:u32,maintx_postion:u32)->Result<(),MaintxStateError>{
        //let key: Vec<u8> = vec![1, 2, 3, 4, 5];
        //let value: Vec<u8> = vec![1, 2, 3, 4, 5];
        let mut tmp_key_bw = BufferWriter::new();
        tmp_key_bw.put_u32(MAINTX_STATE_KEY_IDENTIFIER);
        tmp_key_bw.put_hash(hash);
        let tmp_key = tmp_key_bw.get_bytes();
        let mut tmp_value_bw = BufferWriter::new();
        tmp_value_bw.put_u32(maintx_mainblock_height);
        tmp_value_bw.put_u32(maintx_postion);
        let tmp_value = tmp_value_bw.get_bytes();
        //
        let statedb = self.shared_statedb.lock().await;
        // Insert a key-value pair into the database
        statedb.insert(&tmp_key, &tmp_value)?;
        Ok(())
    }
    pub async fn get_maintx_state(& mut self,hash:Hash)->Result<(u32,u32),MaintxStateError> {
        //let key: Vec<u8> = vec![1, 2, 3, 4, 5];
        //let value: Vec<u8> = vec![1, 2, 3, 4, 5];
        let mut tmp_key_bw = BufferWriter::new();
        tmp_key_bw.put_u32(MAINTX_STATE_KEY_IDENTIFIER);
        tmp_key_bw.put_hash(hash);
        let tmp_key = tmp_key_bw.get_bytes();
        // 
        let statedb = self.shared_statedb.lock().await;
        // Get a key-value pair from the database
        let get_result = statedb.get(&tmp_key);
        match get_result {
            Ok(Some(tmp_value)) => {
                println!("Get key successful: {:?}", tmp_key);

                let mut tmp_value_br = BufferReader::new(tmp_value);
                let maintx_mainblock_height=tmp_value_br.get_u32()?;
                let maintx_postion=tmp_value_br.get_u32()?;
                return Ok((maintx_mainblock_height,maintx_postion));
            }
            Ok(None) => {
                println!("Key not found.");
                //Err(Box::new(std::io::Error::new(ErrorKind::Other, "get_maintx_state error - maintx_state not found")))
                Err(MaintxStateError::OtherError("get_maintx_state error - maintx_state not found".to_string()))
            }
            Err(err) => {
                println!("Error: {}", err);
                Err(MaintxStateError::DbError(err))
            }
        }    
    }
    //
    pub async fn insert_maintx_out_state(& mut self,hash:Hash,index:u32,maintx_out_state:u32)->Result<(),MaintxStateError>{
        let mut tmp_key_bw = BufferWriter::new();
        tmp_key_bw.put_u32(MAINTX_OUT_STATE_KEY_IDENTIFIER);
        tmp_key_bw.put_hash(hash);
        tmp_key_bw.put_u32(index);
        let tmp_key = tmp_key_bw.get_bytes();
        let mut tmp_value_bw = BufferWriter::new();
        tmp_value_bw.put_u32(maintx_out_state);
        let tmp_value = tmp_value_bw.get_bytes();
        //
        let statedb = self.shared_statedb.lock().await;
        // Insert a key-value pair into the database
        statedb.insert(&tmp_key, &tmp_value)?;
        Ok(())
    }
    pub async fn get_maintx_out_state(& mut self,hash:Hash,index:u32)->Result<(u32),MaintxStateError> {
        let mut tmp_key_bw = BufferWriter::new();
        tmp_key_bw.put_u32(MAINTX_OUT_STATE_KEY_IDENTIFIER);
        tmp_key_bw.put_hash(hash);
        tmp_key_bw.put_u32(index);
        let tmp_key = tmp_key_bw.get_bytes();
        // 
        let statedb = self.shared_statedb.lock().await;
        // Get a key-value pair from the database
        let get_result = statedb.get(&tmp_key);
        match get_result {
            Ok(Some(tmp_value)) => {
                println!("Get key successful: {:?}", tmp_key);
                let mut tmp_value_br = BufferReader::new(tmp_value);
                let maintx_out_state=tmp_value_br.get_u32()?;
                return Ok((maintx_out_state));
            }
            Ok(None) => {
                println!("Key not found.");
                //Err(Box::new(std::io::Error::new(ErrorKind::Other, "get_maintx_state error - maintx_state not found")))
                Err(MaintxStateError::OtherError("get_maintx_state error - maintx_state not found".to_string()))
            }
            Err(err) => {
                println!("Error: {}", err);
                //Err(Box::new(err))
                Err(MaintxStateError::DbError(err))
            }
        }    
    }
    pub async fn is_maintx_out_unspent(& mut self,hash:Hash,maintx_out_index:u32)->Result<(bool),MaintxStateError>{
        let get_result2 = self.get_maintx_out_state(hash,maintx_out_index);
        match get_result2.await {
            Ok(maintx_out_state) => {
                if maintx_out_state==MAINTX_OUT_STATE_VALUE_IDENTIFIER_UNSPENT {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            },
            Err(err) => Err(err),
        }
    }
    //pub async fn is_maintx_out_spent

}

