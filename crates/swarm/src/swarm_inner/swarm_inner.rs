//use utility::hashing;
//use utility::ecdsa;
use thiserror::Error;
//use utility::hashing;
//use utility::ecdsa;
//use utility::hashing::Hash;

//use utility::bufferwriter::BufferWriter;
//use utility::bufferreader::BufferReader;
//use utility::utility::generate_random_number;
//use tx::tx::Transaction;
//use utility::bytesfile;
//use std::fs;
// A swarm starts with a type then a version then an array of hashes that are used to generate key pairs 
//use crate::swarm::WALLET_CATEGORY_SEQUENTIAL;
//
//use crate::asset;
//use crate::asset::Asset;
//#[derive(Clone)] 
#[derive(Debug)]
pub struct SwarmInner {
    //pub category: usize,
    path: String
}
// SwarmInnerError Definition
#[derive(Debug, Error)]
pub enum SwarmInnerError {
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
    #[error("MaintxInError error: {0}")]
    MaintxInError(#[from] MaintxInError),
    #[error("MaintxOutError error: {0}")]
    MaintxOutError(#[from] MaintxOutError),
    #[error("Mainstate error: {0}")]
    MainstateError(#[from] MainstateError),
    #[error("MaintxStateError error: {0}")]
    MaintxStateError(#[from] MaintxStateError),
    #[error("ConfirmationError error: {0}")]
    ConfirmationError(#[from] ConfirmationError),
    
    */
    
}
/*

impl SwarmInner {
    

}
*/
pub fn new_swarm_inner (tmp_path: String) -> Result<SwarmInner, SwarmInnerError> {
    let mut new_swarm_inner= SwarmInner {
            //category:0,
            path:tmp_path
        };

    Ok(new_swarm_inner)
}