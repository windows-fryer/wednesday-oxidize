use winapi::um::winnt::HANDLE;

struct _Process {
    pid: u32,
    handle: HANDLE,
}
