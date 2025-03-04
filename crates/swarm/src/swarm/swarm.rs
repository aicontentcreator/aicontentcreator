
use tokio::sync::{mpsc, oneshot};
use crate::swarm_actor::swarm_actor::SwarmActor;
use crate::swarm_actor::swarm_actor::SwarmActorError;
use crate::swarm_actor::swarm_actor::new_swarm_actor;
use crate::swarm_proxy::swarm_proxy::SwarmProxy;
use thiserror::Error;
// SwarmError Definition
#[derive(Debug, Error)]
pub enum SwarmError {
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
    #[error("SwarmActorError error: {0}")]
    SwarmActorError(#[from] SwarmActorError),
    
}
// Initialize the actor and its corresponding proxy
pub fn init_swarm_actor_swarm_proxy(tmp_path: String, size: usize) -> Result<(SwarmActor, SwarmProxy), SwarmError> {//(SwarmActor, SwarmProxy) {
    let (sender, receiver) = mpsc::channel(size);
    //let actor = SwarmActor { receiver, name: name.clone() };
    let tmp_result = new_swarm_actor(tmp_path,receiver);
    match tmp_result {
        Ok(tmp_new_swarm_actor)=> {
            let proxy = SwarmProxy { sender };
            Ok((tmp_new_swarm_actor, proxy))
        }
        Err(e)=> {
            println!("swarm_inner generation error {:?}",e);
            return Err(SwarmError::SwarmActorError(e))
        }
    }

}