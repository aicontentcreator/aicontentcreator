use tokio::sync::{mpsc, oneshot};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use crate::daemon_command::daemon_command::DaemonCommand;
use crate::daemon_inner::daemon_inner::DaemonInner;
use crate::daemon_inner::daemon_inner::DaemonInnerError;
use crate::daemon_inner::daemon_inner::new_daemon_inner;
use wallet::wallet_v1::wallet_proxy::WalletProxy;
use utility::system::time::{format_timestamp_to_gmt_string, timestamp_now};
// Define the actor struct
#[derive(Debug)]
pub struct DaemonActor {
    pub shared_daemon_inner:Arc<Mutex<DaemonInner>>, 
    pub receiver: mpsc::Receiver<DaemonCommand>,
    //pub name: String,

}
// DaemonActorError Definition
#[derive(Debug, Error)]
pub enum DaemonActorError {
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
    #[error("DaemonInnerError error: {0}")]
    DaemonInnerError(#[from] DaemonInnerError),
}

impl DaemonActor {
    // Process incoming commands asynchronously
    //async fn process(mut self) {
    pub async fn process(mut self) {
        while let Some(command) = self.receiver.recv().await {
            match command {
                /*
                DaemonCommand::GetAddresses { result_sender } => {
                    let tmp_daemon_inner = self.shared_daemon_inner.lock().unwrap();
                    //let tmp_balance =tmp_daemon_inner.get_balance();
                    let _ = result_sender.send(tmp_daemon_inner.get_addresses()); // Send the result of addition
                }
                //
                DaemonCommand::GetSharedDaemonInner { result_sender } => {
                    //let tmp_daemon_inner = self.shared_daemon_inner.lock().unwrap();
                    //let tmp_balance =tmp_daemon_inner.get_balance();
                    let _ = result_sender.send(self.shared_daemon_inner.clone()); // Send the result of addition
                }
                //
                DaemonCommand::GetBalance { result_sender } => {
                    let tmp_daemon_inner = self.shared_daemon_inner.lock().unwrap();
                    let tmp_balance =tmp_daemon_inner.get_balance();
                    let _ = result_sender.send(tmp_balance); // Send the result of addition
                }
                */
                DaemonCommand::GetOperationalSituation { result_sender } => {
                    let current_time = format_timestamp_to_gmt_string(timestamp_now());
                    let _ = result_sender.send(current_time);//(String::from("Hello World")); // Send the result of addition
                }
                DaemonCommand::GetWalletOperationalSituation { result_sender } => {
                    let current_time = format_timestamp_to_gmt_string(timestamp_now());
                    let _ = result_sender.send(current_time);//(String::from("Hello World")); // Send the result of addition
                }
                DaemonCommand::Add { x, y, result_sender } => {
                    let _ = result_sender.send(x + y); // Send the result of addition
                }
                DaemonCommand::Concatenate { x, y, result_sender } => {
                    let result = if x.is_ascii() && y.is_ascii() {
                        Ok(format!("{}{}", x, y)) // Concatenate strings if both are ASCII
                    } else {
                        Err(()) // Return error if either string is non-ASCII
                    };
                    let _ = result_sender.send(result); // Send the result of concatenation
                }
            }
        }
    }
    /*
    pub fn get_shared_daemon_inner(mut self)-> Arc<Mutex<DaemonInner>> {
        return self.shared_daemon_inner.clone()
    }*/
    // Access the actor's name
    //pub fn name(&self) -> &str {
    //    &self.name
    //}
}

pub async fn new_daemon_actor (tmp_path: String,app_wallet_proxy:WalletProxy,receiver:mpsc::Receiver<DaemonCommand>) -> Result<DaemonActor, DaemonActorError> {
    let tmp_result=new_daemon_inner(tmp_path,app_wallet_proxy);
    match tmp_result.await {
        Ok(tmp_new_daemon_inner)=> {
            let tmp_shared_daemon_inner = Arc::new(Mutex::new(tmp_new_daemon_inner));
            let new_daemon_actor= DaemonActor {
                shared_daemon_inner:tmp_shared_daemon_inner,
                receiver,
            };

            Ok(new_daemon_actor)
        }
        Err(e)=> {
            return Err(DaemonActorError::DaemonInnerError(e))
        }
    }

}