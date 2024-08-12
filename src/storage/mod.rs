mod sleddb;

use sled::IVec;
pub use sleddb::SledDb;
pub use sleddb::*;
use crate::{error::BlockChainError};
use crate::{blocks::Block};
use crate::utils::{deserialize, serialize};

pub const TIP_KEY: &str = "hash";
pub const HEIGHT: &str = "height";
pub const TABLE_OF_BLOCK: &str = "blocks";

pub trait Storage: Send + Sync + 'static {
    // 获取最后一个块的hash值
    fn get_tip(&self) -> Result<Option<String>, BlockChainError>;
    // 获取一个区块
    fn get_block(&self, key: &str) -> Result<Option<Block>, BlockChainError>;
    // 获取区块链的高度
    fn get_height(&self) -> Result<Option<usize>, BlockChainError>;
    // 以事务的方式更新区块链
    fn update_blocks(&self, key: &str, block: &Block, height: usize);
    // 获取区块的迭代器
    fn get_block_iter(&self) -> Result<Box<dyn Iterator<Item = Block>>, BlockChainError>;

}


pub struct StorageIterator<T> {
    data: T
}

impl<T> StorageIterator<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> Iterator for StorageIterator<T>
where
    T: Iterator,
    T::Item: Into<Block>
{
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|v| v.into())
    }
}


