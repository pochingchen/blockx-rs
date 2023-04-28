use crate::block::{Block};
use anyhow::Result;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![Block::new_genesis_block()],
        }
    }

    pub fn add_block(&mut self, data: String) -> Result<()> {
        let prev = self.blocks.last().unwrap();
        let new_block = Block::new_block(data, prev.get_hash(), prev.get_height()+1)?;
        self.blocks.push(new_block);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain() {
        let mut bc = Blockchain::new();
        bc.add_block("tx".to_string()).expect("add tx error");
        bc.add_block("tx2".to_string()).expect("add tx error");
        bc.add_block("tc3".to_string()).expect("add tx error");

        dbg!(bc);
    }
}