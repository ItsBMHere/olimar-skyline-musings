#![feature(proc_macro_hygiene)]

use smash::app::{FighterModuleAccessor, Fighter};

use skyline::{hook, install_hooks};
use skyline::hooks::InlineCtx;
use std::sync::Once;

// 0f51dd0

#[hook(offset = 0x0f51dd0)]
pub unsafe fn watch_0f51dd0(
    param_1: u32,
    param_2: i32,
    param_3: i16,
) -> u64 {
    let ori = original!()(
        param_1,
        param_2,
        param_3
    );
    println!("Unknown function 0x0f51dd0 called: {:#1x}", ori);
    ori
}

// 0f51710

#[hook(offset = 0x0f51710)]
pub unsafe fn watch_0f51710(
    param_1: u32,
    param_2: i32,
) -> u64 {
    let ori = original!()(
        param_1,
        param_2,
    );
    println!("Unknown function 0x0f51710 called: {:#1x}", ori);
    ori
}

#[hook(offset = 0x0f51e24, inline)]
pub unsafe fn watch_0f51e24_inline(
    ctx: &mut InlineCtx
) {
    static ONCE: Once = Once::new();
    static mut OFFSET: usize = 0;
    OFFSET = (*ctx.registers[8].x.as_ref() - skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64) as usize;
    
    println!("0x0f51e24 {:?}", OFFSET);
}

// 0f51e58

#[hook(offset = 0x0f51e58, inline)]
pub unsafe fn watch_0f51e58_inline(
    ctx: &mut InlineCtx
) {
    static ONCE: Once = Once::new();
    static mut OFFSET: usize = 0;
    OFFSET = (*ctx.registers[8].x.as_ref() - skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64) as usize;
    
    println!("0x0f51e58 {:?}", OFFSET);
}

// 0f51774

#[hook(offset = 0x0f51774, inline)]
pub unsafe fn watch_0f51774_inline(
    ctx: &mut InlineCtx
) {
    static ONCE: Once = Once::new();
    static mut OFFSET: usize = 0;
    OFFSET = (*ctx.registers[8].x.as_ref() - skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64) as usize;
    
    println!("0x0f51774 {:?}", OFFSET);
}

#[skyline::main(name = "skyline_rs_template")]
pub fn main() {
    install_hooks!(
        watch_0f51dd0,
        watch_0f51710,
        watch_0f51e24_inline,
        watch_0f51e58_inline,
        watch_0f51774_inline,
    );
}