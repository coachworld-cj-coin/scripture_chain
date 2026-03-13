use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub verse_reference: String,
    pub verse_text: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {

    pub fn calculate_hash(&self) -> String {

        let data = format!(
            "{}{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.verse_reference,
            self.verse_text,
            self.previous_hash,
            self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(data);

        hex::encode(hasher.finalize())
    }
}
