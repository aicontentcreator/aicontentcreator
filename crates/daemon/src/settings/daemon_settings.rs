use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct DaemonSettings {
    //pub name: String,
    pub channel_size: usize,
}

impl DaemonSettings {
    /// Creates a new `DaemonSettings` instance with default values.
    pub fn new_default_daemon_settings() -> Self {
        Self {
            //name: "DefaultDaemon".to_string(),
            channel_size: 128,
        }
    }
}
