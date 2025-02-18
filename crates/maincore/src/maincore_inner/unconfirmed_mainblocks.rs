use std::io;
use thiserror::Error;
use utility::storage::storage_directory::StorageDirectory;
use utility::storage::storage_directory::StorageDirectoryError;
use crate::mainblock::mainblock::Mainblock;
use crate::mainblock::mainblock::MainblockError;
use crate::mainblock::mainblock::unserialize_mainblock;
use crate::mainheader::mainheader::Mainheader;
use crate::mainblock;
use utility::system::directory;

#[derive(Error, Debug)]
pub enum UnconfirmedMainblocksError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
    
    //#[error("Directory error: {0}")]
    //DirectoryError(#[from] io::Error),
    #[error("Storage directory error: {0}")]
    StorageDirectoryError(#[from] StorageDirectoryError),
    #[error("Mainblock error: {0}")]
    MainblockError(#[from] MainblockError),
    #[error("{0}")]
    OtherError(String),
}

pub struct UnconfirmedMainblocks {
    pub unconfirmed_mainblocks_queue_vector: Vec<UnconfirmedMainblocksQueue>,
}

impl UnconfirmedMainblocks {
    pub fn new() -> Self {
        UnconfirmedMainblocks {
            unconfirmed_mainblocks_queue_vector: Vec::new(),
        }
    }

    pub fn init(&self) -> Result<(), UnconfirmedMainblocksError> {
        prep_tmp_unconfirmed_mainblocks_directory()?;
        Ok(())
    }

    pub async fn new_unconfirmed_mainblocks_queue(
        &mut self,
        tmp_start_height: u32,
        parent_queue_index_option: Option<usize>,
    ) -> Result<usize, UnconfirmedMainblocksError> {
        let tmp_new_queue_index = self.unconfirmed_mainblocks_queue_vector.len();
        let new_queue = UnconfirmedMainblocksQueue::new(tmp_new_queue_index, tmp_start_height, parent_queue_index_option).await?;
        
        self.unconfirmed_mainblocks_queue_vector.push(new_queue);
        Ok(tmp_new_queue_index)
    }

    pub fn get_queues_count(& self) -> usize {
        self.unconfirmed_mainblocks_queue_vector.len()
    }
    pub async fn add_unconfirmed_mainblock_to_queue_with_index(&mut self,index:usize,mb: Mainblock) -> Result<(), UnconfirmedMainblocksError>  {
        self.unconfirmed_mainblocks_queue_vector[index].add_unconfirmed_mainblock_to_queue(mb).await?;
        Ok(())
    }
    pub fn get_unconfirmed_inmem_header_with_queue_index_and_height(&mut self,index:usize,header_height: u32)-> Result<Mainheader, UnconfirmedMainblocksError> {
        //let tmp_start_height=self.unconfirmed_mainblocks_queue_vector[index].start_height;
        //println!("get_unconfirmed_inmem_header_with_queue_index_and_height");
        self.unconfirmed_mainblocks_queue_vector[index].get_queue_unconfirmed_inmem_header(header_height)
    }
    pub fn get_start_height_with_queue_index(&self,index:usize) -> u32 {
        self.unconfirmed_mainblocks_queue_vector[index].start_height
    }
    pub fn get_queue_unconfirmed_mainblocks_last_height(&self,index:usize) -> u32 {
        self.unconfirmed_mainblocks_queue_vector[index].sd.get_storage_files_last_index()
    }
    pub fn get_queue_unconfirmed_mainblocks_start_height(&self,index:usize) -> u32 {
        self.unconfirmed_mainblocks_queue_vector[index].start_height
    }
    //

    pub fn get_longestqueue_last_height(&self) -> u32 {
        let mut tmp_last_height:u32=0;
        for i in 0..self.unconfirmed_mainblocks_queue_vector.len() {
            let new_tmp_last_height=self.get_queue_unconfirmed_mainblocks_last_height(i);
            if new_tmp_last_height>tmp_last_height {
                tmp_last_height=new_tmp_last_height;
            }
        }
        tmp_last_height
    }
    pub fn get_longestqueue_index(&self) -> usize {
        let mut tmp_blocks_count=0;
        let mut tmp_index=0;
        for i in 0..self.unconfirmed_mainblocks_queue_vector.len() {
            let new_tmp_blocks_count=self.get_queue_unconfirmed_mainblocks_last_height(i);
            if new_tmp_blocks_count>tmp_blocks_count {
                tmp_blocks_count=new_tmp_blocks_count;
                tmp_index=i;
            }
        }
        tmp_index
    }
    //pub fn get_absolute_unconfirmed_mainblocks_queue_blocks_count_with_index(&self,index:usize) -> usize {
    //    self.unconfirmed_mainblocks_queue_vector[index].start_height+self.get_unconfirmed_mainblocks_queue_blocks_count(index)
    //}

    pub fn get_longestqueue_inmem_mainheader(&self,tmp_height:u32) -> Result<Mainheader,UnconfirmedMainblocksError> {
        let mut tmp_index=self.get_longestqueue_index();
        for i in 0..1000 {
            let tmp_start_height=self.unconfirmed_mainblocks_queue_vector[tmp_index].start_height;
            println!("tmp_height {} tmp_start_height {}",tmp_height,tmp_start_height);
            if tmp_height>=tmp_start_height {
                println!("******************* get_longestqueue_inmem_mainheader calling get_queue_unconfirmed_inmem_header tmp_height {}",tmp_height);

                let tmp_header=self.unconfirmed_mainblocks_queue_vector[tmp_index].get_queue_unconfirmed_inmem_header(tmp_height)?;
                return Ok(tmp_header);
            } else  {
                match self.unconfirmed_mainblocks_queue_vector[tmp_index].parent_queue_index_option {
                    Some(value) => {
                        tmp_index=value;//self.unconfirmed_mainblocks_queue_vector[tmp_index].get_parent_unconfirmed_mainblocks_queue_index();
                        //self.get_absolute_longest_unconfirmed_mainblocks_queue_inmem_header(tmp_height,tmp_index)?;
                        continue;      
                    }
                    None => {
                        //return Err(Box::new(std::io::Error::new(ErrorKind::Other, " get_longestqueue_inmem_mainheader error no proper inmem_header found ")));
                        return Err(UnconfirmedMainblocksError::OtherError("get_longestqueue_inmem_mainheader error no proper inmem_header found ".to_string()));
                    }
                }
                /*
                if self.unconfirmed_mainblocks_queue_vector[tmp_index].has_parent_unconfirmed_mainblocks_queue() { 
                    tmp_index=self.unconfirmed_mainblocks_queue_vector[tmp_index].get_parent_unconfirmed_mainblocks_queue_index();
                    //self.get_absolute_longest_unconfirmed_mainblocks_queue_inmem_header(tmp_height,tmp_index)?;
                    continue;      
                } else {
                    //return Err(Box::new(std::io::Error::new(ErrorKind::Other, " get_longestqueue_inmem_mainheader error no proper inmem_header found ")));
                    return Err(UnconfirmedMainblocksError::OtherError("get_longestqueue_inmem_mainheader error no proper inmem_header found ".to_string()));
                }
                */
            } 
        }
        //
        //return Err(Box::new(std::io::Error::new(ErrorKind::Other, " get_absolute_longest_unconfirmed_mainblocks_queue_inmem_header error seached too much ")));
        return Err(UnconfirmedMainblocksError::OtherError("get_longestqueue_inmem_mainheader error seached too much".to_string()));
    }
    //get_longest_unconfirmed_mainblocks_queue_block
    pub async fn get_longestqueue_mainblock(&mut self,tmp_height:u32) -> Result<Mainblock,UnconfirmedMainblocksError> {
        let mut tmp_index=self.get_longestqueue_index();
        //self.get_longest_unconfirmed_mainblocks_queue_block_with_index(tmp_height,tmp_index).await?;

        for i in 0..1000 {
            let tmp_start_height=self.unconfirmed_mainblocks_queue_vector[tmp_index].start_height;
            println!("tmp_height {} tmp_start_height {}",tmp_height,tmp_start_height);
            if tmp_height>=tmp_start_height {
                let tmp_block=self.unconfirmed_mainblocks_queue_vector[tmp_index].get_queue_unconfirmed_mainblock(tmp_height).await?;
                return Ok(tmp_block);
            } else  {
                
                match self.unconfirmed_mainblocks_queue_vector[tmp_index].parent_queue_index_option {
                    Some(value) => {
                        tmp_index=value;//self.unconfirmed_mainblocks_queue_vector[tmp_index].get_parent_unconfirmed_mainblocks_queue_index();
                        //self.get_longest_unconfirmed_mainblocks_queue_block_with_index(tmp_height,tmp_index.await).await?;
                        continue;   
                    }
                    None => {
                        //return Err(Box::new(std::io::Error::new(ErrorKind::Other, "get_absolute_longest_unconfirmed_mainblocks_queue_block error no proper block found")));
                        return Err(UnconfirmedMainblocksError::OtherError("get_absolute_longest_unconfirmed_mainblocks_queue_block error no proper block found".to_string()));
                    }
                }        
                /*
                if self.unconfirmed_mainblocks_queue_vector[tmp_index].has_parent_unconfirmed_mainblocks_queue() {
                    tmp_index=self.unconfirmed_mainblocks_queue_vector[tmp_index].get_parent_unconfirmed_mainblocks_queue_index();
                    //self.get_longest_unconfirmed_mainblocks_queue_block_with_index(tmp_height,tmp_index.await).await?;
                    continue;            
                } else {
                    //return Err(Box::new(std::io::Error::new(ErrorKind::Other, "get_absolute_longest_unconfirmed_mainblocks_queue_block error no proper block found")));
                    return Err(UnconfirmedMainblocksError::OtherError("get_absolute_longest_unconfirmed_mainblocks_queue_block error no proper block found".to_string()));
                }*/
            }
        }
        //return Err(Box::new(std::io::Error::new(ErrorKind::Other, "get_absolute_longest_unconfirmed_mainblocks_queue_block error searched too much ")));
        return Err(UnconfirmedMainblocksError::OtherError("get_longest_queue_mainblock error searched too much".to_string()));
    }

    
}

pub struct UnconfirmedMainblocksQueue {
    sd: StorageDirectory,
    start_height: u32,
    unconfirmed_mainheader_vector: Vec<Mainheader>,
    parent_queue_index_option: Option<usize>,
}

impl UnconfirmedMainblocksQueue {
    pub async fn new(
        new_index: usize,
        tmp_start_height: u32,
        tmp_parent_queue_index_option: Option<usize>,
    ) -> Result<Self, UnconfirmedMainblocksError> {
        let tmp_sd_path = format!("TmpUnconfirmedMainblocks/UnconfirmedMainblocksQueue{:03}", new_index);
        directory::create_directory(&tmp_sd_path)
            //.map_err(|_| UnconfirmedMainblocksError::DirectoryError("Failed to create directory".to_string()))?;
            .map_err(UnconfirmedMainblocksError::Io)?; 
        
        let mut tmp_sd = StorageDirectory::new(&tmp_sd_path,String::from("UnconfirmedMainblock")).await?;
        tmp_sd.init().await.map_err(UnconfirmedMainblocksError::StorageDirectoryError)?;       
        Ok(Self {
            sd: tmp_sd,
            start_height: tmp_start_height,
            unconfirmed_mainheader_vector: Vec::new(),
            parent_queue_index_option: tmp_parent_queue_index_option,
        })
    }
    
    pub async fn add_unconfirmed_mainblock_to_queue(&mut self, mb:Mainblock) -> Result<(), UnconfirmedMainblocksError>  {
        let tmpheader=mb.get_mainheader();
        self.unconfirmed_mainheader_vector.push(tmpheader);

        let mb_rawbytes=mb.serialize();
        match self.sd.add_chunk(mb_rawbytes.as_slice()).await {
            Ok(_)=> {
                println!("ChunksStorage add_chunk success");
                Ok(())
            }
            Err(e)=> {
                println!("ChunksStorage add_chunk error {:?}",e);
                Err(UnconfirmedMainblocksError::StorageDirectoryError(e))
            }
        }
    }
    pub fn get_queue_unconfirmed_inmem_header(&self,header_height: u32)-> Result<Mainheader, UnconfirmedMainblocksError> {
        let tmp_start_height=self.start_height;
        Ok(self.unconfirmed_mainheader_vector[(header_height-tmp_start_height) as usize].clone()) 
    }
    pub async fn get_queue_unconfirmed_mainblock(&mut self,block_height: u32)-> Result<Mainblock, UnconfirmedMainblocksError> {
        let tmp_start_height=self.start_height;
        match self.sd.get_chunk(block_height).await {
            Ok(mb_rawbytes)=> {
                println!("ChunksStorage get_chunk success");
                let mb=unserialize_mainblock(mb_rawbytes)?;
                Ok(mb)
            }
            Err(e)=> {
                println!("ChunksStorage get_chunk error {:?}",e);
                Err(UnconfirmedMainblocksError::StorageDirectoryError(e))
            }
        }
    }



}

fn prep_tmp_unconfirmed_mainblocks_directory() -> Result<(), io::Error> {
    if directory::does_directory_exist("TmpUnconfirmedMainblocks") {
        directory::remove_directory_all("TmpUnconfirmedMainblocks")?;
    }
    //directory::create_directory("Tmp")?;
    directory::create_directory("TmpUnconfirmedMainblocks")?;
    Ok(())
}
