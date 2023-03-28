use crate::transaction::Transaction;
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature};
use rand::rngs::OsRng;
use std::error::Error;


pub struct Wallet {
    pub keypair: Keypair,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        Wallet { keypair }
    }

    pub fn public_key(&self) -> PublicKey {
        self.keypair.public
    }

    pub fn sign_transaction(&self, tx: &Transaction) -> Result<Signature, Box<dyn Error>> {
        let message = tx.to_message()?;
        let signature = self.keypair.sign(&message);
        Ok(signature)
    }

    pub fn verify_signature(&self, tx: &Transaction, signature: &Signature) -> Result<bool, Box<dyn Error>> {
        let message = tx.to_message()?;
        self.keypair.public.verify(&message, signature).is_ok()
    }
}