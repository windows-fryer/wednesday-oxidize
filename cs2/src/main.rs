use wednesday_themida::SecureEngineSDK::*;
use wednesday_themida_macros::themida;

#[themida(vm = "TIGER_BLACK")]
fn something_else() {
    println!("Something else");
}

#[themida(vm = "TIGER_WHITE")]
fn main() -> ! {
    println!("Hello, world!");

    something_else();

    loop {}
}
