//use utility::hashing;
//use utility::ecdsa;
use std::error::Error;
use utility::hashing;
use utility::ecdsa;
use utility::hashing::Hash;

use utility::bufferwriter::BufferWriter;
use utility::bufferreader::BufferReader;
use utility::utility::generate_random_number;
use tx::tx::Transaction;
use utility::bytesfile;
use std::fs;
// A wallet starts with a type then a version then an array of hashes that are used to generate key pairs 
//use crate::wallet::WALLET_CATEGORY_SEQUENTIAL;
//
use crate::asset;
use crate::asset::Asset;
//#[derive(Clone)] 
#[derive(Debug,Clone)]
pub struct WalletInner {
    //pub category: usize,
    version: i32,
    vkp: Vec<ecdsa::KeyPair>,
    last_known_height: usize,
    vasset: Vec<Asset>,
}
//

impl WalletInner {
    pub fn new() -> Self {
        WalletInner {
            version:1,
            vkp:Vec::new(),
            last_known_height:0,
            vasset:Vec::new(),
        }
    }
    pub fn get_version(&self) -> i32 {
        self.version
    }
    pub fn get_last_known_height(&self) -> usize {
        self.last_known_height
    }
    pub fn put_last_known_height(& mut self,height:usize) {
        self.last_known_height=height;
    }
    pub fn get_secret_key_bytes(&self, index:  usize) -> Vec<u8> {
        self.vkp[index].get_secret_key_bytes().clone()
    }
    pub fn get_public_key_compressed_bytes(&self, index:  usize) -> Vec<u8> {
        self.vkp[index].get_public_key_compressed_bytes().clone()
    }
    pub fn get_address(&self, index:  usize) -> Hash {
        self.vkp[index].get_address()
    }
    pub fn get_addresses(&self)-> Vec<Hash> {
        let mut tmp_addresses:Vec<Hash>=Vec::new();
        for i in 0..self.vkp.len() {
            //println!("vkp i {} secret_key: {:?} secret_bytes len() {} secret_bytes len() {:?}  public_key: {:?}",i,vkp[i].secret_key(),vkp[i].secret_key().secret_bytes().to_vec().len(),vkp[i].secret_key().secret_bytes().to_vec(),vkp[i].public_key());        
            //let tmp_addr=hashing::compute_hash(&(self.vkp[i].get_public_key_compressed_bytes()));
            tmp_addresses.push(self.vkp[i].get_address());
        }
        return tmp_addresses
    }
    pub fn get_keypairs_count(&self) -> usize {
        self.vkp.len()
    }
    pub fn get_keypair(&self,index: usize) ->  Result<ecdsa::KeyPair, Box<dyn Error>>{
        Ok(self.vkp[index].clone())
    }
    pub fn get_last_address(&self) -> Result<Hash, Box<dyn Error>>{
        Ok(self.get_address(self.get_keypairs_count()-1))
    }
    pub fn get_random_keypair(&self) -> Result<ecdsa::KeyPair, Box<dyn Error>>{
        match generate_random_number(0,self.vkp.len()-1) {
            Ok(index) => {
                Ok(self.vkp[index].clone())
            },
            Err(err) => {
                println!("Error get_random_keypair: {}", err);
                return Err(err);//Err(Box::new(e))
            }
        }
        
    }
    pub fn generate_keypair(&mut self) -> Result<(), Box<dyn Error>>{
        let keypairs_count=self.vkp.len();
        if keypairs_count==0 {
            return Err("wallet generate_keypair error: key pair vector empty".into()) 
        }
        let last_secret_key=self.vkp[keypairs_count-1].get_secret_key_bytes();
        let last_secret_key_hash=hashing::compute_hash(&last_secret_key);
        match ecdsa::generate_keypair(&(last_secret_key_hash.to_vec())) {
            Ok(kp) => {
                //println!("get_secret_key_bytes: {:?}", kp.get_secret_key_bytes());
                //println!("get_public_key_compressed_bytes: {:?}", kp.get_public_key_compressed_bytes());
                self.vkp.push(kp);
                Ok(())
            },
            Err(err) => {
                println!("Error generating key pair: {}", err);
                return Err(err);//Err(Box::new(e))
            }
        }
    }
    pub fn set_asset_as_unavailable(&mut self,index:usize){
        self.vasset[index].available=false;
    }
    pub fn update_assets(&mut self,tmptx:Transaction) -> Result<(), Box<dyn Error>>{
        //let mut tmp_addresses:Vec<Hash>=Vec::new();
        //for i in 0..self.vkp.len() {
            //println!("vkp i {} secret_key: {:?} secret_bytes len() {} secret_bytes len() {:?}  public_key: {:?}",i,vkp[i].secret_key(),vkp[i].secret_key().secret_bytes().to_vec().len(),vkp[i].secret_key().secret_bytes().to_vec(),vkp[i].public_key());        
            //let tmp_addr=hashing::compute_hash(&(self.vkp[i].get_public_key_compressed_bytes()));
            //tmp_addresses.push(tmp_addr);
            let tmp_addresses=self.get_addresses();

            for i in 0..tmptx.vout.len() {
                for j in 0..tmp_addresses.len(){
                    //println!("tmp_addresses.len() {} self.vkp.len() {}",tmp_addresses.len(),self.vkp.len());
                    //TODO in case !tmptx.vout[l].is_ecdsatxout continue
                    if tmptx.vout[i].is_ecdsatxout_address(&tmp_addresses[j]) {
                        let h=tmptx.compute_hash();
                        self.add_unspent_asset(h,i as u32,tmptx.vout[i].get_value()?,j.try_into().unwrap());
                        println!("is an asset i {} j {} {:?} {:?}",i,j,tmptx.vout[i].get_address().unwrap(),tmp_addresses[j]);
                    } else {
                        println!("not an asset i {} j {} {:?} {:?}",i,j,tmptx.vout[i].get_address().unwrap(),tmp_addresses[j]);
                    }
                    /*
                    match tx::bytecode::is_ecdsa_txout(tmptx.vout[l].bytecode.clone(),tmp_addresses[m].clone()) {
                        Ok(tmpresult_is_ecdsa_txin)=> {
                            if tmpresult_is_ecdsa_txin {
                                let h=tmptx.compute_hash();
                                self.add_unspent_asset(h,l as u32,tmptx.vout[l].value,m.try_into().unwrap());
                            }
                        },
                        Err(err)=> {
                            println!("error {}",err);
                        }
                    }
                    */       
                }
            }
            //
            for l in 0..tmptx.vin.len() {
                for m in 0..self.vkp.len(){
                    if tmptx.vin[l].is_ecdsatxin() {
                        self.update_asset_to_spent(tmptx.vin[l].get_hash()?,tmptx.vin[l].get_index()?);
                    }
                    /*
                    match tx::bytecode::is_ecdsa_txin(tmptx.vin[l].bytecode.clone(),self.vkp[m].get_public_key_compressed_bytes()) {
                        Ok(tmpresult_is_ecdsa_txin)=> {
                            if tmpresult_is_ecdsa_txin {
                                self.update_asset_to_spent(tmptx.vin[l].hash.clone(),tmptx.vin[l].index);
                            }     
                        },
                        Err(err)=> {
                            println!("error {}",err);
                        }
                    }
                    */
                }
            }

        //}
        //
        Ok(())
    }
    pub fn add_unspent_asset(&mut self, h :Hash, tmpindex: u32,value:u64,key_index: usize) {
        
        /*
        let mut new_asset= Asset {
            hash: h,
            index: tmpindex,
            status: Vec::new(),
        };
        let mut tmpbw = BufferWriter::new();
        tmpbw.put_u32(ASSET_STATUS_IDENTIFIER_UNSPENT);
        //tmpbw.put_var_u64(pubkeycompressedbytes.len() as u64);
        //tmpbw.put_bytes(pubkeycompressedbytes);
        //tmpbw.put_var_u64(0); // TODO extend to support extradata
        new_asset.status.extend_from_slice(&tmpbw.get_bytes());
        */
        for i in 0..self.vasset.len() {
            if self.vasset[i].hash.is_eq(&h.clone()) && self.vasset[i].index==tmpindex {
                return
            }
        }
        let new_asset=asset::new_unspent_asset(h,tmpindex,value,key_index);
        self.vasset.push(new_asset);
        println!("vasset.len() {}",self.vasset.len());
    }
    pub fn update_asset_to_spent(&mut self, h :Hash, tmpindex: u32) {
        for i in 0..self.vasset.len() {
            //if self.vasset[i].is_eq(&h,tmpindex) {
            
            if self.vasset[i].hash.is_eq(&h) && self.vasset[i].index==tmpindex {
                self.vasset[i].update_asset_to_spent();

                /*
            	let mut tmpbw = BufferWriter::new();
                tmpbw.put_u32(ASSET_STATUS_IDENTIFIER_SPENT);
                //tmpbw.put_var_u64(pubkeycompressedbytes.len() as u64);
                //tmpbw.put_bytes(pubkeycompressedbytes);
                //tmpbw.put_var_u64(0); // TODO extend to support extradata
                self.vasset[i].status.clear();
                self.vasset[i].status.extend_from_slice(&tmpbw.get_bytes());
            	*/
            }
            //
        }
        
    }
    //
    pub fn get_balance(&self) -> u64 {
        let mut total_balance=0;
        println!("get_balance vasset.len() {}",self.vasset.len());
        for i in 0..self.vasset.len() {
            if self.vasset[i].is_unspent_asset(){
                total_balance+=self.vasset[i].value;
            }
            
        }
        total_balance
    }
    //

