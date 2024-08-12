use serde::{Serialize, Deserialize};
use crate::error::BlockChainError;
use sha3::{Sha3_256, Digest};
use hex;
use std::fmt::Debug;
use tracing::info;

pub fn serialize<T>(data: &T) -> Result<Vec<u8>, BlockChainError>
where
    T: Serialize + ?Sized + std::fmt::Debug
{
    Ok(bincode::serialize(data)?)
}

#[allow(dead_code)]
pub fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, BlockChainError>
where
    T: Deserialize<'a> + ?Sized
{
    Ok(bincode::deserialize(data)?)
}
pub fn hash_to_str(data: &[u8]) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    let result = hasher.finalize();

    // 5. 将结果转换为十六进制字符串
    let result_str = hex::encode(result);
    result_str
}

#[allow(dead_code)]
pub fn hash_to_u8(data: &[u8], out: &mut [u8]) {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    let result = hasher.finalize();
    out.copy_from_slice(&result[..out.len()]);
}
