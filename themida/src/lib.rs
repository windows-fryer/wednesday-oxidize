#![no_main]

#[allow(non_snake_case)]
#[cfg(feature = "themida")]
pub mod SecureEngineSDK;

#[cfg(not(feature = "themida"))]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
pub mod SecureEngineSDK {
    pub unsafe fn VM_START() {}
    pub unsafe fn VM_END() {}
    pub unsafe fn STR_ENCRYPT_START() {}
    pub unsafe fn STR_ENCRYPT_END() {}
    pub unsafe fn STR_ENCRYPTW_START() {}
    pub unsafe fn STR_ENCRYPTW_END() {}
    pub unsafe fn UNPROTECTED_START() {}
    pub unsafe fn UNPROTECTED_END() {}
    pub unsafe fn CHECK_DEBUGGER(var: &mut i32, val: i32) {}
    pub unsafe fn CHECK_PROTECTION(var: &mut i32, val: i32) {}
    pub unsafe fn CHECK_CODE_INTEGRITY(var: &mut i32, val: i32) {}
    pub unsafe fn CHECK_REGISTRATION(var: &mut i32, val: i32) {}
    pub unsafe fn CHECK_VIRTUAL_PC(var: &mut i32, val: i32) {}
    pub unsafe fn VM_TIGER_WHITE_START() {}
    pub unsafe fn VM_TIGER_WHITE_END() {}
    pub unsafe fn VM_TIGER_RED_START() {}
    pub unsafe fn VM_TIGER_RED_END() {}
    pub unsafe fn VM_TIGER_BLACK_START() {}
    pub unsafe fn VM_TIGER_BLACK_END() {}
    pub unsafe fn VM_FISH_WHITE_START() {}
    pub unsafe fn VM_FISH_WHITE_END() {}
    pub unsafe fn VM_FISH_RED_START() {}
    pub unsafe fn VM_FISH_RED_END() {}
    pub unsafe fn VM_FISH_BLACK_START() {}
    pub unsafe fn VM_FISH_BLACK_END() {}
    pub unsafe fn VM_PUMA_WHITE_START() {}
    pub unsafe fn VM_PUMA_WHITE_END() {}
    pub unsafe fn VM_PUMA_RED_START() {}
    pub unsafe fn VM_PUMA_RED_END() {}
    pub unsafe fn VM_PUMA_BLACK_START() {}
    pub unsafe fn VM_PUMA_BLACK_END() {}
    pub unsafe fn VM_SHARK_WHITE_START() {}
    pub unsafe fn VM_SHARK_WHITE_END() {}
    pub unsafe fn VM_SHARK_RED_START() {}
    pub unsafe fn VM_SHARK_RED_END() {}
    pub unsafe fn VM_SHARK_BLACK_START() {}
    pub unsafe fn VM_SHARK_BLACK_END() {}
    pub unsafe fn VM_DOLPHIN_WHITE_START() {}
    pub unsafe fn VM_DOLPHIN_WHITE_END() {}
    pub unsafe fn VM_DOLPHIN_RED_START() {}
    pub unsafe fn VM_DOLPHIN_RED_END() {}
    pub unsafe fn VM_DOLPHIN_BLACK_START() {}
    pub unsafe fn VM_DOLPHIN_BLACK_END() {}
    pub unsafe fn VM_EAGLE_WHITE_START() {}
    pub unsafe fn VM_EAGLE_WHITE_END() {}
    pub unsafe fn VM_EAGLE_RED_START() {}
    pub unsafe fn VM_EAGLE_RED_END() {}
    pub unsafe fn VM_EAGLE_BLACK_START() {}
    pub unsafe fn VM_EAGLE_BLACK_END() {}
    pub unsafe fn VM_LION_WHITE_START() {}
    pub unsafe fn VM_LION_WHITE_END() {}
    pub unsafe fn VM_LION_RED_START() {}
    pub unsafe fn VM_LION_RED_END() {}
    pub unsafe fn VM_LION_BLACK_START() {}
    pub unsafe fn VM_LION_BLACK_END() {}
    pub unsafe fn VM_COBRA_WHITE_START() {}
    pub unsafe fn VM_COBRA_WHITE_END() {}
    pub unsafe fn VM_COBRA_RED_START() {}
    pub unsafe fn VM_COBRA_RED_END() {}
    pub unsafe fn VM_COBRA_BLACK_START() {}
    pub unsafe fn VM_COBRA_BLACK_END() {}
    pub unsafe fn VM_WOLF_WHITE_START() {}
    pub unsafe fn VM_WOLF_WHITE_END() {}
    pub unsafe fn VM_WOLF_RED_START() {}
    pub unsafe fn VM_WOLF_RED_END() {}
    pub unsafe fn VM_WOLF_BLACK_START() {}
    pub unsafe fn VM_WOLF_BLACK_END() {}
    pub unsafe fn VM_MUTATE_ONLY_START() {}
    pub unsafe fn VM_MUTATE_ONLY_END() {}
    pub unsafe fn VM_FALCON_TINY_START() {}
    pub unsafe fn VM_FALCON_TINY_END() {}
}
