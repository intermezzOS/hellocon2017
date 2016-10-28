#![no_std]
#![no_main]
#![feature(lang_items)]

#[cfg(not(test))]
pub mod panic;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    unsafe {
        let rust = 0xb8180 as *mut u64;
        *rust = 0x0254025302550252;

        let belt = 0xb8360 as *mut u64;
        *belt = 0x0254024c02450242;

        let wow = 0xb8298 as *mut u64;
        *wow = 0x0577056f0577;

        let rust2 = 0xb8680 as *mut u64;
        *rust2 = 0x0254025302550252;

        let such = 0xb8750 as *mut u64;
        *such = 0x0568056305750573;

        let os = 0xb8760 as *mut u64;
        *os = 0x0573056f;

        let exes = 0xb8880 as *mut u64;
        *exes = 0x0221022102210221; 

        let amaz = 0xb8988 as *mut u64;
        *amaz = 0x057a0561056d0561;

        let e = 0xb8992 as *mut u64;
        *e = 0x0565;
    };

    loop { }
}
