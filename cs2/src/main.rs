#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]

use wednesday_themida::SecureEngineSDK::*;
use wednesday_themida_macros::themida;

#[themida(vm = "TIGER_RED")]
fn test() {
    println!("Hello, world! from TIGER_RED");
}

#[themida(vm = "TIGER_WHITE")]
fn main() -> ! {
    println!("Hello, world!");

    #[themida(vm = "TIGER_BLACK")]
    {
        println!("Hello, world! from TIGER_BLACK");
    }

    test();

    loop {}
}
