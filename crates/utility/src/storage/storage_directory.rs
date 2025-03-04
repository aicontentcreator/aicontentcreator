
use tokio::fs::{self, File};//, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::path::{Path, PathBuf};
use thiserror::Error;


#[derive(Debug, Error)]
pub enum StorageDirectoryError {
    #[error("I/O error occurred: {0}")]
    Io(#[from] io::Error),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Failed to read a portion of the file: {0}")]
    ReadPortionError(io::Error), // Manually specify io::Error here

    #[error("File creation failed: {0}")]
    FileCreationError(String),
}

pub struct StorageDirectory{
    path: PathBuf,
    category:String,
    storage_files_last_index:Option<u32>,
}

//const MAX_STORAGE_FILE_INDEX:u32=50000000;

impl StorageDirectory{
    /// Creates a new `StorageDirectory` instance.
    pub async fn new<P: AsRef<Path>>(path: P,category: String) -> Result<Self,StorageDirectoryError> {
        let path = path.as_ref().to_path_buf();
        if !path.exists() {
            fs::create_dir_all(&path).await?;
        }
        println!("StorageDirectory - path:{:?} category {:?}", path,category);

        Ok(Self { path, category,storage_files_last_index:None})
    }

    /// Lists all files in the directory.
    pub async fn list_files(&self) -> Result<Vec<PathBuf>,StorageDirectoryError> {
        let mut files = Vec::new();
        let mut entries = fs::read_dir(&self.path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }
        Ok(files)
    }


    /// load the contents of a file in the directory.
    pub async fn load_bytes_from_file(&self, filename: &str) -> Result<Vec<u8>,StorageDirectoryError> {
        let file_path = self.path.join(filename);
        let mut file = File::open(file_path.clone()).await.map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => StorageDirectoryError::FileNotFound((file_path.clone()).to_str().expect("Failed to convert path to string").to_string()),
            _ => StorageDirectoryError::Io(e),
        })?;
    
        let mut contents = vec![];
        file.read_to_end(&mut contents).await.map_err(StorageDirectoryError::Io)?;
        Ok(contents)
    }

    /// save data to a file in the directory (creates the file if it doesn't exist).
    pub async fn save_bytes_to_file(&self, filename: &str, data: &[u8]) -> Result<(),StorageDirectoryError> {
        println!("StorageDirectory save_bytes_to_file {:?}",filename);
        let file_path = self.path.join(filename);
        let mut file = File::create(file_path).await.map_err(|e| StorageDirectoryError::FileCreationError(e.to_string()))?;
        file.write_all(data).await.map_err(StorageDirectoryError::Io)?;
        Ok(())
    }
    
    // TODO empty file - it does not delete the file it just empty it to save space, file should always be kept 

    /// Checks if a file exists in the directory.
    pub async fn file_exists(&self, filename: &str) -> bool {
        println!("does {:?} file_exists",filename);
        self.path.join(filename).exists()
    }
    pub fn get_storage_files_last_index(&self) -> u32 {
        match self.storage_files_last_index {
            Some(index) => return index,//println!("self.storage_files_last_index: {}", count),
            None => {


                match self.storage_files_last_index {
                    Some(new_index) => return new_index,//println!("self.storage_files_last_index: {}", count),
                    None => panic!("Fatal init_storage_files_last_index failed"),
                }
            },
        }
        
    }
    pub async fn init_storage_files_last_index(&mut self) {
        if let Ok(mut entries) = fs::read_dir(self.path.clone()).await {
            let mut max_index: Option<u32> = None;
            
            while let Ok(Some(entry)) = entries.next_entry().await {
                let filename = entry.file_name();
                let filename_str = filename.to_str().unwrap_or("");
                println!("filename_str {:?}", filename_str);
                
                if filename_str.starts_with(&self.category) {
                    if let Ok(index) = filename_str[self.category.len()..].parse::<u32>() {
                        max_index = max_index.map_or(Some(index), |current| Some(current.max(index)));
                    }
                }
            }
            //println!("max_index {:?}", max_index);
            //self.storage_files_last_index= max_index;
            match max_index {
                Some(tmp_index)=> self.storage_files_last_index=Some(tmp_index),
                None=> self.storage_files_last_index=Some(0),
            }
        }
    }
    pub async fn init(&mut self) -> Result<(),StorageDirectoryError> {
        //TODO better error handling
        self.init_storage_files_last_index().await;
        Ok(())
    }
    pub async fn add_chunk(&mut self,chunk_bytes: &[u8])->  Result<(),StorageDirectoryError> {

        let i=self.get_storage_files_last_index()+1;
        //match index_result {
        //    Some(index) => i=index,//println!("get_storage_files_last_index: {}", index),
        //    None => i=0,//println!("storage files last index not initialized"), 
        //}
        let file_path=format!("{}{}",self.category, i);
        self.save_bytes_to_file(&file_path, chunk_bytes).await?;
        
        if let Some(ref mut count) = self.storage_files_last_index {
            *count += 1;
        } else if self.storage_files_last_index .is_none() {
            self.storage_files_last_index  = Some(0);
        }

        Ok(())
    }
    pub async fn get_chunk(&mut self,chunk_height:u32)->  Result<Vec<u8>,StorageDirectoryError> {
        let file_path=format!("{}{}",self.category, chunk_height);
        self.load_bytes_from_file(&file_path).await
    }
    /*
    pub async fn push_file(&mut self, data: &[u8]) -> Result<(),StorageDirectoryError> {
        self.init_storage_files_last_index().await;
        let index=self.get_storage_files_last_index();
        let mut tmp_index=0;
        //match index_result {
        //    Some(index) => {
                tmp_index=index+1;
        //    },
        //    None => println!("storage files last index not initialized"), 
        //}
        println!("get_storage_files_last_index: {}", tmp_index);
        let file_name=format!("{}{}",self.category,tmp_index);
        self.save_bytes_to_file(&file_name, data ).await?;
        Ok(())
    }
    */
    pub async fn load_bytes_from_file_with_index(&self, index: u32) -> Result<Vec<u8>,StorageDirectoryError> {
        let file_name=format!("{}{}",self.category,index);
        let content=self.load_bytes_from_file(&file_name).await?;
        Ok(content)
    }

    pub async fn load_bytes_from_last_file(&self) -> Result<Vec<u8>,StorageDirectoryError> {
        let index=self.get_storage_files_last_index();

        //println!("get_storage_files_last_index: {}", index);
        let file_name=format!("{}{}",self.category,index);
        let content=self.load_bytes_from_file(&file_name).await?;
        Ok(content)
    }

}
