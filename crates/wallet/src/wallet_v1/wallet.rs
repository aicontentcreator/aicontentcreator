
use tokio::sync::{mpsc, oneshot};
use crate::wallet_v1::wallet_actor::WalletActor;
use crate::wallet_v1::wallet_actor::new_wallet_actor;
use crate::wallet_v1::wallet_proxy::WalletProxy;
use crate::wallet_v1::wallet_actor::WalletActorError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("WalletActorError error: {0}")]
    WalletActorError(#[from] WalletActorError),
    /*
    #[error("Key pair vector is empty")]
    EmptyEcdsaKeySet,

    #[error("Invalid index: {0}")]
    InvalidIndex(usize),
    #[error("KeyDerivationError error: {0}")]
    KeyDerivationError(#[from] KeyDerivationError),

    #[error("generate_key_set failed after too many attempt")]
    GenerateKeySetFailedAfterTooManyAttempt,

    //#[error("ECDSA error: {0}")]
    //EcdsaKeySetError(String),
    #[error("EcdsaKeySetError error: {0}")]
    EcdsaKeySetError(#[from] EcdsaKeySetError),

    #[error("Failed to generate random number: {0}")]
    RandomNumberError(String),

    #[error("MaintxOutError error: {0}")]
    MaintxOutError(#[from] MaintxOutError),
    #[error("MaintxInError error: {0}")]
    MaintxInError(#[from] MaintxInError),
    */
}

// Initialize the actor and its corresponding proxy
pub fn init_wallet_actor_wallet_proxy(seed: String, size: usize) -> Result<(WalletActor, WalletProxy), WalletError> {//(WalletActor, WalletProxy) {
    let (sender, receiver) = mpsc::channel(size);
    //let actor = WalletActor { receiver, name: name.clone() };
    let tmp_result = new_wallet_actor(seed,receiver);
    match tmp_result {
        Ok(tmp_new_wallet_actor)=> {
            let proxy = WalletProxy { sender };
            Ok((tmp_new_wallet_actor, proxy))
        }
        Err(e)=> {
            println!("wallet_inner generation error {:?}",e);
            return Err(WalletError::WalletActorError(e))
        }
    }

}
