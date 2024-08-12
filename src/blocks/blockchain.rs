use tracing::info;
use crate::{Block, storage::SledDb, storage::Storage};

use std::sync::{Arc, RwLock, atomic::{AtomicUsize, Ordering}};
const CURR_BITS: usize = 8;
pub struct BlockChain<T = SledDb> {
    storage: T,
    blocks: Arc<RwLock<String>>,
    height: AtomicUsize,
}

impl<T: Storage> BlockChain<T> {
    pub fn new(storage: T) -> Self {
        if let Ok(Some(tip)) = storage.get_tip() {
            let height: Option<usize> = storage.get_height().unwrap();
            Self {
                storage,
                blocks: Arc::new(RwLock::new(tip)),
                height: AtomicUsize::new(height.unwrap()),
            }
        }else {
            let first_block = Block::create_born_block(CURR_BITS);
            let hash = first_block.get_hash();
            storage.update_blocks(&hash, &first_block, 0 as usize);

            Self {
                storage,
                blocks: Arc::new(RwLock::new(hash)),
                height: AtomicUsize::new(0),
            }
        }


    }

    pub fn mine_block(&mut self, data: &str) {
        let block = Block::new(data, &self.blocks.read().unwrap(), CURR_BITS);
        let hash  = block.get_hash();
        self.height.fetch_add(1, Ordering::Relaxed);
        self.storage.update_blocks(&hash, &block, self.height.load(Ordering::Relaxed));

        let mut next = self.blocks.write().unwrap();
        *next = hash;
    }


    pub fn show_block(&self) {
        let blocks = self.storage.get_block_iter().unwrap();
        for block in blocks {
            info!("{:#?}", block);
        }
    }


}