use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct WalletSettings {
    //pub name: String,
    pub channel_size: usize,
}

impl WalletSettings {
    /// Creates a new `WalletSettings` instance with default values.
    pub fn new_default_wallet_settings() -> Self {
        Self {
            //name: "DefaultWallet".to_string(),
            channel_size: 128,
        }
    }
}
