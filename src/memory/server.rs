use std::{io, ptr};
use std::sync::{Arc, Condvar, Mutex};

#[cfg(windows)]
use windows::Win32::System::Memory::{
    CreateFileMappingA, MapViewOfFile, FILE_MAP_ALL_ACCESS,
    PAGE_READWRITE,
};
#[cfg(windows)]
use windows::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
#[cfg(windows)]
use windows::core::PCSTR;

#[cfg(unix)]
use std::fs::OpenOptions;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
#[cfg(unix)]
use std::os::unix::io::AsRawFd;
#[cfg(unix)]
use libc;

#[cfg(windows)]
use windows::Win32::System::Threading::{CreateEventA, WaitForSingleObject, INFINITE};

pub struct SharedMemoryServer {
    #[cfg(unix)]
    shm_fd: std::fs::File,
    #[cfg(windows)]
    mapping_handle: HANDLE,
    #[cfg(windows)]
    event_handle: HANDLE,
    buffer_size: usize,
    ptr: *mut u8,
    data_ready: Arc<(Mutex<bool>, Condvar)>,
}

impl SharedMemoryServer {
    pub fn new(name: &str, size: usize) -> io::Result<Self> {
        let data_ready = Arc::new((Mutex::new(false), Condvar::new()));

        #[cfg(unix)]
        {
            // Create shared memory file
            let path = format!("/dev/shm/{}", name);
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .mode(0o666)
                .open(&path)?;

            // Set the size of the shared memory segment
            file.set_len(size as u64)?;

            // Map the shared memory into our address space
            let ptr = unsafe {
                libc::mmap(
                    ptr::null_mut(),
                    size,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_SHARED,
                    file.as_raw_fd(),
                    0,
                ) as *mut u8
            };

            if ptr == libc::MAP_FAILED as *mut u8 {
                return Err(io::Error::last_os_error());
            }

            Ok(SharedMemoryServer {
                shm_fd: file,
                buffer_size: size,
                ptr,
                data_ready,
            })
        }

        #[cfg(windows)]
        {
            unsafe {
                // Create unique names for Windows shared memory objects
                let name_cstr = format!("{}\0", name);
                let event_name = format!("{}_event\0", name);

                // Create the file mapping object
                let mapping_handle = CreateFileMappingA(
                    INVALID_HANDLE_VALUE,
                    None,
                    PAGE_READWRITE,
                    0,
                    size as u32,
                    PCSTR(name_cstr.as_ptr()),
                )?;

                // Create the event for signaling
                let event_handle = CreateEventA(
                    None,
                    false, // Auto-reset event
                    false, // Initial state non-signaled
                    PCSTR(event_name.as_ptr()),
                )?;

                // Map view of the file
                let ptr = MapViewOfFile(
                    mapping_handle,
                    FILE_MAP_ALL_ACCESS,
                    0,
                    0,
                    size,
                );

                if ptr.Value.is_null() {
                    return Err(io::Error::last_os_error());
                }

                Ok(SharedMemoryServer {
                    mapping_handle,
                    event_handle,
                    buffer_size: size,
                    ptr: ptr.Value as *mut u8,
                    data_ready,
                })
            }
        }
    }

    pub fn wait_for_data(&self) -> bool {
        #[cfg(windows)]
        unsafe {
            WaitForSingleObject(self.event_handle, INFINITE);
            true
        }

        #[cfg(unix)]
        {
            let (lock, cvar) = &*self.data_ready;
            let mut data_ready = lock.lock().unwrap();
            *data_ready = false;
            drop(cvar.wait(data_ready).unwrap());
            true
        }
    }

    pub fn process_data(&mut self) -> io::Result<(u32, Vec<u8>)> {
        // Read total size (first 4 bytes)
        let total_size = unsafe { *(self.ptr as *const u32) };

        // Read client ID (next 4 bytes)
        let client_id = unsafe { *(self.ptr.add(4) as *const u32) };

        // Read actual response data (remaining bytes)
        let data_size = total_size - 4; // Subtract size of client ID
        let mut data = vec![0u8; data_size as usize];
        unsafe {
            ptr::copy_nonoverlapping(
                self.ptr.add(8), // Skip total_size (4) and client_id (4)
                data.as_mut_ptr(),
                data_size as usize
            );
        }

        Ok((client_id, data))
    }
}

impl Drop for SharedMemoryServer {
    fn drop(&mut self) {
        unsafe {
            #[cfg(windows)]
            {
                windows::Win32::System::Memory::UnmapViewOfFile(
                    windows::Win32::System::Memory::MEMORY_MAPPED_VIEW_ADDRESS {
                        Value: self.ptr as _
                    }
                );
                windows::Win32::Foundation::CloseHandle(self.mapping_handle);
                windows::Win32::Foundation::CloseHandle(self.event_handle);
            }

            #[cfg(unix)]
            {
                libc::munmap(self.ptr as *mut libc::c_void, self.buffer_size);
            }
        }
    }
}