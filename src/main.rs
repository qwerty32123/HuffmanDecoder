mod huffman;
mod service;

use std::time::Instant;
use crate::huffman::optimized_huffman_decoder::OptimizedHuffmanDecoder;
use crate::service::reader_service::{parse_stock_bytes};

const MAX_SHORT_BITS: usize = 8;


fn main() {

    let mut decoder = OptimizedHuffmanDecoder::new();
    let start = Instant::now();
    let decoded = decoder.decode_file( "main.bin"); // This returns Result<Vec<u8>, Error>
    let duration = start.elapsed();
    let result = match decoded {
        Ok(bytes) => parse_stock_bytes(&bytes),
        Err(e) => {
            eprintln!("Error decoding file: {}", e);
            return ;
        }
    };



    println!("Results: {:?}", result);

    let milliseconds = duration.as_secs() as f64 * 1000.0 + duration.subsec_micros() as f64 / 1000.0;

    println!("Time taken: {:.3} ms", milliseconds);
}