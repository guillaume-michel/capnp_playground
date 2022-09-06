#![deny(unsafe_code)]
#![no_std]
#![no_main]

use capnp_playground as _; // global logger + panicking-behavior + memory layout

use panic_probe as _;

use capnp_playground::ALLOCATOR;

pub mod commands_capnp {
    include!(concat!(env!("OUT_DIR"), "/capnp/command_capnp.rs"));
}

use capnp::serialize::compute_serialized_size_in_words;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 4096;
        static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        #[allow(unsafe_code)]
        unsafe {
            ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE)
        }
    }

    use commands_capnp::distance_response;

    let mut message = ::capnp::message::Builder::new_default();

    {
        let mut cmd: distance_response::Builder = message.init_root::<distance_response::Builder>();

        cmd.set_distance(2.0_f32);
    }
    let s = compute_serialized_size_in_words(&message);

    defmt::info!("CapNProto unpacked message size: {} bytes", s * 8);

    let mut buf: [u8; 32] = [0xff; 32];

    capnp::serialize::write_message(&mut buf.as_mut_slice(), &message).unwrap();

    defmt::info!("unpacked capnp buf: {:X}", buf);

    let mut buf: [u8; 32] = [0xff; 32];

    capnp::serialize_packed::write_message(&mut buf.as_mut_slice(), &message).unwrap();

    defmt::info!("packed capnp buf: {:X}", buf);

    defmt::info!("The End!");

    capnp_playground::exit()
}
