mod huffman;
mod service;
mod memory;
mod discord;

use std::env;
use crate::huffman::decoder::OptimizedHuffmanDecoder;
use crate::service::reader_service::parse_stock_bytes;
use crate::memory::server::SharedMemoryServer;

use std::time::{Instant};
use serenity::prelude::*;
use dotenv::dotenv;
use serenity::model::id::ChannelId;

const CHANNEL: u64 = 1309587781252546710;
const CHANNEL_ID: ChannelId = ChannelId::new(CHANNEL);

const MAX_SHORT_BITS: usize = 8;
const SHARED_MEMORY_SIZE: usize = 1024 * 1024; // 1MB



#[tokio::main]
async fn main() {
    dotenv().ok();

    let secret = env::var("BOT_SECRET").expect("Missing secret!");
    let client = Client::builder(secret, GatewayIntents::empty())
        .await
        .expect("Failed to create client");
    let http = client.http.clone();

    let mut server = SharedMemoryServer::new("Test2", SHARED_MEMORY_SIZE)
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
                println!("{:?}", &result);

                let http = http.clone();

                if !result.is_empty() {
                    tokio::spawn(async move {
                        if let Err(why) = CHANNEL_ID.say(&http, "ew stock data!").await {
                            eprintln!("Error sending Discord message: {:?}", why);
                        }
                    });
                }

                println!("Time taken: {} ns", duration.as_nanos());
            },
            Err(e) => eprintln!("Error processing data: {}", e),
        }
    }
}
