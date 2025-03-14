#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]

use wednesday_themida::SecureEngineSDK::*;
use wednesday_themida_macros::themida;

use wednesday_mem::System;

#[themida(vm = "TIGER_WHITE")]
fn main() {
    let processes = System::processes().unwrap();

    for process in processes {
        let name = process.title().unwrap();

        println!("{name}");
    }
}
