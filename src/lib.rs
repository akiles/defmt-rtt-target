//! `defmt` global logger over RTT using `rtt-target`

#![no_std]

use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m::{interrupt, register};
use rtt_target::UpChannel;

static mut CHANNEL: Option<UpChannel> = None;

#[defmt::global_logger]
struct Logger;

impl defmt::Write for Logger {
    fn write(&mut self, bytes: &[u8]) {
        unsafe {
            if let Some(c) = &mut CHANNEL {
                c.write(bytes);
            }
        };
    }
}

pub fn init(channel: UpChannel) {
    unsafe { CHANNEL = Some(channel) }
}

static TAKEN: AtomicBool = AtomicBool::new(false);
static INTERRUPTS_ACTIVE: AtomicBool = AtomicBool::new(false);

unsafe impl defmt::Logger for Logger {
    fn acquire() -> Option<NonNull<dyn defmt::Write>> {
        let primask = register::primask::read();
        interrupt::disable();
        if !TAKEN.load(Ordering::Relaxed) {
            // no need for CAS because interrupts are disabled
            TAKEN.store(true, Ordering::Relaxed);

            INTERRUPTS_ACTIVE.store(primask.is_active(), Ordering::Relaxed);

            Some(NonNull::from(&Logger as &dyn defmt::Write))
        } else {
            if primask.is_active() {
                // re-enable interrupts
                unsafe { interrupt::enable() }
            }
            None
        }
    }

    unsafe fn release(_: NonNull<dyn defmt::Write>) {
        TAKEN.store(false, Ordering::Relaxed);
        if INTERRUPTS_ACTIVE.load(Ordering::Relaxed) {
            // re-enable interrupts
            interrupt::enable()
        }
    }
}
