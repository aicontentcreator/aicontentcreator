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
// A daemon starts with a type then a version then an array of hashes that are used to generate key pairs 
//use crate::daemon::WALLET_CATEGORY_SEQUENTIAL;
//
//use crate::asset;
//use crate::asset::Asset;
//#[derive(Clone)]

//
use wallet::wallet_v1::wallet_proxy::WalletProxy;
use tokio::sync::Mutex;

//
#[derive(Debug)]
pub struct DaemonInner {
    //pub category: usize,
    path: String,
    shared_app_wallet_proxy:Mutex<WalletProxy>,
}
// DaemonInnerError Definition
#[derive(Debug, Error)]
pub enum DaemonInnerError {
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

impl DaemonInner {

}
*/
pub async fn new_daemon_inner (tmp_path: String,app_wallet_proxy:WalletProxy) -> Result<DaemonInner, DaemonInnerError> {

    let mut new_daemon_inner= DaemonInner {
        //category:0,
        path:tmp_path,
        shared_app_wallet_proxy:Mutex::new(app_wallet_proxy),

    };

    Ok(new_daemon_inner)
}