use thiserror::Error;
use utility::hash::hash::Hash;
use utility::buffer::buffer_writer::BufferWriter;
use utility::buffer::buffer_reader::BufferReader;
use utility::buffer::buffer_reader::BufferReaderError;
use utility::ecdsa::ecdsa::verify_signature;

pub const MAINTX_IN_IDENTIFIER_MAINBLOCK_REWARD: u32 = 0;
pub const MAINTX_IN_IDENTIFIER_ECDSA: u32 = 1;

#[derive(Debug, Clone)]
pub struct MaintxInMainBlockReward {
    pub mainblock_height: u32,
}

#[derive(Debug, Clone)]
pub struct MaintxInEcdsa {
    pub hash: Hash,
    pub index: u32,
    pub publickey: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum MaintxIn {
    MaintxInMainblockRewardVariant(MaintxInMainBlockReward),
    MaintxInEcdsaVariant(MaintxInEcdsa),
}

#[derive(Error, Debug)]
pub enum MaintxInError {
    #[error("Expected MaintxInEcdsa, but got MaintxInMainBlockReward")]
    NotEcdsaVariant,

    #[error("Unknown MAINTXIN Identifier: {0}")]
    UnknownIdentifier(u32),

    //#[error("I/O Error: {0}")]
    //Io(#[from] std::io::Error),
    #[error("Buffer reader error: {0}")]
    BufferReaderError(#[from] BufferReaderError),

}

impl MaintxInMainBlockReward {
    pub fn serialize(&self, writer: &mut BufferWriter) {
        writer.put_var_u32(MAINTX_IN_IDENTIFIER_MAINBLOCK_REWARD);
        writer.put_u32(self.mainblock_height);
    }
}

impl MaintxInEcdsa {
    pub fn serialize(&self, writer: &mut BufferWriter, signing: bool) {
        writer.put_var_u32(MAINTX_IN_IDENTIFIER_ECDSA);
        writer.put_hash(self.hash.clone());
        writer.put_var_u32(self.index);
        writer.put_var_bytes(&self.publickey.clone());

        if signing {
            writer.put_var_bytes(&self.signature.clone());
        }

        writer.put_var_u64(0); // No extradata
    }

    pub fn check_signature(&self, hash: Hash) -> bool {
        match verify_signature(&self.publickey, hash, &self.signature) {
            Ok(value) => {
               return false;
            }
            Err(_) => {
                return false;
            }
        }
    }

    pub fn set_signature(&mut self, signature: Vec<u8>) {
        self.signature = signature;
    }
}

impl MaintxIn {
    pub fn serialize(&self, writer: &mut BufferWriter, signing: bool) {
        match self {
            MaintxIn::MaintxInMainblockRewardVariant(txinmainblockreward) => txinmainblockreward.serialize(writer),
            MaintxIn::MaintxInEcdsaVariant(txinecdsa) => txinecdsa.serialize(writer, signing),
        }
    }

    pub fn is_ecdsa(&self) -> bool {
        matches!(self, MaintxIn::MaintxInEcdsaVariant(_))
    }

