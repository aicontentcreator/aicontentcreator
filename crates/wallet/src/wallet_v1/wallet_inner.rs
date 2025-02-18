use std::error::Error;
use thiserror::Error;
use utility::ecdsa::key_derivation_v1::HARDENED_OFFSET;
use utility::ecdsa::key_derivation_v1::ExtendedSecretKey;
use utility::ecdsa::ecdsa::derive_child_key_set;

use utility::ecdsa::key_derivation_v1::derive_master_extended_secret_key;
use utility::ecdsa::key_derivation_v1::KeyDerivationError;
use utility::ecdsa::ecdsa::EcdsaKeySet;
use utility::ecdsa::ecdsa::EcdsaKeySetError;
use utility::ecdsa::ecdsa::{sign_messagehash, verify_signature};

use utility::hash::hash::Hash;
use utility::system::random::generate_random_number;
use utility::buffer::buffer_writer::BufferWriter;
use utility::buffer::buffer_reader::BufferReader;
//use utility::bytesfile;
use std::fs;
use maintx::maintx::maintx::Maintx;
use maintx::maintx_out::maintx_out::MaintxOutError;

use maintx::maintx_in::maintx_in::MaintxInError;

use crate::wallet_v1::resource::Resource;
use crate::wallet_v1::resource::new_unspent_resource;
#[derive(Debug, Error)]
pub enum WalletInnerError {
    #[error("Key pair vector is empty")]
    EmptyEcdsaKeySet,

