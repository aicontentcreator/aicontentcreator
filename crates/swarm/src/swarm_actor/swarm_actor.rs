use tokio::sync::{mpsc, oneshot};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use crate::swarm_command::swarm_command::SwarmCommand;
use crate::swarm_inner::swarm_inner::SwarmInner;
use crate::swarm_inner::swarm_inner::SwarmInnerError;
use crate::swarm_inner::swarm_inner::new_swarm_inner;
use utility::system::time::{format_timestamp_to_gmt_string, timestamp_now};
// Define the actor struct
#[derive(Debug)]
pub struct SwarmActor {
    pub shared_swarm_inner:Arc<Mutex<SwarmInner>>, 
    pub receiver: mpsc::Receiver<SwarmCommand>,
    //pub name: String,

}
// SwarmActorError Definition
#[derive(Debug, Error)]
pub enum SwarmActorError {
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
    #[error("SwarmInnerError error: {0}")]
    SwarmInnerError(#[from] SwarmInnerError),
}

impl SwarmActor {
    // Process incoming commands asynchronously
    pub async fn process(mut self) {
        while let Some(command) = self.receiver.recv().await {
            match command {
                /*
                SwarmCommand::GetAddresses { result_sender } => {
                    let tmp_swarm_inner = self.shared_swarm_inner.lock().unwrap();
                    //let tmp_balance =tmp_swarm_inner.get_balance();
                    let _ = result_sender.send(tmp_swarm_inner.get_addresses()); // Send the result of addition
                }
                //
                SwarmCommand::GetSharedSwarmInner { result_sender } => {
                    //let tmp_swarm_inner = self.shared_swarm_inner.lock().unwrap();
                    //let tmp_balance =tmp_swarm_inner.get_balance();
                    let _ = result_sender.send(self.shared_swarm_inner.clone()); // Send the result of addition
                }
                //
                SwarmCommand::GetBalance { result_sender } => {
                    let tmp_swarm_inner = self.shared_swarm_inner.lock().unwrap();
                    let tmp_balance =tmp_swarm_inner.get_balance();
                    let _ = result_sender.send(tmp_balance); // Send the result of addition
                }
                */
                SwarmCommand::GetOperationalSituation { result_sender } => {
                    let current_time = format_timestamp_to_gmt_string(timestamp_now());
                    let _ = result_sender.send(current_time);//(String::from("Hello World")); // Send the result of addition
                }
                SwarmCommand::Add { x, y, result_sender } => {
                    let _ = result_sender.send(x + y); // Send the result of addition
                }
                SwarmCommand::Concatenate { x, y, result_sender } => {
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
    pub fn get_shared_swarm_inner(mut self)-> Arc<Mutex<SwarmInner>> {
        return self.shared_swarm_inner.clone()
    }*/
    // Access the actor's name
    //pub fn name(&self) -> &str {
    //    &self.name
    //}
}

pub fn new_swarm_actor (tmp_path: String,receiver:mpsc::Receiver<SwarmCommand>) -> Result<SwarmActor, SwarmActorError> {
    let tmp_result=new_swarm_inner(tmp_path);
    match tmp_result {
        Ok(tmp_new_swarm_inner)=> {
            let tmp_shared_swarm_inner = Arc::new(Mutex::new(tmp_new_swarm_inner));
            let new_swarm_actor= SwarmActor {
                shared_swarm_inner:tmp_shared_swarm_inner,
                receiver,
            };

            Ok(new_swarm_actor)
        }
        Err(e)=> {
            return Err(SwarmActorError::SwarmInnerError(e))
        }
    }

}