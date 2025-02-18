use sled::Db;
//use std::sync::Arc;
//use std::fs;
pub use sled::Error;
pub struct StateDb {
    db: Db,//Arc<Db>,
    path: String,
}

impl StateDb {
    pub fn new(path: &str) -> Result<Self, Error> {
        let db = sled::open(path)?;//Arc::new(sled::open(path)?);
        Ok(Self { db,path: path.to_string() })
    }

    pub fn insert(&self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        self.db.insert(key, value)?;
        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        let result = self.db.get(key)?;
        Ok(result.map(|data| data.to_vec()))
    }

    pub fn remove(&self, key: &[u8]) -> Result<(), Error> {
        self.db.remove(key)?;
        Ok(())
    }

    pub fn flush(&self) -> Result<(), Error> {
        self.db.flush()?;
        // Close the database before deleting it to ensure everything is cleaned up.
        //drop(self.db);
        //fs::remove_dir_all(self.db.context.get_path())?;
        Ok(())
    }
}