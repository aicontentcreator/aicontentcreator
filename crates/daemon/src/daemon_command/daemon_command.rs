use tokio::sync::{mpsc, oneshot};
use std::sync::{Arc, Mutex};
use crate::daemon_inner::daemon_inner::DaemonInner;
//use utility::hashing::Hash;
use thiserror::Error;

/*
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

*/

// Define the types of commands that can be sent to the actor
#[derive(Debug)]
pub enum DaemonCommand {
    /*
    GetAddresses {
        result_sender: oneshot::Sender<Vec<Hash>>,       
    },
    //
    GetSharedDaemonInner {
        result_sender: oneshot::Sender<Arc<Mutex<DaemonInner>> >,       
    },
    GetBalance {
        result_sender: oneshot::Sender<u64>,       
    },
    */
    ///////////////////////
    GetOperationalSituation {
        result_sender: oneshot::Sender<String>,
    },
    //
    GetWalletOperationalSituation {
        result_sender: oneshot::Sender<String>,
    },    
    ///////////////////////
    Add {
        x: i32,
        y: i32,
        result_sender: oneshot::Sender<i32>,
    },
    Concatenate {
        x: String,
        y: String,
        result_sender: oneshot::Sender<Result<String, ()>>,
    },
    //////////////////////////
}