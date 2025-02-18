use tokio::fs;
use tokio::io;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error; 
use utility::hash::hash::Hash;

use maintx::maintx::maintx::Maintx;
use maintx::maintx_in::maintx_in::MaintxInError;
use maintx::maintx_out::maintx_out::MaintxOutError;
use utility::storage::storage_directory::StorageDirectory;
use utility::storage::storage_directory::StorageDirectoryError;
use utility::hash::bigint;
use crate::mainheader::mainheader::Mainheader;
use crate::mainblock::mainblock::Mainblock;
use crate::mainblock::mainblock::unserialize_mainblock;
use crate::mainblock::mainblock::MainblockError;

use crate::maincore_inner::unconfirmed_mainblocks::UnconfirmedMainblocks;
use crate::maincore_inner::unconfirmed_mainblocks::UnconfirmedMainblocksError;

use crate::maincore_inner::miner::Miner;
use crate::maincore_inner::maintxs_pool::MaintxsPool;
use std::process;

use crate::mainstate::mainstate::Mainstate;
use crate::mainstate::mainstate::MainstateError;

use crate::mainstate::maintx_state::MaintxStateError;
use crate::mainstate::confirmation::ConfirmationError;
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
    #[error("UnconfirmedMainblocksError error: {0}")]
    UnconfirmedMainblocksError(#[from] UnconfirmedMainblocksError),
    #[error("{0}")]
    OtherError(String),
    #[error("MaintxInError error: {0}")]
    MaintxInError(#[from] MaintxInError),
    #[error("MaintxOutError error: {0}")]
    MaintxOutError(#[from] MaintxOutError),
    #[error("Mainstate error: {0}")]
    MainstateError(#[from] MainstateError),
    #[error("MaintxStateError error: {0}")]
    MaintxStateError(#[from] MaintxStateError),
    #[error("ConfirmationError error: {0}")]
    ConfirmationError(#[from] ConfirmationError),
    

    
}

// Define the MainCoreInner struct
//#[derive(Debug)] // Implementing Debug trait for MainCoreInner
pub struct MaincoreInner {
    mci_path: PathBuf,
    genesis_mainblock:Option<Mainblock>,
    main_sd: StorageDirectory,
    confirmed_mainheader_vector:Vec<Mainheader>,
    confirmation_depth:u32,
    unconfirmed_mainblocks:UnconfirmedMainblocks,
    mainstate:Mainstate,
    maintxs_pool:MaintxsPool,
    miner:Miner,
    
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
        let tmp_ms=Mainstate::new()?;
        Ok(Self { 
            mci_path,
            main_sd,
            genesis_mainblock:None,
            confirmed_mainheader_vector: Vec::new(),//
            confirmation_depth: 6,
            unconfirmed_mainblocks:UnconfirmedMainblocks::new(),
            mainstate:tmp_ms,
            maintxs_pool:MaintxsPool::new(),
            miner:Miner::new(),
        })
    }
    pub async fn init(&mut self)-> Result<(),MaincoreInnerError> {
        self.init_storage_directory().await?;
        self.unconfirmed_mainblocks.init()?;
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
    pub fn get_confirmed_mainblocks_last_height(&self) -> u32 {
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
        //All the txs that have been included in a confimred block will be removed from the txspoll (self.maintxs_pool.remove(tmp_hash))
        //TODONOW with self.maintxs_pool.remove(tmp_hash)

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
    ////////////////////////////////////////////////////////////////////////////////
    pub async fn process_blocks_confirmation(&mut self)-> Result<(), MaincoreInnerError>  {
        let tmp_start_height=self.get_confirmed_mainblocks_last_height()+1;
        let mut tmp_final_height:u32=0;
        if self.unconfirmed_mainblocks.get_longestqueue_last_height()+1>self.confirmation_depth {
            tmp_final_height=self.unconfirmed_mainblocks.get_longestqueue_last_height()+1-self.confirmation_depth;
        }
        println!("process_blocks_confirmation tmp_start_height {} tmp_final_height {}",tmp_start_height,tmp_final_height);
        
        for tmp_height in tmp_start_height..tmp_final_height {
            let mb=self.get_longestchain_mainblock(tmp_height).await?;//TODONOW implement get_longest_chain_block
            println!("start confirm_block for height {}",tmp_height);
            self.add_confirmed_mainblock(mb.clone()).await?;
            /////////////////////////////////////////////////////////
            self.mainstate.confirm_block(mb,tmp_height as u32).await?;
            /////////////////////////////////////////////////////////
        }
        Ok(())
    }
    //
    pub async fn add_unconfirmed_mainblock(&mut self,mb: Mainblock)-> Result<(), MaincoreInnerError>  {
        // first we check if this block links to the last block
        let tmp_last_inmem_header=self.get_last_confirmed_inmem_mainheader()?;
        let tmp_mb_hearder_prev_hash1=mb.header.get_prev_hash();
        if (tmp_last_inmem_header.get_hash())==(tmp_mb_hearder_prev_hash1) {
            let tmp_start_height=self.get_confirmed_mainblocks_last_height()+1;
            let tmp_new_queue_index=self.unconfirmed_mainblocks.new_unconfirmed_mainblocks_queue(tmp_start_height,None).await?;
            self.unconfirmed_mainblocks.add_unconfirmed_mainblock_to_queue_with_index(tmp_new_queue_index,mb.clone()).await?;
            println!("add_unconfirmed_block new block attaches to the confirmed chain");
            return Ok(());
        }
        // else we check if there is an associated unconfirmed_mainblocks queue
        for tmpindexi in 0..self.unconfirmed_mainblocks.get_queues_count() {
            for tmpindexj in self.unconfirmed_mainblocks.get_queue_unconfirmed_mainblocks_start_height(tmpindexi)..=self.unconfirmed_mainblocks.get_queue_unconfirmed_mainblocks_last_height(tmpindexi) {
                let tmp_unconfirmed_header=self.unconfirmed_mainblocks.get_unconfirmed_inmem_header_with_queue_index_and_height(tmpindexi,tmpindexj)?;
                let tmp_mb_hearder_prev_hash2=mb.header.get_prev_hash();
                if (tmp_unconfirmed_header.get_hash())==(tmp_mb_hearder_prev_hash2) {
                    if tmpindexj==self.unconfirmed_mainblocks.get_queue_unconfirmed_mainblocks_last_height(tmpindexi) {
                        //here the unconfirmed block can be linked to the last block of this tmpindexi unconfirmed_mainblocks queue
                        self.unconfirmed_mainblocks.add_unconfirmed_mainblock_to_queue_with_index(tmpindexi,mb.clone()).await?;
                        return Ok(());
                    } else {
                        let tmp_start_height=self.unconfirmed_mainblocks.get_start_height_with_queue_index(tmpindexi)+tmpindexj+1;//self.get_blocks_count();
                        let tmp_new_queue_index=self.unconfirmed_mainblocks.new_unconfirmed_mainblocks_queue(tmp_start_height,Some(tmpindexi)).await?;
                        self.unconfirmed_mainblocks.add_unconfirmed_mainblock_to_queue_with_index(tmp_new_queue_index,mb.clone()).await?;
                        return Ok(());
                    }
                }
            }
            
        }
        Err(MaincoreInnerError::OtherError("add_unconfirmed_block error - unconfirmed_block is not related with confirmed mainchain".to_string()))
    }
    //
  
    ////////////////////////////////////////////////////////////////////////////////
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
        let tmpblocks_count=self.get_confirmed_mainblocks_last_height();
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
    ////////////////////////////////////////////////////////////////
    /*
    pub fn get_longestchain_last_inmem_mainheader(&self)-> Result<Mainheader, MaincoreInnerError> {
        if self.unconfirmed_mainblocks.get_longestqueue_mainblocks_count()==0 {
            return self.get_last_inmem_mainheader();
        } 
        let tmp_last_inmem_header_height=self.unconfirmed_mainblocks.get_longestchain_mainblocks_count()-1;
        //println!("tmp_last_inmem_header_height {}",tmp_last_inmem_header_height);
        self.unconfirmed_mainblocks.get_longestchain_inmem_mainheader(tmp_last_inmem_header_height)
    }
    */
    pub fn get_longestchain_last_height(&self)-> Result<u32, MaincoreInnerError> {

            Ok(self.unconfirmed_mainblocks.get_longestqueue_last_height())
   
    }
    pub fn get_longestchain_inmem_mainheader(&self,tmp_height:u32)-> Result<Mainheader, MaincoreInnerError> {
        let last_inmem_header_height=(self.confirmed_mainheader_vector.len())-1;
        println!("tmp_height {} last_inmem_header_height {}",tmp_height,last_inmem_header_height);
        if (tmp_height as usize)<=last_inmem_header_height {
            return self.get_confirmed_inmem_mainheader(tmp_height);
        } else {
            //return self.unconfirmed_mainblocks.get_longestqueue_inmem_mainheader(tmp_height);
            println!("return Ok(self.unconfirmed_mainblocks.get_longestqueue_inmem_mainheader(tmp_height)?);");
            return Ok(self.unconfirmed_mainblocks.get_longestqueue_inmem_mainheader(tmp_height)?);
        }
    }
    //
    pub async fn get_longestchain_mainblock(&mut self,tmp_height:u32)-> Result<Mainblock, MaincoreInnerError> {
        let last_block_height=self.get_confirmed_mainblocks_last_height();
 
        if tmp_height<=last_block_height {
            return self.get_mainblock(tmp_height).await;
        } else {
            //self.syncpool.get_absolute_longest_syncchain_inmem_header(tmp_height)
            return Ok(self.unconfirmed_mainblocks.get_longestqueue_mainblock(tmp_height).await?);
        }
    }
    /*
    pub fn get_longestchain_inmem_mainheader(&self,header_height: u32)-> Result<Mainheader, MaincoreInnerError> {
        //TODO inclued unconfirmed_inmem_mainheader
        Ok(self.confirmed_mainheader_vector[header_height].clone()) 
    }
    */
    /////////////////////////////////////////////////////////////////
    pub fn get_confirmed_inmem_mainheader(&self,header_height: u32)-> Result<Mainheader, MaincoreInnerError> {
        Ok(self.confirmed_mainheader_vector[header_height as usize].clone()) 
    }
    pub fn compute_longestchain_bits(&mut self,tmp_height:u32)-> Result<u32, MaincoreInnerError>  {
        //let newbits:u32;
        //let tmp_height=self.confirmed_mainheader_vector.len();
        //let tmp_height=tmp_height_32 as usize;
        let prev_mainheader=self.get_longestchain_inmem_mainheader(tmp_height-1)?;
        //
        if tmp_height % 1440 ==0 {//it was //if tmp_height % 4032 ==0 {
            let mut summedtimestamp:u64=0;
            //for i in (tmp_height-4031)..(tmp_height) {
            for i in (tmp_height-1430)..(tmp_height) {
                let longestchain_inmem_header_i=self.get_longestchain_inmem_mainheader(i)?;
                let longestchain_inmem_header_i_less_1=self.get_longestchain_inmem_mainheader(i-1)?;
                let deltatimestamp=longestchain_inmem_header_i.get_timestamp()-longestchain_inmem_header_i_less_1.get_timestamp();
                //let deltatimestamp=self.get_longestchain_inmem_mainheader(i).get_timestamp()-self.get_longestchain_inmem_mainheader(i-1).get_timestamp();
                if deltatimestamp==0 {
                    println!("check deltatimestamp {} for i {}",deltatimestamp,i);
                    println!("self.get_longestchain_inmem_mainheader(i).get_timestamp() {} self.get_longestchain_inmem_mainheader(i-1).get_timestamp() {}",longestchain_inmem_header_i.get_timestamp(),longestchain_inmem_header_i_less_1.get_timestamp());
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
            //println!("check deltatimestamp {}",self.get_longestchain_inmem_mainheader(1.get_timestamp()-self.get_longestchain_inmem_mainheader(0).get_timestamp()));
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
        let tmp_last_height=self.get_confirmed_mainblocks_last_height();
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
    //////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////
    pub async fn mine(&mut self){
        println!("****** maincore_inner unconfirmed_mainblocks.get_longestqueue_last_height {}",self.unconfirmed_mainblocks.get_longestqueue_last_height());
        println!("Mining ...");
        // Generate a random integer in the range from 1 to 100 (inclusive)
        //let random_number = ptest1_utility::utility::generate_random_number(1,num_addresses)-1; // Generates a random number between 0 and 10000
        //let random_number=generate_random_number(1,num_addresses).unwrap()-1;
        //println!("Random {}",random_number);
        let tmpheight=self.unconfirmed_mainblocks.get_longestqueue_last_height()+1;//self.get_blocks_count();
        println!("******** Mining For Height {}",tmpheight);
        let tmp_newbits =self.compute_longestchain_bits(tmpheight).unwrap();//TODONOW recode this 
        //
        let tmp_last_inmem_header=self.get_longestchain_inmem_mainheader(tmpheight-1).unwrap();//self.get_last_inmem_header().unwrap();
        
        let tmp_prioritized_txs=self.maintxs_pool.get_prioritized_maintxs();
        println!("---------> get_prioritized_txs {:?}",tmp_prioritized_txs);
        let mut total_fees:u64=0;
        for tmpi in 0..tmp_prioritized_txs.len() {
            match self.compute_fee(tmp_prioritized_txs[tmpi].clone()).await{
                Ok(tmp_fee)=>{
                    total_fees+=tmp_fee;
                }
                Err(e)=>{
                    println!("Error {:?}",e)
                }
            }
        }
        
        //let tmp_prioritized_txs=Vec::new();
        //let total_fees:u64=0;
        ///////////////////////////////////////////
        //match prem_mine_mainblock(tmp_mc.get_last_inmem_header().unwrap(),tmpheight,tmp_mc.get_newbits(),addresses[random_number].clone()) {
        
        match self.miner.mine_mainblock(tmp_last_inmem_header,tmpheight,tmp_newbits,tmp_prioritized_txs.clone(),total_fees) {
            Ok(newmb)=>{
                /*  - this was tempory
                match self.mainstate.confirm_block(newmb.clone(),tmpheight as u32).await {
                    Ok(_)=>{
                        println!("confirm_block success");
                    }
                    Err(e)=>{
                        println!("confirm_block error {:?}",e);
                    }
                }*/
                ////////////////////////////////
                println!("mine_mainblock success {:?}",newmb);
                match self.add_unconfirmed_mainblock(newmb).await {//TODONOW replace with add_unconfirmed_block
                    Ok(_)=> {
                        ////////////////////////////////

                        println!("add newmb (mined mainblock) success");
                        for tmp_tx in &tmp_prioritized_txs {
                            // Code to be executed for each iteration
                            let tmp_hash=tmp_tx.compute_hash();
                            self.maintxs_pool.set_tx_to_mainblock_height(tmp_hash,tmpheight);
                        }
                        //process::exit(1);
                        
                        
                        
                    }
                    Err(e)=> {
                        println!("add newmb error {:?}",e);
                    }
                }
            }
            Err(e)=> {
                println!("mine_mainblock error {:?}",e);
            }
        }
    }
    //////////////////////////////////////////////////////////////////////////////////////
    pub async fn compute_fee(&mut self,tx:Maintx)->Result<u64, MaincoreInnerError> {
        let mut total_txin_amount:u64=0;
        let mut total_txout_amount:u64=0;
        
        for i in 0..tx.vin.len() {
            let hash=tx.vin[i].get_hash()?;
            let index=tx.vin[i].get_index()?;
            //println!("compute_fee hash {:?} index {}",hash,index);
            let tmp_txin_amount=self.get_txout_amount(hash,index).await?;
            total_txin_amount+=tmp_txin_amount;
        }
        for j in 0..tx.vout.len() {
            total_txout_amount+=tx.vout[j].get_value()?;
        }
        let tmpfee=total_txin_amount-total_txout_amount;
        println!("compute_fee tmpfee {}",tmpfee);
        return Ok(tmpfee);
    }
    pub async fn get_txout_amount(&mut self,hash:Hash,index:u32)->Result<u64, MaincoreInnerError> {
        if !(self.mainstate.is_maintx_out_unspent(hash.clone(),index).await?) {
            return Ok(0);
        } else {
            let get_result1 = self.mainstate.get_maintx_state(hash.clone());
            match get_result1.await {
                Ok((tx_mainblock_height,tx_postion)) =>{
                    println!("****** get_tx_state executed successfully {} {}",tx_mainblock_height,tx_postion);
                    match self.get_mainblock(tx_mainblock_height).await{
                        Ok(tmpmb)=>{
                            if (tx_postion as usize)>=tmpmb.transactions.len(){
                                return Ok(0);
                            }
                            let tmptx=tmpmb.transactions[tx_postion as usize].clone();
                            if (index as usize)>=tmptx.vout.len(){
                                return Ok(0);
                            }
                            //return tmptx.vout[index as usize].get_value();
                            return Ok(tmptx.vout[index as usize].get_value()?);

                        }
                        Err(e)=> {
                            println!("get_block error {:?}",e);
                            //return 0;
                            Err(e)
                        }   
                    }
                },
                Err(err) => {
                    println!("Error: {}", err);
                    //Err(err)
                    Err(MaincoreInnerError::MaintxStateError(err))

            },
            }
        }
    }
    //////////////////////////////////////////////////////////////////////////////////////
    pub fn insert_new_maintx_in_txpool(&mut self,newtx:Maintx){
        //TODO compute priority using tmp_mc.compute_fee(tx);
        self.maintxs_pool.insert_new_maintx(newtx,1.0);
    }
    pub fn add_address(&mut self,tmp_address: Hash){
        //self.miner.addresses_vector.push(tmp_address);
        self.miner.add_address(tmp_address);
    }
    //////////////////////////////////////////////////////////////////////////////////////

}