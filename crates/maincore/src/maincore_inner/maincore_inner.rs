use tokio::fs;
use tokio::io;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error; 
use utility::storage::storage_directory::StorageDirectory;
use utility::storage::storage_directory::StorageDirectoryError;
use utility::hash::bigint;
use crate::mainheader::mainheader::Mainheader;
use crate::mainblock::mainblock::Mainblock;
use crate::mainblock::mainblock::unserialize_mainblock;
use crate::mainblock::mainblock::MainblockError;

use std::process;

use wallet::wallet_v1::wallet_inner::WalletInner;
use std::sync::{Arc, Mutex};

// MainCoreError Definition
#[derive(Debug, Error)]
pub enum MaincoreInnerError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    //#[error("Custom error: {0}")]
    //Custom(String),

    //#[error("BigInt error: {0}")]
    //BigIntError(String),

    //#[error("Chunk storage error: {0}")]
    //ChunkStorageError(String),

    //#[error("Header serialization error: {0}")]
    //SerializationError(String),
    //////////////////////////////////////////////
    /////////////////////////////////////////////
    //#[error("Buffer reader error: {0}")]
    //BufferReaderError(#[from] BufferReaderError),
    #[error("Storage directory error: {0}")]
    StorageDirectoryError(#[from] StorageDirectoryError),
    #[error("Mainblock error: {0}")]
    MainblockError(#[from] MainblockError),
}

// Define the MainCoreInner struct
//#[derive(Debug)] // Implementing Debug trait for MainCoreInner
pub struct MaincoreInner {
    mci_path: PathBuf,
    genesis_mainblock:Option<Mainblock>,
    main_sd: StorageDirectory,
    confirmed_mainheader_vector:Vec<Mainheader>,
    confimation_depth:usize,
    //syncpool:Syncpool,
    //mainstate:Mainstate,
    //txspool:Maintxspool,
    //miner:Miner,
    
}

impl MaincoreInner{
    /// Creates a new `StorageDirectory` instance.
    pub async fn new<P: AsRef<Path>>(mci_path: P) -> Result<Self,MaincoreInnerError> {
        let mci_path = mci_path.as_ref().to_path_buf();
        if !mci_path.exists() {
            fs::create_dir_all(&mci_path).await?;
        }
        
        println!("MaincoreInner - path:{:?} ", mci_path);

        let sd_sub_path_buf=PathBuf::from("Mainblocks");// can be string but should be PathBuf
        let sd_path_buf=mci_path.join(sd_sub_path_buf);
        let main_sd= StorageDirectory::new(sd_path_buf,String::from("Mainblock")).await?;

        //Ok(Self { mci_path,main_sd, confimation_depth:100})

        Ok(Self { 
            mci_path,
            main_sd,
            genesis_mainblock:None,
            confirmed_mainheader_vector: Vec::new(),//
            confimation_depth: 6,
            //syncpool:Syncpool::new(),
            //mainstate:tmp_ms,
            //txspool:Maintxspool::new(),
            //miner:Miner::new(),
        })
    }
    pub async fn init(&mut self)-> Result<(),MaincoreInnerError> {
        self.init_storage_directory().await?;
        //self.syncpool.init()?;
        Ok(())
    }
    pub async fn init_storage_directory(&mut self)-> Result<(),MaincoreInnerError> {
        match self.main_sd.init().await {
            Ok(_)=> {
                println!("StorageDirectory init success");
                //
                let index=self.main_sd.get_storage_files_last_index();
                //match index_result {
                //    Some(index) =>{
                        println!("get_storage_files_last_index: {}", index);
                //    },
                //    None => {
                //        println!("storage files last index not initialized");

                //    }, 
                //}        
                return Ok(());
            }
            Err(e)=> {
                println!("StorageDirectory init error {:?}",e);
                return Err(MaincoreInnerError::StorageDirectoryError(e))
            }
        }
        Ok(())
    }
    pub fn get_confirmed_mainblocks_last_index(&self) -> u32 {
        //let index_result=self.main_sd.get_storage_files_last_index();
        //match index_result {
        //    Some(index) => return index+1,//println!("get_storage_files_last_index: {}", index),
        //    None => return 0,//println!("storage files last index not initialized"), 
        //}
        self.main_sd.get_storage_files_last_index()
    }
    pub fn add_genesis_mainblock(&mut self,mb: Mainblock)-> Result<(),MaincoreInnerError> {
        let tmpheader=mb.get_mainheader();
        self.confirmed_mainheader_vector.push(tmpheader);
        self.genesis_mainblock=Some(mb);
        Ok(())
    }
    pub async fn add_confirmed_mainblock(&mut self,mb: Mainblock)-> Result<(),MaincoreInnerError> {
        println!("************ add_confirmed_mainblock");
        let tmpheader=mb.get_mainheader();
        self.confirmed_mainheader_vector.push(tmpheader);
        if self.confirmed_mainheader_vector.len()>1 {
            /*
            for i in 1..self.confirmed_mainheader_vector.len(){
                println!("i {}",i);
                println!("self.confirmed_mainheader_vector[i].get_timestamp() {} self.confirmed_mainheader_vector[i-1].get_timestamp() {}",self.confirmed_mainheader_vector[i].get_timestamp(),self.confirmed_mainheader_vector[i-1].get_timestamp());
                process::exit(1);
            }
            */
            println!("self.confirmed_mainheader_vector.len() {}",self.confirmed_mainheader_vector.len());
            let tmp_height=self.confirmed_mainheader_vector.len()-1;
            let deltatimestamp=self.confirmed_mainheader_vector[tmp_height].get_timestamp()-self.confirmed_mainheader_vector[tmp_height-1].get_timestamp();
            println!("deltatimestamp {}",deltatimestamp);
            /*
            if deltatimestamp==0 {
                process::exit(1);
            }
            */
        }
        //All the txs that have been included in a confimred block will be removed from the txspoll (self.txspool.remove(tmp_hash))
        //TODONOW with self.txspool.remove(tmp_hash)

        //All the txs that have been frozen because they have been included in a certain block height WILL BE reset (reset_tx_with_mainblock_height)
        //TODONOW with reset_tx_with_mainblock_height 

        let mb_rawbytes=mb.serialize();
        match self.main_sd.add_chunk(mb_rawbytes.as_slice()).await {
            Ok(_)=> {
                println!("StorageDirectory add_chunk success");
                Ok(())
            }
            Err(e)=> {
                println!("StorageDirectory add_chunk error {:?}",e);
                return Err(MaincoreInnerError::StorageDirectoryError(e))
            }
        }
    }
    pub async fn get_mainblock(&mut self,block_height: u32)-> Result<Mainblock,MaincoreInnerError>{
        if block_height==0 {
            match self.genesis_mainblock.clone() {
                Some(mb)=> return Ok(mb),
                None => panic!("Fatal get_mainblock 0 failed no genesis_mainblock"),
            }
        }
        match self.main_sd.get_chunk(block_height).await {
            Ok(mb_rawbytes)=> {
                println!("StorageDirectory get_chunk success");
                println!("mb_rawbytes {:?}",mb_rawbytes);
                let mb=unserialize_mainblock(mb_rawbytes)?;
                Ok(mb)
            }
            Err(e)=> {
                println!("StorageDirectory get_chunk error {:?}",e);
                Err(MaincoreInnerError::StorageDirectoryError(e))
            }
        }
    }
    //
    pub async fn load_confirmed_mainheaders(&mut self) -> Result<(),MaincoreInnerError> {
        let tmpblocks_count=self.get_confirmed_mainblocks_last_index();
        println!("loading mainheaders - number of mainblocks {}",tmpblocks_count);
        if tmpblocks_count==0 {
            //Err(e) => {
                //return Err(eprintln!("))
                println!("load_headers finished-no mainheaders");
                return Ok(());
                //return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Maincore ChunckStorage is empty get_blocks_count()==0")))
           // }
        }
        for i in 0..tmpblocks_count {
            let tmpheader=self.get_mainblock(i).await?;
            self.confirmed_mainheader_vector.push(tmpheader.get_mainheader());
            /*
            match self.get_mainheader(i).await {
                Ok(tmpheader)=> {
                    self.confirmed_mainheader_vector.push(tmpheader);
                    println!("loaded mainheader {}",i);
                    //return Ok(());
                }
                Err(e)=> {
                    println!("load_headers error {:?}",e);
                    return Err(e);
                }
            }
            */
        }
        println!("tmpblocks_count {}",tmpblocks_count);
        let tmp_height=self.confirmed_mainheader_vector.len();
        println!("load_headers finished with {} mainheaders loaded",tmp_height);
        Ok(())
    }
    /*
    pub async fn get_mainheader(&mut self,header_height: usize)-> Result<Mainheader, MaincoreInnerError> {
        
        Ok(mb.get_mainheader())

    }
    */
    pub fn get_last_confirmed_inmem_mainheader(&self)-> Result<Mainheader, MaincoreInnerError> {
        let last_block_height=(self.confirmed_mainheader_vector.len())-1;
        //let last_block_height1=self.get_blocks_count()-1;
        //println!("*********** last_block_height {} {}",last_block_height1,last_block_height);
        Ok(self.confirmed_mainheader_vector[last_block_height].clone())
        
    }
    pub fn get_longestchain_inmem_header(&self,header_height: usize)-> Result<Mainheader, MaincoreInnerError> {
        //TODO inclued unconfirmed_inmem_mainheader
        Ok(self.confirmed_mainheader_vector[header_height].clone()) 
    }
    pub fn get_confirmed_inmem_mainheader(&self,header_height: usize)-> Result<Mainheader, MaincoreInnerError> {
        Ok(self.confirmed_mainheader_vector[header_height].clone()) 
    }
    pub fn compute_longestchain_bits(&mut self,tmp_height_32:u32)-> Result<u32, MaincoreInnerError>  {
        //let newbits:u32;
        //let tmp_height=self.confirmed_mainheader_vector.len();
        let tmp_height=tmp_height_32 as usize;
        let prev_mainheader=self.get_longestchain_inmem_header(tmp_height-1)?;
        //
        if tmp_height % 1440 ==0 {//it was //if tmp_height % 4032 ==0 {
            let mut summedtimestamp:u64=0;
            //for i in (tmp_height-4031)..(tmp_height) {
            for i in (tmp_height-1430)..(tmp_height) {
                let longestchain_inmem_header_i=self.get_longestchain_inmem_header(i)?;
                let longestchain_inmem_header_i_less_1=self.get_longestchain_inmem_header(i-1)?;
                let deltatimestamp=longestchain_inmem_header_i.get_timestamp()-longestchain_inmem_header_i_less_1.get_timestamp();
                //let deltatimestamp=self.get_longestchain_inmem_header(i).get_timestamp()-self.get_longestchain_inmem_header(i-1).get_timestamp();
                if deltatimestamp==0 {
                    println!("check deltatimestamp {} for i {}",deltatimestamp,i);
                    println!("self.get_longestchain_inmem_header(i).get_timestamp() {} self.get_longestchain_inmem_header(i-1).get_timestamp() {}",longestchain_inmem_header_i.get_timestamp(),longestchain_inmem_header_i_less_1.get_timestamp());
                    //process::exit(1);
                }
                summedtimestamp+=deltatimestamp as u64;
            }
            let bitsbigint=bigint::biguint_from_compact(prev_mainheader.get_bits());
            //summedtimestamp=1430*60;
            let newbitsbigint=(bitsbigint.clone()*bigint::biguint_from_u64(summedtimestamp as u64))/bigint::biguint_from_u64((1430*60) as u64);
            
            //it was //let newbitsbigint=bitsbigint.clone()*bigint::biguint_from_u64(summedtimestamp as u64)/bigint::biguint_from_u64((4031*300) as u64);
            
            println!("summed over interval from {} to {}",tmp_height-1430,tmp_height-1);
            //it was //println!("summed over interval from {} to {}",tmp_height-4031,tmp_height-1);
            println!("summedtimestamp {} idealtime {}",summedtimestamp,1430*60);
            //it was //println!("summedtimestamp {} idealtime {}",summedtimestamp,4031*300);
            //println!("check deltatimestamp {}",self.get_longestchain_inmem_header(1.get_timestamp()-self.get_longestchain_inmem_header(0).get_timestamp()));
            println!("oldbits {} newbits {}",bitsbigint,newbitsbigint);
            println!("oldbits compact {} newbits compact {}",bigint::compact_from_biguint(bitsbigint.clone()),bigint::compact_from_biguint(newbitsbigint.clone()));
            println!("newbits {}",bigint::compact_from_biguint(newbitsbigint.clone()));
            println!("oldbits {}",bigint::compact_from_biguint(bitsbigint.clone()));
            if newbitsbigint!=bitsbigint {
                process::exit(1);
            }
            
            
            //return Ok(bigint::compact_from_biguint(newbitsbigint.clone()));
            return Ok(bigint::compact_from_biguint(bitsbigint.clone()));

        } else {
            return Ok(prev_mainheader.get_bits());
        }

    }
    //
    pub async fn sync_wallet(&mut self,mut shared_walletinner:Arc<Mutex<WalletInner>>) {
        //for tmpindex in (wltinner.get_last_known_height()+1)..(self.get_blocks_count() as u32) {
        //before wltinner.get_last_known_height() should be < mc.get_blocks_count()

        let mut wltinner=shared_walletinner.lock().unwrap();
        println!("sync_wallet started!");
        //let tmpindex=wltinner.get_last_known_height()+1;
        let tmp_last_height=self.get_confirmed_mainblocks_last_index();
        for tmpindex in wltinner.get_last_known_height()+1..=tmp_last_height {
            match self.get_mainblock(tmpindex).await{
                Ok(tmpmb)=>{
                    println!("sync_wallet againt block {} nb transactions {}",tmpindex,tmpmb.transactions.len());
                    for i in 0..tmpmb.transactions.len() {
                        //let update_assets_result=wltinner.update_assets(tmpmb.transactions[i].clone());
                        println!("sync_wallet {:?}",tmpmb.transactions[i].clone());
                        match wltinner.update_resources(tmpmb.transactions[i].clone()) {
                            Ok(_) => {},//println!("update_assets successful!"),
                            Err(err) => println!("update_assets Error: {}", err),
                          }
                    }
                    wltinner.set_last_known_height(tmpindex);
                    //println!("sync_wallet end! wallet {:?}",wltinner);
                }
                Err(e)=> {
                    println!("get_block error {:?}",e);
                    //Err(Box::new(e))
                }   
            }
        }
    }

}