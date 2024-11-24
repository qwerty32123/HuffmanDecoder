# H278 High-Performance Data Processing System

> **Educational Project Notice**: A high-performance system demonstrating cross-platform shared memory management and Huffman compression in Rust, designed for processing and analyzing large data streams in real-time.

## Core Features

- **Advanced Huffman Decompression**: Optimized implementation with lookup table acceleration
- **Cross-Platform Shared Memory**: Unified interface for Windows and Unix systems
- **Real-time Data Processing**: Efficient handling of compressed data streams
- **Multi-threaded Architecture**: Concurrent processing with thread-safe shared memory access

## Architecture

### Huffman Decompression System
The project implements a sophisticated Huffman decompression system with the following components:

#### Hybrid Lookup Table
```rust
pub struct HybridLookupTable {
    pub short_table: HashMap,
    pub long_codes: HashMap,
    pub max_short_bits: u8,
}
```
- Optimized for quick lookups of common patterns
- Two-tier design for handling both short and long codes
- Configurable bit-length thresholds

#### Bit Buffer Management
```rust
pub struct BitBuffer {
    pub buffer: u64,
    pub bits_in_buffer: u8,
}
```
- Efficient bit-level operations
- Streamlined buffer management
- Optimized for performance-critical operations

### Shared Memory Implementation

#### Cross-Platform Interface
```rust
pub struct SharedMemoryServer {
    #[cfg(unix)]
    shm_fd: std::fs::File,
    #[cfg(windows)]
    mapping_handle: HANDLE,
    buffer_size: usize,
    ptr: *mut u8,
}
```

Platform-specific implementations:

##### Unix Systems
- Uses POSIX shared memory (shm_open)
- Memory-mapped file implementation
- Full permissions management

##### Windows Systems
- Windows memory-mapped files
- CreateFileMappingA implementation
- Secure handle management

## System Requirements

- Rust 1.23.2 or higher
- Operating System:
  - Linux: POSIX-compliant system
  - Windows: Windows 10 or higher
- Recommended: Multi-core processor for optimal performance

## Performance Optimizations

### Huffman Decoder
- Hybrid lookup table for fast symbol resolution
- Optimized bit buffer management
- Efficient memory usage patterns

### Shared Memory
- Zero-copy data transfer
- Minimal context switching
- Efficient cross-process communication

## Usage

### Initialize Shared Memory
```rust
let server = SharedMemoryServer::new("h278", SHARED_MEMORY_SIZE)
    .expect("Failed to create shared memory server");
```

### Set Up Huffman Decoder
```rust
let mut decoder = OptimizedHuffmanDecoder::new();
decoder.parse_header_fast(&data);
decoder.build_efficient_tree();
```

### Process Data Stream
```rust
while server.wait_for_data() {
    match server.process_data() {
        Ok(shared_mem_data) => {
            let decoded = decoder.decode_to_bytes(&shared_mem_data);
            // Process decoded data
        },
        Err(e) => eprintln!("Error processing data: {}", e),
    }
}
```

## Benchmarking

### Huffman Decoder Performance
```rust
// Example benchmark results
Small data (1KB):  ~0.05ms
Medium data (1MB): ~2.5ms
Large data (10MB): ~24ms
```


### Memory Security
- Proper cleanup on shutdown
- Secure memory permissions
- Resource leak prevention

### Data Handling
- Input validation
- Buffer overflow prevention
- Safe memory management

## Error Handling

The system implements comprehensive error handling:
- Memory allocation failures
- Decompression errors
- Invalid data handling
- Platform-specific error management

## Contributing

1. Fork the repository
2. Create your feature branch
3. Implement changes with tests
4. Submit a pull request

## License

This project is released under a permissive educational license:

Permission is hereby granted, free of charge, to any person obtaining a copy of this software, to use, copy, modify, and distribute this software and its documentation for educational purposes, subject to the following conditions:

- The above copyright notice shall be included in all copies
- For educational and learning purposes only
- No warranty is provided
- Commercial use requires separate licensing

## Disclaimer

This project is intended for educational purposes, demonstrating advanced concepts in:
- Data compression and decompression
- Cross-platform shared memory management
- High-performance data processing
- System programming in Rust
