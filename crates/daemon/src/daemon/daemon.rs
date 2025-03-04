
use tokio::sync::{mpsc, oneshot};
use crate::daemon_actor::daemon_actor::DaemonActor;
use crate::daemon_actor::daemon_actor::DaemonActorError;
use crate::daemon_actor::daemon_actor::new_daemon_actor;
use crate::daemon_proxy::daemon_proxy::DaemonProxy;
use crate::settings::daemon_settings::DaemonSettings;
use wallet::wallet_v1::wallet_proxy::WalletProxy;
use thiserror::Error;
// DaemonError Definition
#[derive(Debug, Error)]
pub enum DaemonError {
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
    #[error("DaemonActorError error: {0}")]
    DaemonActorError(#[from] DaemonActorError),
    
}
// Initialize the actor and its corresponding proxy
pub async fn init_daemon_actor_daemon_proxy(tmp_path: String,app_wallet_proxy:WalletProxy, tmp_daemon_settings: DaemonSettings) -> Result<(DaemonActor, DaemonProxy), DaemonError> {//(DaemonActor, DaemonProxy) {
    let (sender, receiver) = mpsc::channel(tmp_daemon_settings.channel_size);
    //let actor = DaemonActor { receiver, name: name.clone() };
    let tmp_result = new_daemon_actor(tmp_path,app_wallet_proxy,receiver);
    match tmp_result.await {
        Ok(tmp_new_daemon_actor)=> {
            let proxy = DaemonProxy { sender };
            Ok((tmp_new_daemon_actor, proxy))
        }
        Err(e)=> {
            println!("daemon_inner generation error {:?}",e);
            return Err(DaemonError::DaemonActorError(e))
        }
    }

}