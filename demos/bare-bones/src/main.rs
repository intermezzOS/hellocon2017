#![no_std]
#![no_main]
#![feature(lang_items)]

#[cfg(not(test))]
pub mod panic;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    unsafe {
        let hell = 0xb8180 as *mut u64;
        *hell = 0x026c026c02650268;
      
        let o = 0xb8188 as *mut u64;
        *o = 0x026f;

        let wow = 0xb8298 as *mut u64;
        *wow = 0x0577056f0577;

        let hell2 = 0xb8360 as *mut u64;
        *hell2 = 0x024c024c02450248;

        let o2 = 0xb8368 as *mut u64;
        *o2 = 0x024f;

        let con = 0xb8680 as *mut u64;
        *con = 0x024e024f0243;

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
