use std::path::Path;

use sled::{Db, IVec, transaction::TransactionResult};
use crate::storage::Storage;
use crate::storage::StorageIterator;
use crate::utils::{deserialize, serialize};
use crate::{storage::{TIP_KEY, HEIGHT, TABLE_OF_BLOCK}, Block, };
use crate::{error::BlockChainError};


pub struct SledDb {
    db: Db
}

impl SledDb {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            db: sled::open(path).unwrap()
        }
    }

    fn get_full_key(table: &str, key: &str) -> String {
        format!("{}:{}", table, key)
    }
}

impl Storage for SledDb {
    fn get_tip(&self) -> Result<Option<String>, BlockChainError> {
        let result = self.db.get(TIP_KEY)?;

        // 检查是否有数据
        if let Some(ivec) = result {
            // 尝试将数据反序列化为String
            let deserialized: Result<String, _> = deserialize::<String>(&ivec.to_vec());

            // 如果反序列化成功，返回Ok(Some(String))
            deserialized.map(Some).map_err(BlockChainError::from)
        } else {
            // 如果没有数据，返回Ok(None)
            Ok(None)
        }
    }

    fn get_block(&self, key: &str) -> Result<Option<Block>, BlockChainError> {
        let name = Self::get_full_key(TABLE_OF_BLOCK, key);
        let result = self.db.get(name)?.map(|v| v.into());
        Ok(result)
    }

    fn get_height(&self) -> Result<Option<usize>, BlockChainError> {
        let result = self.db.get(HEIGHT)?;

        // 检查是否有数据
        if let Some(ivec) = result {
            // 尝试将IVec转为Vec<u8>
            let vec = ivec.to_vec();

            // 尝试将Vec<u8>转换为usize
            let height = deserialize::<usize>(&vec)?;

            // 返回Ok(Some(usize))
            Ok(Some(height))
        } else {
            // 如果没有数据，返回Ok(None)
            Ok(None)
        }
    }

    fn update_blocks(&self, key: &str, block: &Block, height: usize) {
        let _: TransactionResult<(), ()> = self.db.transaction(|db| {
            let name = Self::get_full_key(TABLE_OF_BLOCK, key);
            db.insert(name.as_str(), serialize(block).unwrap())?;
            db.insert(TIP_KEY, serialize(key).unwrap())?;
            db.insert(HEIGHT, serialize(&height).unwrap())?;
            db.flush();
            Ok(())
        });
    }

    fn get_block_iter(&self) -> Result<Box<dyn Iterator<Item = Block>>, BlockChainError> {
        let prefix = format!("{}:", TABLE_OF_BLOCK);
        let iter = StorageIterator::new(self.db.scan_prefix(prefix));
        Ok(Box::new(iter))
    }
}


impl From<IVec> for Block {
    fn from(v: IVec) -> Self {
        let result = deserialize::<Block>(&v.to_vec());
        match result {
            Ok(block) => block,
            Err(_) => Block::default(),
        }
    }
}

impl From<Result<(IVec, IVec), sled::Error>> for Block {
    fn from(result: Result<(IVec, IVec), sled::Error>) -> Self {
        match result {
            Ok((_, v)) => match deserialize::<Block>(&v.to_vec()) {
                Ok(block) => block,
                Err(_) => Block::default(),
            },
            Err(_) => Block::default(),
        }
    }
}

