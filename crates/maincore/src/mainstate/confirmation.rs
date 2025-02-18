use utility::hash::hash::Hash;


use super::mainstate::MAINTX_OUT_STATE_VALUE_IDENTIFIER_UNSPENT;
use super::mainstate::MAINTX_OUT_STATE_VALUE_IDENTIFIER_SPENT;
use maintx::maintx::maintx::Maintx;

use super::mainstate::Mainstate;
use super::mainstate::MainstateError;

use super::maintx_state::MaintxStateError;
use thiserror::Error;
use crate::mainblock::mainblock::Mainblock;

use utility::buffer::buffer_reader::BufferReaderError;

#[derive(Debug, Error)]
pub enum ConfirmationError {
    //#[error("I/O error: {0}")]
    //IoError(#[from] io::Error),

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
    #[error("Buffer reader error: {0}")]
    BufferReaderError(#[from] BufferReaderError),
    #[error("Maintx state error: {0}")]
    MaintxStateError(#[from] MaintxStateError),
    /*
    #[error("Storage directory error: {0}")]
    StorageDirectoryError(#[from] StorageDirectoryError),
    #[error("Mainblock error: {0}")]
    MainblockError(#[from] MainblockError),
    #[error("UnconfirmedMainblocksError error: {0}")]
    UnconfirmedMainblocksError(#[from] UnconfirmedMainblocksError),
    #[error("{0}")]
    OtherError(String),
    */
    //#[error("Mainheader error: {0}")]
    //MainheaderError(#[from] MainheaderError),
}

impl Mainstate {
    pub async fn confirm_maintx(&mut self,tmp_maintx:Maintx,maintx_mainblock_height:u32,maintx_postion:u32)->Result<(),ConfirmationError>{
        let tmp_maintx_hash=tmp_maintx.compute_hash();
        self.insert_maintx_state(tmp_maintx_hash.clone(),maintx_mainblock_height,maintx_postion).await?;
        println!(" insert_maintx_state tmp_maintx_hash {:?},maintx_mainblock_height {}, maintx_postion {}",tmp_maintx_hash,maintx_mainblock_height,maintx_postion);
        //
        for i in 0..tmp_maintx.vin.len() {
            if tmp_maintx.vin[i].is_ecdsa() {
                self.insert_maintx_out_state(tmp_maintx_hash.clone(),i as u32,MAINTX_OUT_STATE_VALUE_IDENTIFIER_SPENT).await?;
            }
        }
        //
        for j in 0..tmp_maintx.vout.len() {
            if tmp_maintx.vout[j].is_ecdsa() {
                self.insert_maintx_out_state(tmp_maintx_hash.clone(),j as u32,MAINTX_OUT_STATE_VALUE_IDENTIFIER_UNSPENT).await?;
                println!("insert_maintx_out_state hash {:?} index {}",tmp_maintx_hash,j);
            }
        }

        Ok(())
    }
    pub async fn confirm_block(&mut self,mb: Mainblock,height:u32)-> Result<(), ConfirmationError>  {
        println!("confirm_block mb.transactions.len() {}",mb.transactions.len());
        for i in 0..mb.transactions.len() {
            self.confirm_maintx(mb.transactions[i].clone(),height,i as u32).await?;
        }
        Ok(())
    }

}

