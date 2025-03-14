use crate::System;

use winapi::{
    shared::{minwindef::MAX_PATH, ntstatus::STATUS_NOT_FOUND, windef::HWND},
    um::{
        errhandlingapi::GetLastError,
        handleapi::CloseHandle,
        processthreadsapi::OpenProcess,
        winbase::QueryFullProcessImageNameW,
        winnt::{HANDLE, PROCESS_ALL_ACCESS},
        winuser::{GetWindowTextLengthW, GetWindowTextW},
    },
};

#[derive(Debug, PartialEq, Eq)]
pub struct Process {
    pid: u32,
    handle: HANDLE,
}

impl Default for Process {
    fn default() -> Self {
        Self {
            pid: 0,
            handle: std::ptr::null_mut(),
        }
    }
}

impl Process {
    pub fn from_name(name: &str) -> Result<Self, u32> {
        let processes = System::processes()?;

        for process in processes {
            if process.name()? == name {
                return Ok(process);
            }
        }

        Err(STATUS_NOT_FOUND as u32)
    }

    pub fn from_pid(pid: u32) -> Result<Self, u32> {
        unsafe {
            // SAFETY:
            // - The `pid` passed in is assumed to be a valid process identifier.
            // - OpenProcess is called with PROCESS_ALL_ACCESS; while this grants extensive rights,
            //   it is acceptable here given our current needs (it may be restricted in the future).
            // - We immediately check if the returned handle is null, ensuring that we only proceed
            //   with a valid handle.
            // - If the handle is null, we retrieve the error via GetLastError to handle failure.

            // In the future, maybe change PROCESS_ALL_ACCESS to something more restrictive
            let handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid);

            if handle == std::ptr::null_mut() {
                return Err(GetLastError());
            }

            Ok(Self { pid, handle })
        }
    }

    pub fn path(&self) -> Result<String, u32> {
        unsafe {
            // SAFETY:
            // - We allocate a buffer with MAX_PATH elements, which is guaranteed to be
            //   large enough to contain the full process path.
            // - The Windows API function QueryFullProcessImageNameW writes at most `length` characters
            //   and, per documentation, ensures the buffer is null-terminated.
            // - These guarantees allow safe conversion from UTF-16 to a Rust String with String::from_utf16_lossy.

            let mut name = [0u16; MAX_PATH];
            let mut length = name.len() as u32;

            if QueryFullProcessImageNameW(self.handle, 0, name.as_mut_ptr(), &mut length) == 0 {
                return Err(GetLastError());
            }

            name[length as usize] = '\0' as u16;

            Ok(String::from_utf16_lossy(&name[..length as usize]))
        }
    }

    pub fn name(&self) -> Result<String, u32> {
        Ok(self
            .path()?
            .split('\\')
            .last()
            .expect("Invalid path")
            .to_string())
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        if self.handle == std::ptr::null_mut() {
            return;
        }

        unsafe {
            // SAFETY:
            // - CloseHandle is a Windows API function used to release operating system handles.
            // - We only call CloseHandle when self.handle is non-null, ensuring we have a valid handle.
            // - Since the handle was previously obtained via OpenProcess, calling CloseHandle here
            //   properly cleans up the resource.
            // - If CloseHandle fails, we panic to indicate an unexpected error during resource cleanup.

            if CloseHandle(self.handle) == 0 {
                panic!("Failed to close process handle: {}", GetLastError());
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Iter {
    processes: Vec<u32>,
    index: usize,
}

impl Iter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(processes: Vec<u32>) -> Self {
        Self {
            processes,
            index: 0,
        }
    }
}

impl Iterator for Iter {
    type Item = Process;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.processes.len() {
            let process = Process::from_pid(self.processes[self.index]);

            self.index += 1;

            // If the process failed to be created, we skip it and try the next one. No need to poison the iterator.
            if process.is_err() {
                return self.next();
            }

            return Some(process.unwrap());
        }

        None
    }
}
