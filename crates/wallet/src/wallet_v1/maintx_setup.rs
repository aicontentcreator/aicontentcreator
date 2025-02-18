use thiserror::Error;

use utility::hash::hash::Hash;
use maintx::maintx::maintx::Maintx;
use maintx::maintx_in::maintx_in::new_ecdsa_maintx_in;
use maintx::maintx_out::maintx_out::new_ecdsa_maintx_out;
use crate::wallet_v1::wallet_inner::WalletInner;
use crate::wallet_v1::wallet_inner::WalletInnerError;

use utility::ecdsa::ecdsa::sign_messagehash;
use utility::ecdsa::ecdsa::EcdsaKeySetError;

use std::sync::{Arc, Mutex};

//
// MainCoreError Definition
#[derive(Debug, Error)]
pub enum MaintxSetupError {
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
    //#[error("Buffer reader error: {0}")]
    //BufferReaderError(#[from] BufferReaderError),
    /*
    #[error("Storage directory error: {0}")]
    StorageDirectoryError(#[from] StorageDirectoryError),
    #[error("Mainblock error: {0}")]
    MainblockError(#[from] MainblockError),
    #[error("UnconfirmedMainblocksError error: {0}")]
    UnconfirmedMainblocksError(#[from] UnconfirmedMainblocksError),

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
    
*/
    //WalletInnerError
    #[error("WalletInnerError error: {0}")]
    WalletInnerError(#[from] WalletInnerError),
    //EcdsaKeySetError
    #[error("EcdsaKeySetError error: {0}")]
    EcdsaKeySetError(#[from] EcdsaKeySetError),
    #[error("{0}")]
    OtherError(String),
}

//
pub fn new_send_to_address_transaction(mut shared_wallet_inner: Arc<Mutex<WalletInner>>,value: u64, fee: u64, pubkeyhash: Hash) ->  Result<Maintx, MaintxSetupError>  {
    let mut tmp_wallet_inner = shared_wallet_inner.lock().unwrap();
    let (tmp_vresource,tmp_vresource_indexes,total_value)= tmp_wallet_inner.get_available_unspent_resources(value+fee);
    if tmp_vresource.len()==0 {
        //NOT ENOUGH VALUE 
        //let error: Box<dyn Error> = "not enough globals in the wallet".into();
        return Err(MaintxSetupError::OtherError("not enough globals in the wallet".to_string()))
    }
    //tmp_wallet_inner.get_random_keypair()?;
    //let new_wallet_address=tmp_wallet_inner.get_last_address()?;

    let tmp_random_keyset=tmp_wallet_inner.get_random_keyset()?;
    let new_wallet_address=tmp_random_keyset.get_address();

    let mut tx = Maintx {
        version: 1,
        vin: Vec::new(),
        vout: Vec::new(),//vec![new_ecdsa_txout(value + fee, pubkeyhash)],
    };
    //tx.vin.push(tx::tx::new_ecdsa_txin(inhash: Hash, index: u32, pubkeycompressedbytes: Vec<u8>))
    for i in 0..tmp_vresource.len() {
        tx.vin.push(new_ecdsa_maintx_in(tmp_vresource[i].hash.clone(),tmp_vresource[i].index, tmp_wallet_inner.get_public_key_compressed_bytes(tmp_vresource[i].key_index)?))
    }
    //
    tx.vout.push(new_ecdsa_maintx_out(value, pubkeyhash));
    tx.vout.push(new_ecdsa_maintx_out(total_value-value-fee, new_wallet_address));
    //
    let tmp_tx_hash=tx.compute_hash();
    //pub fn sign_messagehash(kp: &KeyPair, message_hash: Hash) -> Result<Signature, Box<dyn Error>> {
    for i in 0..tmp_vresource.len() {
        let tmp_kp=tmp_wallet_inner.get_keypair(tmp_vresource[i].key_index)?;
        tx.vin[i].set_signature(sign_messagehash(&tmp_kp,tmp_tx_hash.clone())?);
    }
    //tmp_wallet_inner.category+=1;
    for i in 0..tmp_vresource_indexes.len(){
        tmp_wallet_inner.set_resource_as_unavailable(tmp_vresource_indexes[i]);
    }
    println!("from new_send_to_address_transaction Wallet {:?} ",tmp_wallet_inner);
    Ok(tx)
}
//
/*
//extern crate utility;
//extern crate tx;

use utility::hashing;
use utility::hashing::Hash;


use tx;
fn main() {
    println!("Hello, world!");
    let newaddress: Hash = Hash::new([
        0xd5, 0x67, 0xeb, 0x23, 0xc7, 0xf9, 0xb6, 0xb1, 
        0x1e, 0x25, 0x5d, 0x0c, 0xb6, 0x76, 0x0a, 0xf8, 
        0x3f, 0x77, 0x79, 0xb7, 0xde, 0xcd, 0x2e, 0x95, 
        0x1f, 0x44, 0x42, 0x33, 0x01, 0x81, 0x73, 0x6c
    ]);
    //
    //let newtx = utility::tx::new_reward_transaction(50000000000000,0,Hash::compute_hash(b"Initi".to_vec()));
    let newtx = tx::tx::new_reward_transaction(10_000_000_000_000,0,newaddress);
    let new_root_hash=newtx.compute_hash();
    let newtx_serialization=newtx.serialize_transaction();
    println!("newtx_serialization {:?}",newtx_serialization);
    //let newtimestamp=time::utility::timestamp_now();
    let newnewtx=tx::tx::unserialize_transaction(newtx_serialization).unwrap();
    println!("newnewtx {:?}",newnewtx);
}

*/