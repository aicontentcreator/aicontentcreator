use tokio::sync::{mpsc, oneshot};
use crate::swarm_command::swarm_command::SwarmCommand;
use utility::hash::hash::Hash;
use crate::swarm_inner::swarm_inner::SwarmInner;
use std::sync::{Arc, Mutex};
// Define the proxy struct for interacting with the actor
#[derive(Debug)]
pub struct SwarmProxy {
    pub sender: mpsc::Sender<SwarmCommand>,
}

impl SwarmProxy {
    /*
    pub async fn get_shared_swarm_inner(&self) -> Result<Arc<Mutex<SwarmInner>> , tokio::sync::mpsc::error::SendError<SwarmCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(SwarmCommand::GetSharedSwarmInner { result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }
    //
    pub async fn get_addresses(&self) -> Result<Vec<Hash>, tokio::sync::mpsc::error::SendError<SwarmCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(SwarmCommand::GetAddresses { result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }
    //
    pub async fn get_balance(&self) -> Result<u64, tokio::sync::mpsc::error::SendError<SwarmCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(SwarmCommand::GetBalance { result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }*/
    // Send an operational situation (String) command to the actor and await the result
    pub async fn get_operational_situation(&self) -> Result<String, tokio::sync::mpsc::error::SendError<SwarmCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(SwarmCommand::GetOperationalSituation { result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }

    // Send an addition command to the actor and await the result
    pub async fn add(&self, x: i32, y: i32) -> Result<i32, tokio::sync::mpsc::error::SendError<SwarmCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(SwarmCommand::Add { x, y, result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }

    // Send a concatenation command to the actor and await the result
    pub async fn concatenate(&self, x: String, y: String) -> Result<String, tokio::sync::mpsc::error::SendError<SwarmCommand>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(SwarmCommand::Concatenate { x, y, result_sender }).await?;
        let result = result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor"));
        match result {
            Ok(value) => Ok(value),
            Err(_) => panic!("Failed to concatenate"),
        }
    }
}