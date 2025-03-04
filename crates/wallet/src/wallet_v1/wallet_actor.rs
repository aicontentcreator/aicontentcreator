use tokio::sync::{mpsc, oneshot};
use std::sync::{Arc, Mutex};
use std::error::Error;
use crate::wallet_v1::wallet_command::WalletCommand;
use crate::wallet_v1::wallet_inner::WalletInner;
use crate::wallet_v1::wallet_inner::new_hardened_wallet_inner_with_seed;
use crate::wallet_v1::wallet_inner::WalletInnerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalletActorError {
    #[error("WalletInnerError error: {0}")]
    WalletInnerError(#[from] WalletInnerError),
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

// Define the actor struct
#[derive(Debug)]
pub struct WalletActor {
    pub shared_wallet_inner:Arc<Mutex<WalletInner>>, 
    pub receiver: mpsc::Receiver<WalletCommand>,
    //pub name: String,

}

impl WalletActor {

    // Process incoming commands asynchronously
    pub async fn process(mut self) {
        while let Some(command) = self.receiver.recv().await {
            match command {
                WalletCommand::GetAddresses { result_sender } => {
                    let tmp_wallet_inner = self.shared_wallet_inner.lock().unwrap();
                    //let tmp_balance =tmp_wallet_inner.get_balance();
                    let _ = result_sender.send(tmp_wallet_inner.get_addresses()); // Send the result of addition
                }
                //
                WalletCommand::GetSharedWalletInner { result_sender } => {
                    //let tmp_wallet_inner = self.shared_wallet_inner.lock().unwrap();
                    //let tmp_balance =tmp_wallet_inner.get_balance();
                    let _ = result_sender.send(self.shared_wallet_inner.clone()); // Send the result of addition
                }
                //
                WalletCommand::GetBalance { result_sender } => {
                    let tmp_wallet_inner = self.shared_wallet_inner.lock().unwrap();
                    let tmp_balance =tmp_wallet_inner.get_balance();
                    let _ = result_sender.send(tmp_balance); // Send the result of addition
                }
                WalletCommand::GetOperationalSituation { result_sender } => {
                    let tmp_wallet_inner = self.shared_wallet_inner.lock().unwrap();
                    //let tmp_balance =tmp_wallet_inner.get_balance();
                    let _ = result_sender.send(String::from("WalletGetOperationalSituation123456")); // Send the result 
                }
                //
                WalletCommand::Add { x, y, result_sender } => {
                    let _ = result_sender.send(x + y); // Send the result of addition
                }
                WalletCommand::Concatenate { x, y, result_sender } => {
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
    pub fn get_shared_wallet_inner(mut self)-> Arc<Mutex<WalletInner>> {
        return self.shared_wallet_inner.clone()
    }*/
    // Access the actor's name
    //pub fn name(&self) -> &str {
    //    &self.name
    //}
}

pub fn new_wallet_actor_with_seed (wallet_seed: String,receiver:mpsc::Receiver<WalletCommand>) -> Result<WalletActor, WalletActorError> {
    let tmp_result=new_hardened_wallet_inner_with_seed(wallet_seed);//Default wallet is hardened
    match tmp_result {
        Ok(tmp_new_wallet_inner)=> {
            let tmp_shared_wallet_inner = Arc::new(Mutex::new(tmp_new_wallet_inner));
            let new_wallet_actor= WalletActor {
                shared_wallet_inner:tmp_shared_wallet_inner,
                receiver,
            };

            Ok(new_wallet_actor)
        }
        Err(e)=> {
            return Err(WalletActorError::WalletInnerError(e))
        }
    }
}
/*
pub fn new_wallet_actor_using_wallet_file (wallet_file_path: String,receiver:mpsc::Receiver<WalletCommand>) -> Result<WalletActor, WalletActorError> {
    
    let tmp_result=new_hardened_wallet_inner_using_wallet_file(wallet_file_path);
    match tmp_result {
        Ok(tmp_new_wallet_inner)=> {
            let tmp_shared_wallet_inner = Arc::new(Mutex::new(tmp_new_wallet_inner));
            let new_wallet_actor= WalletActor {
                shared_wallet_inner:tmp_shared_wallet_inner,
                receiver,
            };

            Ok(new_wallet_actor)
        }
        Err(e)=> {
            return Err(WalletActorError::WalletInnerError(e))
        }
    }
    
}
*/