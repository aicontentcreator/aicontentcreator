



//extern crate utility;
use utility::statedb::statedb::StateDb;
use utility::statedb::statedb::Error as DbError;
use std::sync::Arc;
use tokio::sync::Mutex;
pub const MAINTX_STATE_KEY_IDENTIFIER : u32=1;
pub const MAINTX_OUT_STATE_KEY_IDENTIFIER:u32=2;
//
pub const MAINTX_OUT_STATE_VALUE_IDENTIFIER_UNSPENT:u32=1;
pub const MAINTX_OUT_STATE_VALUE_IDENTIFIER_SPENT:u32=2;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MainstateError {
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
    //#[error("Mainheader error: {0}")]
    //MainheaderError(#[from] MainheaderError),
    //MaintxStateError
    #[error("Db error: {0}")]
    DbError(#[from] DbError),
}


pub struct Mainstate {
    pub shared_statedb:Arc<Mutex<StateDb>>, 
}
impl Mainstate {
//
    pub fn new()-> Result<Self, MainstateError>{
        Ok(Mainstate {
            shared_statedb:Arc::new(Mutex::new(StateDb::new("Mainstate")?))
        })
    }

 
}