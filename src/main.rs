use sha2::{Sha256, Digest};
use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u64,
    timestamp: i64,
    verse_reference: String,
    verse_text: String,
    previous_hash: String,
    nonce: u64,
    hash: String,
}

struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {

    fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 4,
        };

        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {

        let mut block = Block {
            index: 0,
            timestamp: Utc::now().timestamp(),
            verse_reference: "Genesis 1:1".to_string(),
            verse_text: "In the beginning God created the heaven and the earth.".to_string(),
            previous_hash: "0".to_string(),
            nonce: 0,
            hash: String::new(),
        };

        block.hash = calculate_hash(&block);
        mine_block(&mut block, self.difficulty);

        self.chain.push(block);
    }

    fn add_block(&mut self, reference: String, text: String) {

        let previous = self.chain.last().unwrap();

        let mut block = Block {
            index: previous.index + 1,
            timestamp: Utc::now().timestamp(),
            verse_reference: reference,
            verse_text: text,
            previous_hash: previous.hash.clone(),
            nonce: 0,
            hash: String::new(),
        };

        block.hash = calculate_hash(&block);
        mine_block(&mut block, self.difficulty);

        self.chain.push(block);
    }
}

fn calculate_hash(block: &Block) -> String {

    let data = format!(
        "{}{}{}{}{}{}",
        block.index,
        block.timestamp,
        block.verse_reference,
        block.verse_text,
        block.previous_hash,
        block.nonce
    );

    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    hex::encode(result)
}

fn mine_block(block: &mut Block, difficulty: usize) {

    let target = "0".repeat(difficulty);

    while !block.hash.starts_with(&target) {

        block.nonce += 1;
        block.hash = calculate_hash(block);
    }

    println!("Block mined: {}", block.hash);
}

fn main() {

    let mut bible_chain = Blockchain::new();

    bible_chain.add_block(
        "Genesis 1:2".to_string(),
        "And the earth was without form, and void; and darkness was upon the face of the deep.".to_string(),
    );

    bible_chain.add_block(
        "Genesis 1:3".to_string(),
        "And God said, Let there be light: and there was light.".to_string(),
    );

    for block in bible_chain.chain {
        println!("{:#?}", block);
    }
}
