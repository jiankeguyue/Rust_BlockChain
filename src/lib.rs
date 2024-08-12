mod error;
mod utils;
mod blocks;
mod storage;

pub use storage::*;
pub use blocks::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
