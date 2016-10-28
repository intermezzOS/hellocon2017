#![feature(lang_items)]
#![no_std]

extern crate rlibc;

#[no_mangle]
pub extern fn kmain() -> ! {
    let char_bytes = b" Hello RustFest! ";

    let color_byte = 0x9f;


    let mut message = [color_byte; 24];
    for (i, char_byte) in char_bytes.into_iter().enumerate() {
      message[i * 2] = *char_byte;
    }

    let buffer_ptr = (0xb8000  +1988) as *mut _;

    unsafe { *buffer_ptr = message };

    loop{}

}

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
extern fn rust_begin_panic() -> ! {
    loop {}
}
