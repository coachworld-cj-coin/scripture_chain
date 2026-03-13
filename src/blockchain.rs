use crate::block::Block;
use chrono::Utc;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {

    pub fn new() -> Self {

        let mut bc = Blockchain {
            chain: Vec::new(),
            difficulty: 4,
        };

        bc.create_genesis_block();
        bc
    }

    fn create_genesis_block(&mut self) {

        let mut block = Block {
            index: 0,
            timestamp: Utc::now().timestamp(),
            verse_reference: "Genesis 1:1".into(),
            verse_text: "In the beginning God created the heaven and the earth.".into(),
            previous_hash: "0".into(),
            nonce: 0,
            hash: "".into(),
        };

        mine_block(&mut block, self.difficulty);

        self.chain.push(block);
    }

    pub fn add_block(&mut self, reference: String, text: String) {

        let prev = self.chain.last().unwrap();

        let mut block = Block {
            index: prev.index + 1,
            timestamp: Utc::now().timestamp(),
            verse_reference: reference,
            verse_text: text,
            previous_hash: prev.hash.clone(),
            nonce: 0,
            hash: "".into(),
        };

        mine_block(&mut block, self.difficulty);

        self.chain.push(block);
    }
}

pub fn mine_block(block: &mut Block, difficulty: usize) {

    let target = "0".repeat(difficulty);

    loop {

        block.hash = block.calculate_hash();

        if block.hash.starts_with(&target) {
            break;
        }

        block.nonce += 1;
    }

    println!("Block mined: {}", block.hash);
}
