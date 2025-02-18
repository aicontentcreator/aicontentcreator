


pub fn mainblock_reward(height: u32) -> u64 {
    const INITIAL_REWARD: u64 = 5000_000_000;
    const HALVING_INTERVAL: u64 = 2100_000;  // Change this to usize
    
    let halvings = (height as u64) / HALVING_INTERVAL;  
    let reward = INITIAL_REWARD >> halvings;   
    
    if reward < 1 { 0 } else { reward }
}