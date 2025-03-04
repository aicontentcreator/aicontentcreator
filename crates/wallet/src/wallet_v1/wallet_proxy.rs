use tokio::sync::{mpsc, oneshot};
use crate::wallet_v1::wallet_command::WalletCommand;

use utility::hash::hash::Hash;
use crate::wallet_v1::wallet_inner::WalletInner;
use std::sync::{Arc, Mutex};
// Define the proxy struct for interacting with the actor
#[derive(Debug,Clone)]
pub struct WalletProxy {
    pub sender: mpsc::Sender<WalletCommand>,
}

impl WalletProxy {
    pub async fn get_shared_wallet_inner(&self) -> Result<Arc<Mutex<WalletInner>> , tokio::sync::mpsc::error::SendError<WalletCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(WalletCommand::GetSharedWalletInner { result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }
    //
    pub async fn get_addresses(&self) -> Result<Vec<Hash>, tokio::sync::mpsc::error::SendError<WalletCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(WalletCommand::GetAddresses { result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }
    //
    pub async fn get_balance(&self) -> Result<u64, tokio::sync::mpsc::error::SendError<WalletCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(WalletCommand::GetBalance { result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }
    pub async fn get_operational_situation(&self) -> Result<String, tokio::sync::mpsc::error::SendError<WalletCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(WalletCommand::GetOperationalSituation { result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }
    // Send an addition command to the actor and await the result
    pub async fn add(&self, x: i32, y: i32) -> Result<i32, tokio::sync::mpsc::error::SendError<WalletCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(WalletCommand::Add { x, y, result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }

    // Send a concatenation command to the actor and await the result
    pub async fn concatenate(&self, x: String, y: String) -> Result<String, tokio::sync::mpsc::error::SendError<WalletCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(WalletCommand::Concatenate { x, y, result_sender }).await?;
        let result = result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor"));
        match result {
            Ok(value) => Ok(value),
            Err(_) => panic!("Failed to concatenate"),
        }
    }
}