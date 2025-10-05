use crate::models::Blockchain;
use serde_json;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{Filter, Reply};

pub struct NetworkManager {
    blockchain: Arc<RwLock<Blockchain>>,
    port: u16,
    peers: Vec<String>,
}

impl NetworkManager {
    pub fn new(blockchain: Arc<RwLock<Blockchain>>, port: u16, peers: Vec<String>) -> Self {
        NetworkManager {
            blockchain,
            port,
            peers,
        }
    }

    pub async fn start(&self) {
        let blockchain = self.blockchain.clone();

        let get_chain = warp::path("chain")
            .and(warp::get())
            .map(move || {
            let chain = blockchain.blocking_read();
            warp::reply::json(&*chain)
        });

        let sync_chain = warp::path("sync")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_blockchain(self.blockchain.clone()))
            .and_then(handle_sync);

        let routes = get_chain.or(sync_chain);

        println!("Starting network on port {}", self.port);

        warp::serve(routes)
            .run(([0, 0, 0, 0], self.port))
            .await;
    }

    pub async fn sync_with_peers(&self) -> Result<usize, Box<dyn std::error::Error>> {
        if self.peers.is_empty() {
            println!("Nothing to sync with.");
            return Ok(0);
        }

        println!("{} Syncing...", self.peers.len());

        let sync_count = {
            let mut count = 0;
            for peer in &self.peers {
                match self.fetch_chain(peer).await {
                    Ok(remote_chain) => {
                        let mut local_chain = self.blockchain.write().await;
                        if local_chain.replace_chain(remote_chain.chain) {
                            count += 1;
                            println!("Synced with peer: {}", peer);
                        }
                    }
                    Err(e) => {
                        println!("Failed to fetch chain from {}: {}", peer, e);
                    }
                }
            }
            count
        };

        println!("Sync complete. Synced with {} peers.", sync_count);
        Ok(sync_count)
    }

    async fn fetch_chain(&self, peer: &str) -> Result<Blockchain, Box<dyn std::error::Error>> {
        let url = format!("http://{}/chain", peer);
        let response = reqwest::get(&url).await?;
        let body = response.text().await?;
        let chain: Blockchain = serde_json::from_str(&body)?;
        Ok(chain)
    }
}

fn with_blockchain(
    blockchain: Arc<RwLock<Blockchain>>,
) -> impl Filter<Extract = (Arc<RwLock<Blockchain>>,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || blockchain.clone())
}

async fn handle_sync( 
    new_chain: Blockchain, 
    blockchain: Arc<RwLock<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut local_chain = blockchain.write().await;

    if local_chain.replace_chain(new_chain.chain) {
        Ok(warp::reply::json(&"SYNC SUCCESSFUL!"))
    } else {
        Ok(warp::reply::json(&"SYNC FAILED!"))
    }
}