    pub async fn save_addressesfile(&self,_path:String) {
        let mut bw = BufferWriter::new();

        bw.put_var_u64(self.vkp.len() as u64);
        println!("Saving ... Number of addresses {}",self.vkp.len());
        //
        //let tmp_addr=self.compute_addresses();
        //
        let content = bw.get_bytes();//bw.get_content();
        //println!("Saving ... Addresses Rawbytes len {}",content.len());
        let mut i=0;
        //
        for i in 0..self.vkp.len() {
            //println!("vkp i {} secret_key: {:?} secret_bytes len() {} secret_bytes len() {:?}  public_key: {:?}",i,vkp[i].secret_key(),vkp[i].secret_key().secret_bytes().to_vec().len(),vkp[i].secret_key().secret_bytes().to_vec(),vkp[i].public_key());        
            let tmp_addr=hashing::compute_hash(&(self.vkp[i].get_public_key_compressed_bytes()));
            bw.put_hash(tmp_addr);
            println!("Saving ... Addresses {}",i);
        }
        loop {
    
            let file_path = format!("MiningFiles/AddressesFile{}", i);
            if fs::metadata(file_path.clone()).is_ok() {
                println!("File exists: {}", file_path);
                i+=1;
            } else {
                println!("File does not exist: {}", file_path);
                if let Err(err) = bytesfile::save_bytes_to_file(&content, &file_path).await {
                    eprintln!("Error: {}", err);
                    //return Err(err);
                }
                break;
            }
    
        }
    }
    pub fn compute_addresses(&self)-> Vec<Hash> {
        let mut tmp_addresses:Vec<Hash>=Vec::new();
        for i in 0..self.vkp.len() {
            //println!("vkp i {} secret_key: {:?} secret_bytes len() {} secret_bytes len() {:?}  public_key: {:?}",i,vkp[i].secret_key(),vkp[i].secret_key().secret_bytes().to_vec().len(),vkp[i].secret_key().secret_bytes().to_vec(),vkp[i].public_key());        
            let tmp_addr=hashing::compute_hash(&(self.vkp[i].get_public_key_compressed_bytes()));
            tmp_addresses.push(tmp_addr);
            
        }
        return tmp_addresses
    }
    pub fn get_available_unspent_assets(&self,min_total_value:u64)-> (Vec<Asset>,Vec<usize>,u64) {
        let mut total_value=0;
        let mut tmp_vasset:Vec<Asset>=Vec::new();
        let mut tmp_vasset_indexes:Vec<usize>=Vec::new();
        if min_total_value==0 {
            return (tmp_vasset,tmp_vasset_indexes,0)
        }
        for i in 0..self.vasset.len() {
            if self.vasset[i].is_unspent_asset() && self.vasset[i].available{
                total_value+=self.vasset[i].value;
                tmp_vasset.push(self.vasset[i].clone());
                tmp_vasset_indexes.push(i)
            }
            if total_value >= min_total_value {
                println!("get_available_unspent_assets tmp_vasset {:?} tmp_vasset_indexes {:?} total_value {}",tmp_vasset,tmp_vasset_indexes,total_value);
                return (tmp_vasset,tmp_vasset_indexes,total_value)
            }
        }
        (Vec::new(),Vec::new(),0)
    }

}
//TODONOW recode as two separate functions a "new()" and a "WalletInner init()"
pub fn new_wallet_inner (wallet_seed: String) -> Result<WalletInner, Box<dyn Error>> {
    let mut new_wallet_inner= WalletInner {
            //category:0,
            version:1,
            vkp:Vec::new(),
            last_known_height:0,
            vasset:Vec::new(),
        };

    let mut tmphash = hashing::compute_hash(wallet_seed.as_bytes());
    let stretching_factor:u64=1000;//1000_000_000;//TOFINALIZE
    for i in 0..stretching_factor {
        if ((i+1) * 10) % stretching_factor == 0 {
            println!("Progress {} %", ((i+1) * 10)*10/stretching_factor );
            tmphash=hashing::compute_hash(tmphash.as_bytes());
        }
    }
    println!("Final hash {:?}", tmphash);
    ////////////
    match ecdsa::generate_keypair(tmphash.as_bytes().clone()) {
        Ok(kp) => {
            //println!("get_secret_key_bytes: {:?}", kp.get_secret_key_bytes());
            //println!("get_public_key_compressed_bytes: {:?}", kp.get_public_key_compressed_bytes());
            new_wallet_inner.vkp.push(kp);
        },
        Err(err) => {
            println!("Error generating key pair: {}", err);
            return Err(err);//Err(Box::new(e))
        }
    }
    //let mut vkp: Vec<ecdsa::KeyPair> = Vec::new();
    for i in 0..3 {//TOFINALIZE MIN_KEYPAIRS_COUNT
        new_wallet_inner.generate_keypair()?;
        //tmphash=hashing::compute_hash(tmphash.as_bytes());
    }
    for i in 0..new_wallet_inner.get_keypairs_count() {
        println!("**** vkp i {} secret_key: {:?} public_key_compressed: {:?}",i,new_wallet_inner.get_secret_key_bytes(i),new_wallet_inner.get_public_key_compressed_bytes(i));
    }
    Ok(new_wallet_inner)
}