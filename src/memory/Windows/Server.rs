use std::time::Instant;
use windows::Win32::System::Memory::*;
use windows::Win32::Foundation::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let size = 26 * 1024;

    let handle = unsafe {
        CreateFileMappingA(
            INVALID_HANDLE_VALUE,
            None,
            PAGE_READWRITE,
            0,
            size as u32,
            PCSTR(b"Local\\MySharedMemory\0".as_ptr())
        )?
    };

    let buffer = unsafe {
        MapViewOfFile(
            handle,
            FILE_MAP_ALL_ACCESS,
            0,
            0,
            size
        )
    };

    let start = Instant::now();

    unsafe {
        let slice = std::slice::from_raw_parts(buffer as *const u8, size);

    }
    println!("Read time: {:?}", start.elapsed());

    unsafe {
        UnmapViewOfFile(buffer);
        CloseHandle(handle);
    }
    Ok(())
}