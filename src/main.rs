#![feature(extern_prelude)]
#![feature(global_asm)]
#![feature(panic_implementation)]
#![no_main]
#![no_std]

const LED_ADDRESS: u32 = 0x0300_0000;

// The reset handler
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    r0::zero_bss(&mut _sbss, &mut _ebss);
    r0::init_data(&mut _sdata, &mut _edata, &_sidata);
    main()
}

fn main() -> ! {
    loop {
        for i in 0..256 {
            unsafe { core::ptr::write_volatile(LED_ADDRESS as *mut u32, i); }
        }
    }
}

extern "C" {
    // Boundaries of the .bss section
    static mut _ebss: u32;
    static mut _sbss: u32;

    // Boundaries of the .data section
    static mut _edata: u32;
    static mut _sdata: u32;

    // Initial values of the .data section (stored in Flash)
    static _sidata: u32;
}

#[panic_implementation]
#[no_mangle]
pub fn panic_fmt(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Make sure there is an abort when linking
#[cfg(target_arch = "riscv32")]
global_asm!(r#"
call Reset
.globl abort
abort:
  jal zero, abort
"#);
