use serde::{Serialize, Deserialize};
use thiserror::Error;
use serde_json;

use utility::storage::storage_directory::StorageDirectory;
use utility::storage::storage_directory::StorageDirectoryError;

use crate::settings::daemon_settings::DaemonSettings;

/// Define custom errors
#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("Json Serialization error: {0}")]
    JsonSerializationError(#[from] serde_json::Error),
    //
    #[error("Storage directory error: {0}")]
    StorageDirectoryError(#[from] StorageDirectoryError),
    //#[error("Mainblock error: {0}")]
    //MainblockError(#[from] MainblockError),
    #[error("Invalid UTF-8 data: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("{0}")]
    OtherError(String),
}



//#[derive( Debug)]
pub struct Settings {
    sd: StorageDirectory,
    pub daemon_settings:DaemonSettings,
}

impl Settings {
    pub async fn new_default() -> Result<Self, SettingsError> {
        let tmp_sd_path = "Settings";
        /*
        directory::create_directory(&tmp_sd_path)
            //.map_err(|_| UnconfirmedMainblocksError::DirectoryError("Failed to create directory".to_string()))?;
            .map_err(UnconfirmedMainblocksError::Io)?; 
        */
        let mut tmp_sd = StorageDirectory::new(&tmp_sd_path,String::from("Settings")).await?;
        tmp_sd.init().await.map_err(SettingsError::StorageDirectoryError)?;
        /*let mut new_daemon_settings = DaemonSettings {
            name:String::from(""),
            age:0,
        };*/
        let new_daemon_settings=DaemonSettings::new_default_daemon_settings();
           
        Ok(Self {
            sd: tmp_sd,
            daemon_settings:new_daemon_settings,
        })
    }
    pub async fn save(&mut self) -> Result<(), SettingsError>  {


        let rawstring=json_serialize_settings(&self.daemon_settings)?;
        match self.sd.add_chunk(rawstring.as_bytes()).await {
            Ok(_)=> {
                println!("settings save success");
                Ok(())
            }
            Err(e)=> {
                println!("settings save error {:?}",e);
                Err(SettingsError::StorageDirectoryError(e))
            }
        }
    }
    //
    pub async fn load(&mut self)-> Result<(), SettingsError> {

        match self.sd.load_bytes_from_last_file().await {
            Ok(rawbytes)=> {
                println!("settings load success");
                //let mb=unserialize_mainblock(mb_rawbytes)?;
                //Ok(mb)
                let tmp_string=vec_u8_to_string(rawbytes)?;
                self.daemon_settings=json_deserialize_settings(&tmp_string)?;
                Ok(())
                //match vec_u8_to_string(rawbytes) {
                //    Ok(s) => println!("Valid UTF-8: {}", s),
                //    Err(e) => println!("Error converting valid UTF-8: {}", e),
                //}
            
            }
            Err(e)=> {
                println!("settings load error {:?}",e);
                Err(SettingsError::StorageDirectoryError(e))
            }
        }
    }
}

fn vec_u8_to_string(vec: Vec<u8>) -> Result<String, SettingsError> {
    String::from_utf8(vec).map_err(SettingsError::from)
}

pub fn json_serialize_settings(tmp_settings: &DaemonSettings) -> Result<String, SettingsError> {
    serde_json::to_string(tmp_settings).map_err(SettingsError::from)
}

pub fn json_deserialize_settings(json_string: &str) -> Result<DaemonSettings, SettingsError> {
    serde_json::from_str(json_string).map_err(SettingsError::from)
}
