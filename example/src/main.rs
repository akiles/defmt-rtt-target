#![no_std]
#![no_main]

use defmt_rtt_target as _;
use panic_probe as _;

use cortex_m_rt::entry;
use rtt_target::rtt_init;
use defmt::info;
use core::sync::atomic::{AtomicUsize, Ordering};

#[defmt::timestamp]
fn timestamp() -> u64 {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n as u64
}

#[entry]
fn main() -> ! {
    let channels = rtt_init! {
        up: {
            0: {
                size: 1024
                mode: BlockIfFull
                // probe-run autodetects whether defmt is in use based on this channel name
                name: "defmt"
            }
        }
        down: {
            0: {
                size: 16
                name: "Terminal"
            }
        }
    };

    defmt_rtt_target::init(channels.up.0);

    info!("Hello World!");

    let mut input = channels.down.0;
    loop {
        let mut buf = [0u8; 16];
        let n = input.read(&mut buf);
        if n != 0 {
            info!("Read: {:str}", unsafe { core::str::from_utf8_unchecked(&buf[..n])});
        }
    }
}
