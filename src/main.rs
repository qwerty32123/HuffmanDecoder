mod huffman;
mod service;
mod memory;
mod discord;

mod data;
use std::env;
use crate::huffman::decoder::OptimizedHuffmanDecoder;
use crate::service::reader_service::parse_stock_bytes;
use crate::memory::server::SharedMemoryServer;

use std::time::{Instant};
use serenity::prelude::*;
use dotenv::dotenv;
use serenity::model::id::ChannelId;
use crate::data::cache::{CacheManager, OutfitSearcher};

const CHANNEL: u64 = 1309587781252546710;
const CHANNEL_ID: ChannelId = ChannelId::new(CHANNEL);

const MAX_SHORT_BITS: usize = 8;
const SHARED_MEMORY_SIZE: usize = 1024 * 1024; // 1MB


#[tokio::main]
async fn main() {
    dotenv().ok();

    let cache_manager = CacheManager::new(3600);

    // Load cached data
    let allCostumes = match cache_manager.load_cache("src/data/db/all.bin") {
        Ok(list) => list,
        Err(err) => {
            eprintln!("Failed to load cache: {}", err);
            return;  // or handle the error in another way
        }
    };

    let allBloodys = match cache_manager.load_cache("src/data/db/bloody.bin") {
        Ok(list) => list,
        Err(err) => {
            eprintln!("Failed to load cache: {}", err);
            return;
        }
    };

    // Create searcher
    let searcher = OutfitSearcher::new(allCostumes, allBloodys);

    let secret ="";
    let client = Client::builder(secret, GatewayIntents::empty())
        .await
        .expect("Failed to create client");
    let http = client.http.clone();

    let mut server = SharedMemoryServer::new("h278", SHARED_MEMORY_SIZE)
        .expect("Failed to create shared memory server");
    let mut decoder = OptimizedHuffmanDecoder::new();

    println!("Server started. Waiting for data...");

    while server.wait_for_data() {
        let start = Instant::now();
        match server.process_data() {
            Ok(shared_mem_data) => {
                let duration = start.elapsed();
                let decoded = decoder.decode_to_bytes(&shared_mem_data);
                let result = parse_stock_bytes(&decoded);

                if !result.is_empty() {
                    let search_ids: Vec<String> = result
                        .iter()
                        .map(|(id, _count)| id.to_string())
                        .collect();

                    let search_results = searcher.search_standard(&search_ids);

                    // Filter for items only in first list (has first value but no second value)
                    for (id, first_value, second_value) in &search_results {
                        if second_value.is_none() && first_value.is_some() {
                            let message = format!(
                                "New stock data! Item: {} (ID: {})",
                                first_value.as_ref().unwrap(),
                                id
                            );

                            // Clone http for each iteration
                            let http = http.clone();

                            tokio::spawn(async move {
                                if let Err(why) = CHANNEL_ID.say(&http, message).await {
                                    eprintln!("Error sending Discord message: {:?}", why);
                                }
                            });
                        }
                    }
                }

            },
            Err(e) => eprintln!("Error processing data: {}", e),
        }
    }
}
