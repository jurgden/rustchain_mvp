use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub value: u64,
    pub id: String,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, value: u64) -> Transaction {
        let id = Self.generate_id();
        Transaction {
            sender,
            recipient,
            value,
            id,
        }
    }

    fn generate_id() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn to_message(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let serialized_tx = serde_json::to_string(&self)?;
        Ok(serialized_tx.into_bytes())
    }
}
