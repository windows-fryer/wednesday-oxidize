pub mod process;

use process::Iter;

use winapi::um::{errhandlingapi::GetLastError, psapi::EnumProcesses};

pub struct System;

impl System {
    pub fn processes() -> Result<Iter, u32> {
        const SIZE: usize = 1024;
        const BUFFER_SIZE: usize = SIZE * size_of::<u32>();

        let mut processes = vec![0; SIZE];

        unsafe {
            // SAFETY:
            // - The 'processes' vector is allocated with a known size and its memory is contiguous.
            // - Its pointer, obtained via as_mut_ptr(), is valid for BUFFER_SIZE bytes.
            // - EnumProcesses is expected to write at most BUFFER_SIZE bytes of process IDs.
            // - We check the number of bytes written and, if it exactly equals BUFFER_SIZE,
            //   we assume that the buffer might have been too small; hence, we reallocate more space.
            // - Once EnumProcesses writes less than BUFFER_SIZE bytes, we shrink the vector to the exact
            //   number of process IDs returned, ensuring that subsequent accesses are safe.

            let mut written = 0;

            loop {
                if EnumProcesses(processes.as_mut_ptr(), BUFFER_SIZE as u32, &mut written) == 0 {
                    return Err(GetLastError());
                }

                if written != BUFFER_SIZE as u32 {
                    break;
                }

                processes.resize(processes.len() + SIZE, 0);
            }

            processes.resize(written as usize / size_of::<u32>(), 0);
        }

        Ok(Iter::from(processes))
    }
}
