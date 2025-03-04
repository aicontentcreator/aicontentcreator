use tokio::sync::{mpsc, oneshot};
use crate::daemon_command::daemon_command::DaemonCommand;
use utility::hash::hash::Hash;
use crate::daemon_inner::daemon_inner::DaemonInner;
use std::sync::{Arc, Mutex};
// Define the proxy struct for interacting with the actor
#[derive(Debug,Clone)]
pub struct DaemonProxy {
    pub sender: mpsc::Sender<DaemonCommand>,
}

impl DaemonProxy {
    /*
    pub async fn get_shared_daemon_inner(&self) -> Result<Arc<Mutex<DaemonInner>> , tokio::sync::mpsc::error::SendError<DaemonCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DaemonCommand::GetSharedDaemonInner { result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }
    //
    pub async fn get_addresses(&self) -> Result<Vec<Hash>, tokio::sync::mpsc::error::SendError<DaemonCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DaemonCommand::GetAddresses { result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }
    //
    pub async fn get_balance(&self) -> Result<u64, tokio::sync::mpsc::error::SendError<DaemonCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DaemonCommand::GetBalance { result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }*/
    // Send an operational situation (String) command to the actor and await the result
    pub async fn get_operational_situation(&self) -> Result<String, tokio::sync::mpsc::error::SendError<DaemonCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DaemonCommand::GetOperationalSituation { result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }
    //
    pub async fn get_wallet_operational_situation(&self) -> Result<String, tokio::sync::mpsc::error::SendError<DaemonCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DaemonCommand::GetWalletOperationalSituation { result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }
    // Send an addition command to the actor and await the result
    pub async fn add(&self, x: i32, y: i32) -> Result<i32, tokio::sync::mpsc::error::SendError<DaemonCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DaemonCommand::Add { x, y, result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }

    // Send a concatenation command to the actor and await the result
    pub async fn concatenate(&self, x: String, y: String) -> Result<String, tokio::sync::mpsc::error::SendError<DaemonCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DaemonCommand::Concatenate { x, y, result_sender }).await?;
        let result = result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor"));
        match result {
            Ok(value) => Ok(value),
            Err(_) => panic!("Failed to concatenate"),
        }
    }
}