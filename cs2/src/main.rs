use wednesday_themida::SecureEngineSDK::*;

fn main() {
    unsafe {
        VM_TIGER_WHITE_START();
    };

    println!("Hello, world!");

    unsafe {
        VM_TIGER_WHITE_END();
    };
}
