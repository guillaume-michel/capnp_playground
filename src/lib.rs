#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;

use defmt_rtt as _; // global logger

use panic_probe as _;

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[global_allocator]
pub static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[alloc_error_handler]
fn oom(layout: Layout) -> ! {
    defmt::error!("Could not allocate memory with layout: {}", layout);
    defmt::info!(
        "Global allocator state: used {} - free: {}",
        ALLOCATOR.used(),
        ALLOCATOR.free()
    );
    cortex_m::asm::udf()
}

// defmt-test 0.3.0 has the limitation that this `#[tests]` attribute can only be used
// once within a crate. the module can be in any file but there can only be at most
// one `#[tests]` module in this library crate
#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use defmt::assert;

    #[test]
    fn it_works() {
        assert!(true)
    }
}