    #[error("Invalid index: {0}")]
    InvalidIndex(usize),
    #[error("KeyDerivationError error: {0}")]
    KeyDerivationError(#[from] KeyDerivationError),

    #[error("generate_key_set failed after too many attempt")]
    GenerateKeySetFailedAfterTooManyAttempt,

    //#[error("ECDSA error: {0}")]
    //EcdsaKeySetError(String),
    #[error("EcdsaKeySetError error: {0}")]
    EcdsaKeySetError(#[from] EcdsaKeySetError),

    #[error("Failed to generate random number: {0}")]
    RandomNumberError(String),

    #[error("MaintxOutError error: {0}")]
    MaintxOutError(#[from] MaintxOutError),
    #[error("MaintxInError error: {0}")]
    MaintxInError(#[from] MaintxInError),
}

#[derive(Debug, Clone)]
pub struct WalletInner {
    version: u64,
    hardened:bool,
    master_extended_secret_key:ExtendedSecretKey,
    vks: Vec<EcdsaKeySet>,
    last_known_height: u32,
    vresource: Vec<Resource>,
}

impl WalletInner {
    /*
    pub fn new() -> Self {
        WalletInner {
            version: 1,
            vks: Vec::new(),
            last_known_height: 0,
            vresource: Vec::new(),
        }
    }
    */

    pub fn get_version(&self) -> u64 {
        self.version
    }

    pub fn get_last_known_height(&self) -> u32 {
        self.last_known_height
    }

    pub fn set_last_known_height(&mut self, height: u32) {
        self.last_known_height = height;
    }

    pub fn get_secret_key_bytes(&self, index: usize) -> Result<Vec<u8>, WalletInnerError> {
        self.vks
            .get(index)
            .map(|ks| ks.get_secret_key_bytes().clone())
            .ok_or(WalletInnerError::InvalidIndex(index))
    }

    pub fn get_public_key_compressed_bytes(&self, index: usize) -> Result<Vec<u8>, WalletInnerError> {
        self.vks
            .get(index)
            .map(|ks| ks.get_public_key_compressed_bytes().clone())
            .ok_or(WalletInnerError::InvalidIndex(index))
    }

    pub fn get_address(&self, index: usize) -> Result<Hash, WalletInnerError> {
        self.vks
            .get(index)
            .map(|ks| ks.get_address())
            .ok_or(WalletInnerError::InvalidIndex(index))
    }

    pub fn get_addresses(&self) -> Vec<Hash> {
        self.vks.iter().map(|ks| ks.get_address()).collect()
    }
    pub fn get_derivation_index(&self, index: usize) -> Result<u32, WalletInnerError> {
        self.vks
            .get(index)
            .map(|ks| ks.get_derivation_index())
            .ok_or(WalletInnerError::InvalidIndex(index))
    }

    pub fn get_keysets_count(&self) -> usize {
        self.vks.len()
    }

    pub fn get_keypair(&self, index: usize) -> Result<EcdsaKeySet, WalletInnerError> {
        self.vks
            .get(index)
            .cloned()
            .ok_or(WalletInnerError::InvalidIndex(index))
    }

    pub fn get_random_keyset(&self) -> Result<EcdsaKeySet, WalletInnerError> {
        let count = self.vks.len();
        if count == 0 {
            return Err(WalletInnerError::EmptyEcdsaKeySet);
        }
        let index = generate_random_number(0, count - 1).map_err(|e| WalletInnerError::RandomNumberError(e.to_string()))?;
        Ok(self.vks[index].clone())
    }

    /*
    pub fn generate_keypair(&mut self) -> Result<(), WalletInnerError> {
        if self.vks.is_empty() {
            return Err(WalletInnerError::EmptyEcdsaKeySet);
        }
        let last_secret_key = self.vks.last().unwrap().get_secret_key_bytes();
        let last_secret_key_hash = Hash::compute_hash(&last_secret_key);
        let new_ks = generate_keypair(&last_secret_key_hash.to_vec())
            .map_err(|e| WalletInnerError::EcdsaKeySetError(e.to_string()))?;
        self.vks.push(new_ks);
        Ok(())
    }
    */
    pub fn generate_key_set(&mut self) -> Result<(), WalletInnerError> {
        if self.vks.is_empty() {
            return Err(WalletInnerError::EmptyEcdsaKeySet);
        }

        let last_derivation_index=self.get_derivation_index(self.vks.len()-1)?;//
        
        for count in 1..=5000 { 

            let new_ks_result = derive_child_key_set(&self.master_extended_secret_key, last_derivation_index+count, true);
            match new_ks_result {
                Ok(new_ks) => {
                    self.vks.push(new_ks);
                    return Ok(())
                },
                Err(error) => println!("Error generate_key_set: {}", error),
            }
            
        }

        return Err(WalletInnerError::GenerateKeySetFailedAfterTooManyAttempt);
    }
    pub fn set_resource_as_unavailable(&mut self,index:usize){
        self.vresource[index].available=false;
    }
    pub fn update_resources(&mut self, tmpmaintx: Maintx) -> Result<(), WalletInnerError> {
        let addresses = self.get_addresses();

        for (i, vout) in tmpmaintx.vout.iter().enumerate() {
            for (j, address) in addresses.iter().enumerate() {
                if vout.matches_address(address) {
                    let hash = tmpmaintx.compute_hash();
                    self.add_unspent_resource(hash, i as u32, vout.get_value()?, j);
                }
            }
        }

        for vin in &tmpmaintx.vin {
            if vin.is_ecdsa() {
                self.update_resource_to_spent(vin.get_hash()?, vin.get_index()?);
            }
        }

        Ok(())
    }

    pub fn add_unspent_resource(&mut self, h: Hash, tmpindex: u32, value: u64, key_index: usize) {
        if self.vresource.iter().any(|r| r.hash==h && r.index == tmpindex) {
            return;
        }
        let new_resource = new_unspent_resource(h, tmpindex, value, key_index);
        self.vresource.push(new_resource);
    }

    pub fn update_resource_to_spent(&mut self, h: Hash, tmpindex: u32) {
        if let Some(resource) = self.vresource.iter_mut().find(|r| r.hash==h && r.index == tmpindex) {
            resource.update_resource_to_spent();
        }
    }

    pub fn get_balance(&self) -> u64 {
        self.vresource
            .iter()
            .filter(|r| r.is_unspent_resource())
            .map(|r| r.value)
            .sum()
    }
    /*
    pub async fn save_addressesfile(&self, path: String) -> Result<(), WalletInnerError> {
        let mut bw = BufferWriter::new();
        bw.put_var_u64(self.vks.len() as u64);

        for ks in &self.vks {
            let tmp_addr = hash::hash::compute_hash(&ks.get_public_key_compressed_bytes());
            bw.put_hash(tmp_addr);
        }

        for i in 0.. {
            let file_path = format!("MiningFiles/AddressesFile{}", i);
            if fs::metadata(&file_path).is_err() {
                bytesfile::save_bytes_to_file(&bw.get_bytes(), &file_path).await.map_err(|e| WalletInnerError::EcdsaError(e.to_string()))?;
                break;
            }
        }
        Ok(())
    }
    */
    pub fn get_available_unspent_resources(&self,min_total_value:u64)-> (Vec<Resource>,Vec<usize>,u64) {
        let mut total_value=0;
        let mut tmp_vresource:Vec<Resource>=Vec::new();
        let mut tmp_vresource_indexes:Vec<usize>=Vec::new();
        if min_total_value==0 {
            return (tmp_vresource,tmp_vresource_indexes,0)
        }
        for i in 0..self.vresource.len() {
            if self.vresource[i].is_unspent_resource() && self.vresource[i].available{
                total_value+=self.vresource[i].value;
                tmp_vresource.push(self.vresource[i].clone());
                tmp_vresource_indexes.push(i)
            }
            if total_value >= min_total_value {
                println!("get_available_unspent_resources tmp_vresource {:?} tmp_vresource_indexes {:?} total_value {}",tmp_vresource,tmp_vresource_indexes,total_value);
                return (tmp_vresource,tmp_vresource_indexes,total_value)
            }
        }
        (Vec::new(),Vec::new(),0)
    }
}
pub fn new_hardened_wallet_inner(wallet_seed: String) -> Result<WalletInner, WalletInnerError> {
    let tmp_master_extended_secret_key = derive_master_extended_secret_key(&wallet_seed)?;
    println!("derive_master_extended_secret_key: {:?}", tmp_master_extended_secret_key);

    let mut new_wallet_inner = WalletInner {
        version: 1,
        hardened:true,
        master_extended_secret_key: tmp_master_extended_secret_key.clone(),
        vks: Vec::new(),
        last_known_height: 0,
        vresource: Vec::new(),
    };



        
    for count in 1..=5000 { 

        let initial_ks_result = derive_child_key_set(&tmp_master_extended_secret_key, HARDENED_OFFSET+count, true);
        match initial_ks_result {
            Ok(initial_ks) => {
                new_wallet_inner.vks.push(initial_ks);
                for _ in 0..3 {
                    new_wallet_inner.generate_key_set()?;
                }
            
                return Ok(new_wallet_inner)
            },
            Err(error) => println!("Error generate_key_set: {}", error),
        }
        
    }

    return Err(WalletInnerError::GenerateKeySetFailedAfterTooManyAttempt);





    //let initial_ks = generate_key_set(&master_extended_secret_key, HARDENED_OFFSET, true)?;
    //new_wallet_inner.vks.push(initial_ks);


}
/*
pub fn new_wallet_inner(wallet_seed: String) -> Result<WalletInner, WalletInnerError> {
    let mut new_wallet_inner = WalletInner::new();

    let mut tmphash = Hash::compute_hash(wallet_seed.as_bytes());
    let stretching_factor: u64 = 1000;
    for i in 0..stretching_factor {
        if ((i + 1) * 10) % stretching_factor == 0 {
            tmphash = Hash::compute_hash(tmphash.as_bytes());
        }
    }

    let initial_ks = generate_keypair(tmphash.as_bytes()).map_err(|e| WalletInnerError::EcdsaKeySetError(e.to_string()))?;
    new_wallet_inner.vks.push(initial_ks);

    for _ in 0..3 {
        new_wallet_inner.generate_keypair()?;
    }

    Ok(new_wallet_inner)
}
*/