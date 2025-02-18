use utility::hash::hash::Hash;
use maintx::maintx::maintx::Maintx;

// Define a struct for the transaction item
#[derive(Clone)]
struct MaintxsItem {
    maintx: Maintx,
    hash: Hash,
    mainblock_height: u32,
}

pub struct MaintxsPool {
    // Store items with their priorities in a Vec
    maintxsitemvec: Vec<(MaintxsItem, f64)>, // (maintxsitem, priority)
}

impl MaintxsPool {
    pub fn new() -> Self {
        MaintxsPool {
            maintxsitemvec: Vec::new(),
        }
    }

    pub fn insert_new_maintx(&mut self, newtx: Maintx, priority: f64) {
        let item = MaintxsItem {
            maintx: newtx.clone(),
            hash: newtx.compute_hash(),
            mainblock_height: 0,
        };
        
        // Binary search for insertion position to maintain sorted order
        let pos = match self.maintxsitemvec.binary_search_by(|(_, p)| 
            p.partial_cmp(&priority).unwrap()
        ) {
            Ok(pos) => pos,
            Err(pos) => pos,
        };
        
        self.maintxsitemvec.insert(pos, (item, priority));
    }
    
    pub fn get_prioritized_maintxs(&self) -> Vec<Maintx> {
        let mut total_size = 0;
        let mut result = Vec::new();
        
        for (item, _) in self.maintxsitemvec.iter() {
            if item.mainblock_height != 0 {
                continue;
            }
            
            let tx_size = item.maintx.get_serialization_size();
            if total_size + tx_size > 100_000 {
                break;
            }
            
            total_size += tx_size;
            result.push(item.maintx.clone());
        }
        
        result
    }

    pub fn set_tx_to_mainblock_height(&mut self, tmp_hash: Hash, tmp_height: u32) {
        if let Some(pos) = self.maintxsitemvec.iter_mut()
            .position(|(item, _)| item.hash == tmp_hash) 
        {
            self.maintxsitemvec[pos].0.mainblock_height = tmp_height;
        }
    }

    pub fn reset_tx_with_mainblock_height(&mut self, tmp_mainblock_height: u32) {
        for (item, _) in self.maintxsitemvec.iter_mut() {
            if item.mainblock_height == tmp_mainblock_height {
                item.mainblock_height = 0;
            }
        }
    }

    pub fn remove(&mut self, tmp_hash: Hash) {
        if let Some(pos) = self.maintxsitemvec.iter()
            .position(|(item, _)| item.hash == tmp_hash) 
        {
            self.maintxsitemvec.remove(pos);
        }
    }

    pub fn contains(&self, tmp_hash: Hash) -> bool {
        self.maintxsitemvec.iter()
            .any(|(item, _)| item.hash == tmp_hash)
    }

    pub fn is_empty(&self) -> bool {
        self.maintxsitemvec.is_empty()
    }

    // Helper method to get the current size of the pool
    pub fn len(&self) -> usize {
        self.maintxsitemvec.len()
    }

    // Optional: Method to get capacity hints
    pub fn with_capacity(capacity: usize) -> Self {
        MaintxsPool {
            maintxsitemvec: Vec::with_capacity(capacity),
        }
    }

    // Optional: Method to reserve additional capacity
    pub fn reserve(&mut self, additional: usize) {
        self.maintxsitemvec.reserve(additional);
    }

    // Optional: Debug method to print the pool
    pub fn print(&self) {
        println!("Transaction Pool Contents:");
        for (idx, (item, priority)) in self.maintxsitemvec.iter().enumerate() {
            println!("{}. Hash: {:?}, Priority: {}, Block Height: {}", 
                idx + 1, 
                item.hash, 
                priority, 
                item.mainblock_height
            );
        }
    }

    // Optional: Method to clear all transactions
    pub fn clear(&mut self) {
        self.maintxsitemvec.clear();
    }

    // Optional: Method to get all transactions below a certain block height
    pub fn get_txs_below_height(&self, height: u32) -> Vec<Maintx> {
        self.maintxsitemvec.iter()
            .filter(|(item, _)| item.mainblock_height < height)
            .map(|(item, _)| item.maintx.clone())
            .collect()
    }
}

// Implement iteration over the pool if needed
impl<'a> IntoIterator for &'a MaintxsPool {
    type Item = &'a Maintx;
    type IntoIter = MaintxsPoolIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MaintxsPoolIterator {
            pool: self,
            index: 0,
        }
    }
}

// Custom iterator implementation
pub struct MaintxsPoolIterator<'a> {
    pool: &'a MaintxsPool,
    index: usize,
}

impl<'a> Iterator for MaintxsPoolIterator<'a> {
    type Item = &'a Maintx;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.pool.maintxsitemvec.len() {
            let item = &self.pool.maintxsitemvec[self.index].0.maintx;
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}