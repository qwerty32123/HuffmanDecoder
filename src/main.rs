mod huffman;
mod service;
mod memory;
mod data;
use crate::huffman::decoder::OptimizedHuffmanDecoder;
use crate::service::reader_service::parse_stock_bytes;
use crate::service::channel::{ DiscordChannels};
use crate::memory::server::SharedMemoryServer;

use std::time::{Instant};
use serenity::prelude::*;
use dotenv::dotenv;
use crate::data::cache::{CacheManager, H9123};

//1309907112964653117
//1309907121663508601
//1309907129347473478
//1309907135949180988



const MAX_SHORT_BITS: usize = 8;
const SHARED_MEMORY_SIZE: usize = 1024 * 1024; // 1MB


#[tokio::main]
async fn main() {
    dotenv().ok();

    let cache_manager = CacheManager::new(3600);

    // Load cached data
    let listone = match cache_manager.load_cache("src/data/db/all.bin") {
        Ok(list) => list,
        Err(err) => {
            eprintln!("Failed to load cache: {}", err);
            return;  // or handle the error in another way
        }
    };

    let listwo = match cache_manager.load_cache("src/data/db/bloody.bin") {
        Ok(list) => list,
        Err(err) => {
            eprintln!("Failed to load cache: {}", err);
            return;
        }
    };

    // Create searcher
    let searcher = H9123::new(listone, listwo);


    let secret ="";
    let client = Client::builder(secret, GatewayIntents::empty())
        .await
        .expect("Failed to create client");

    let discord_channels = DiscordChannels::new(client.http.clone());


    let mut server = SharedMemoryServer::new("h278", SHARED_MEMORY_SIZE)
        .expect("Failed to create shared memory server");
    let mut decoder = OptimizedHuffmanDecoder::new();

    println!("Server started. Waiting for data...");

    while server.wait_for_data() {
        let start = Instant::now();
        match server.process_data() {
            Ok((client_id, shared_mem_data)) => {
                let duration = start.elapsed();
                let decoded = decoder.decode_to_bytes(&shared_mem_data);
                let result = parse_stock_bytes(&decoded);

                if !result.is_empty() {
                    let search_ids: Vec<String> = result
                        .iter()
                        .map(|(id, _count)| id.to_string())
                        .collect();

                    let search_results = searcher.search_standard(&search_ids);
                    println!("Time taken: {} ns", duration.as_nanos());

                    // Filter for items only in first list (has first value but no second value)
                    for (id, first_value, second_value) in &search_results {
                        if second_value.is_none() && first_value.is_some() {
                            if let Some(&(_, stock)) = result.iter().find(|(result_id, _)| result_id.to_string() == *id) {
                                let message: String = format!(
                                    "@everyone Client {}: Item: {} (ID: {}) - Stock: {}",
                                    client_id,
                                    first_value.as_ref().unwrap(),
                                    id,
                                    stock
                                );

                                // Clone http for each iteration
                                discord_channels.send_message(client_id, message);
                                println!("COMPLETE OUTFIT NOTIF TIME TAKEN: {} ns", duration.as_nanos());

                            }
                        }
                    }
                }
            },
            Err(e) => eprintln!("Error processing data: {}", e),
        }
    }
}
