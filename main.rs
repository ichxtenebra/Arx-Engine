#![no_std]
#![no_main]

extern crate arx_engine;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    arx_engine::ArxEngine::ignite(0, 0, 800, 600)
}