use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::UnixListener;
use std::time::Instant;

fn main() {
    let _ = fs::remove_file("/tmp/rust_socket");

    let listener = UnixListener::bind("/tmp/rust_socket").unwrap();
    println!("Server listening...");

    // UDS
    {
        let (mut socket, _) = listener.accept().unwrap();
        let start = Instant::now();
        let mut buffer = vec![0u8; 26 * 1024];
        socket.read_exact(&mut buffer).unwrap();
        println!("Socket receive time: {:?}", start.elapsed());
    }

    // Shared memory
    {
        let (mut socket, _) = listener.accept().unwrap();
        let mut buffer = [0u8; 32];
        socket.read_exact(&mut buffer).unwrap();
        let path = String::from_utf8_lossy(&buffer).trim_end_matches('\0').to_string();

        let start = Instant::now();
        let data = fs::read(&path).unwrap();
        assert_eq!(data.len(), 26 * 1024);
        println!("Shared memory receive time: {:?}", start.elapsed());
    }
}