use crate::block::Block;
use crate::blockchain::Blockchain;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tarpc::context;
use tarpc::serde_transport::tcp;
use tarpc::server::BaseChannel;
use tokio_serde::formats::Json;

tarpc::service! {
    #[rpc]
    pub trait P2PService {
        async fn connect(address: String) -> Result<(), Box<dyn Error>>;
        async fn broadcast_block(block: Block) -> Result<(), Box<dyn Error>>;
    }
}

impl P2PService for Arc<Mutex<Blockchain>> {
    async fn connect(
        self,
        context: context::Context,
        address: String,
    ) -> Result<(), Box<dyn Error>> {
        let transport = tcp::listen("0.0.0.0:0", Json::default).await?;
        let addr = transport.local_addr();
        println!("Listening on {}", addr);

        let client_transport = tcp::connect(address, Json::default).await?;
        let mut client = P2PServiceClient::new(client_transport);

        let block = self.lock().unwrap().blocks.last().unwrap().clone();
        let response = client.broadcast_block(context.clone(), block).await?;

        match response {
            Ok(_) => println!("Connected to peer and sent our latest block."),
            Err(e) => eprintln!("Error broadcasting block to peer: {}", e),
        }

        let server = self.clone();
        let base_channel = BaseChannel::with_defaults(transport);
        let server_task = base_channel
            .respond_with(server.serve())
            .execute()
            .await
            .map_err(|e| format!("Error running server: {}", e))?;

        Ok(())
    }

    async fn broadcast_block(
        self,
        context: context::Context,
        block: Block,
    ) -> Result<(), Box<dyn Error>> {
        let mut blockchain = self.lock().unwrap();
        if block.prev_block_hash == blockchain.blocks.last().unwrap().hash {
            println!("Received new block: {:?}", block);
            blockchain.blocks.push(block);
            Ok(())
        } else {
            Err(format!("Block's prev_block_hash does not match our latest block's hash.").into())
        }
    }
}
