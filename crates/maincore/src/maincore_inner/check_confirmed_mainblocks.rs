
use crate::maincore_inner::maincore_inner::MaincoreInner;
use utility::hash::hash::Hash;
pub async fn check_confirmed_mainblocks(mut tmp_mci:MaincoreInner) -> bool {



    let maincoreinner_blocks_count=tmp_mci.get_confirmed_mainblocks_last_height();
    println!("*********maincoreinner blocks count {}",maincoreinner_blocks_count);
    ///////////////////////////////////////////////////
    
    let mut tmp_prev_hash:Hash=Hash::new_empty();
    //let mut last_timestamp:i64=0;
    for i in 0..maincoreinner_blocks_count {
        match tmp_mci.get_mainblock(i).await {
            Ok(mb)=> {
                let mh=mb.get_mainheader();
                println!("height {}",i);
                //for every check_mainheaders //check_target //check_hash
                if !mh.check_hash() {
                    println!("check_confirmed_mainblocks failed - check_hash");
                    return false;
                }
                if !mh.check_target() {
                    println!("check_confirmed_mainblocks failed - check_target");
                    return false;
                }
                //if last_timestamp!=0 {
                //    points.push(( i as f64,(mh.get_timestamp()-last_timestamp) as f64));
                //}
                //println!("block height {} timestamp {}",i,mh.get_timestamp());
                println!("prev_hash {:?}",mh.get_prev_hash());
                println!("hash {:?}",mh.get_hash());
                if (i>0)&&(tmp_prev_hash!=mh.get_prev_hash()) {
                    println!("check_confirmed_mainblocks failed!");
                    return false
                }
                tmp_prev_hash=mh.get_hash();
                //last_timestamp=mh.get_timestamp();
            }
            Err(e)=> {
                println!("get_block error {:?}",e);
                return false;
            }
        }
    }
    println!("check_confirmed_mainblocks success!");
    return true;
}