    pub fn verify_ecdsa_signature(&self, hash: Hash) -> bool {
        if let MaintxIn::MaintxInEcdsaVariant(txin) = self {
            txin.check_signature(hash)
        } else {
            false
        }
    }
    /*
    pub fn as_ecdsa(&self) -> Result<&MaintxInEcdsa, MaintxInError> {
        if let MaintxIn::MaintxInEcdsaVariant(txin) = self {
            Ok(txin)
        } else {
            Err(MaintxInError::NotEcdsaVariant)
        }
    }

    pub fn as_ecdsa_mut(&mut self) -> Result<&mut MaintxInEcdsa, MaintxInError> {
        if let MaintxIn::MaintxInEcdsaVariant(txin) = self {
            Ok(txin)
        } else {
            Err(MaintxInError::NotEcdsaVariant)
        }
    }
    */
    //////////////////////////////////////////////
    pub fn get_ecdsatxin(&self) -> Result<MaintxInEcdsa, MaintxInError>  {
        match self {
            MaintxIn::MaintxInMainblockRewardVariant(_)=> Err(MaintxInError::NotEcdsaVariant),//Err(Box::new(std::io::Error::new(ErrorKind::Other, "get_ecdsatxin error - txin is MainBlockRewardTxInVariant"))),
            MaintxIn::MaintxInEcdsaVariant(txin) => Ok(txin.clone()),
        }
    }
    pub fn set_signature(&mut self,tmp_signature:Vec<u8>) -> Result<(), MaintxInError>  {
        match self {
            MaintxIn::MaintxInMainblockRewardVariant(_)=> Err(MaintxInError::NotEcdsaVariant),//Err(Box::new(std::io::Error::new(ErrorKind::Other, "put_signature error - txin is MainBlockRewardTxInVariant"))),
            MaintxIn::MaintxInEcdsaVariant(ref mut txin) => Ok(txin.set_signature(tmp_signature)),
        }
    }
    pub fn get_hash(&self) -> Result<Hash, MaintxInError>  {
        match self {
            MaintxIn::MaintxInMainblockRewardVariant(_)=> Err(MaintxInError::NotEcdsaVariant),//Err(Box::new(std::io::Error::new(ErrorKind::Other, "get_hash error - txin is MainBlockRewardTxInVariant"))),
            MaintxIn::MaintxInEcdsaVariant(txin) => Ok(txin.hash.clone()),
        }
    }
    pub fn get_index(&self) -> Result<u32, MaintxInError>  {
        match self {
            MaintxIn::MaintxInMainblockRewardVariant(_)=> Err(MaintxInError::NotEcdsaVariant),//Err(Box::new(std::io::Error::new(ErrorKind::Other, "get_index error - txin is MainBlockRewardTxInVariant"))),
            MaintxIn::MaintxInEcdsaVariant(txin) => Ok(txin.index),
        }
    }
    pub fn get_publickey(&self) -> Result<Vec<u8>, MaintxInError>  {
        match self {
            MaintxIn::MaintxInMainblockRewardVariant(_)=> Err(MaintxInError::NotEcdsaVariant),//Err(Box::new(std::io::Error::new(ErrorKind::Other, "get_publickey error - txin is MainBlockRewardTxInVariant"))),
            MaintxIn::MaintxInEcdsaVariant(txin) => Ok(txin.publickey.clone()),
        }
    }
    pub fn get_signature(&self) -> Result<Vec<u8>, MaintxInError>  {
        match self {
            MaintxIn::MaintxInMainblockRewardVariant(_)=> Err(MaintxInError::NotEcdsaVariant),//Err(Box::new(std::io::Error::new(ErrorKind::Other, "get_signature error - txin is MainBlockRewardTxInVariant"))),
            MaintxIn::MaintxInEcdsaVariant(txin) => Ok(txin.signature.clone()),
        }
    }
    //////////////////////////////////////////////
}

pub fn new_mainblockrewardtxin(height: u32) -> MaintxIn {
    MaintxIn::MaintxInMainblockRewardVariant(MaintxInMainBlockReward {
        mainblock_height: height,
    })
}

pub fn new_ecdsa_maintx_in(hash: Hash, index: u32, publickey: Vec<u8>) -> MaintxIn {
    MaintxIn::MaintxInEcdsaVariant(MaintxInEcdsa {
        hash,
        index,
        publickey,
        signature: Vec::new(),
    })
}

pub fn unserialize_maintx_in(reader: &mut BufferReader) -> Result<MaintxIn, MaintxInError> {
    let txin_id = reader.get_var_u32()?;
    //println!("txin_id {}",txin_id);

    if  txin_id == MAINTX_IN_IDENTIFIER_ECDSA {
            println!("MAINTXIN_IDENTIFIER_ECDSA");
            let hash = reader.get_hash()?;
            println!("hash {:?}",hash);
            let index = reader.get_var_u32()?;
            println!("index {}",index);
            let publickey = reader.get_var_bytes()?;
            println!("publickey {:?}",publickey);
            let signature = reader.get_var_bytes()?;
            println!("signature {:?}",signature);
            let _extra_bytes=reader.get_var_u64()?;

            Ok(MaintxIn::MaintxInEcdsaVariant(MaintxInEcdsa {
                hash,
                index,
                publickey,
                signature,
            }))
        } else if txin_id == MAINTX_IN_IDENTIFIER_MAINBLOCK_REWARD {
            let mainblock_height = reader.get_u32()?;
            Ok(MaintxIn::MaintxInMainblockRewardVariant(MaintxInMainBlockReward {
                mainblock_height,
            }))
        } else {
            Err(MaintxInError::UnknownIdentifier(txin_id))
        }


}