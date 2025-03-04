use tokio::sync::{mpsc, oneshot};
use std::sync::{Arc, Mutex};
use crate::wallet_v1::wallet_inner::WalletInner;
use utility::hash::hash::Hash;



// Define the types of commands that can be sent to the actor

pub enum WalletCommand {
    GetAddresses {
        result_sender: oneshot::Sender<Vec<Hash>>,       
    },
    //
    GetSharedWalletInner {
        result_sender: oneshot::Sender<Arc<Mutex<WalletInner>> >,       
    },
    GetBalance {
        result_sender: oneshot::Sender<u64>,       
    },
    GetOperationalSituation {
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
