use utility::hashing;
use utility::hashing::Hash;
use tx;
use tx::tx::Transaction;
use crate::wallet_inner::WalletInner;
use std::error::Error;

use utility::ecdsa::sign_messagehash;

use std::sync::{Arc, Mutex};
//
pub fn new_send_to_address_transaction(mut shared_wallet_inner: Arc<Mutex<WalletInner>>,value: u64, fee: u64, pubkeyhash: Hash) ->  Result<Transaction, Box<dyn Error>>  {
    let mut tmp_wallet_inner = shared_wallet_inner.lock().unwrap();
    let (tmp_vasset,tmp_vasset_indexes,total_value)= tmp_wallet_inner.get_available_unspent_assets(value+fee);
    if tmp_vasset.len()==0 {
        //NOT ENOUGH VALUE 
        let error: Box<dyn Error> = "not enough globals in the wallet".into();
        return Err(error);
    }
    tmp_wallet_inner.get_random_keypair()?;
    let new_wallet_address=tmp_wallet_inner.get_last_address()?;
    let mut tx = Transaction {
        version: 1,
        vin: Vec::new(),
        vout: Vec::new(),//vec![new_ecdsa_txout(value + fee, pubkeyhash)],
    };
    //tx.vin.push(tx::tx::new_ecdsa_txin(inhash: Hash, index: u32, pubkeycompressedbytes: Vec<u8>))
    for i in 0..tmp_vasset.len() {
        tx.vin.push(tx::txin::new_ecdsatxin(tmp_vasset[i].hash.clone(),tmp_vasset[i].index, tmp_wallet_inner.get_public_key_compressed_bytes(tmp_vasset[i].key_index)))
    }
    //
    tx.vout.push(tx::txout::new_ecdsatxout(value, pubkeyhash));
    tx.vout.push(tx::txout::new_ecdsatxout(total_value-value-fee, new_wallet_address));
    //
    let tmp_tx_hash=tx.compute_hash();
    //pub fn sign_messagehash(kp: &KeyPair, message_hash: Hash) -> Result<Signature, Box<dyn Error>> {
    for i in 0..tmp_vasset.len() {
        let tmp_kp=tmp_wallet_inner.get_keypair(tmp_vasset[i].key_index)?;
        tx.vin[i].put_signature(sign_messagehash(&tmp_kp,tmp_tx_hash.clone())?);
    }
    //tmp_wallet_inner.category+=1;
    for i in 0..tmp_vasset_indexes.len(){
        tmp_wallet_inner.set_asset_as_unavailable(tmp_vasset_indexes[i]);
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