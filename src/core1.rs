use embassy_rp::gpio::Output;
use embassy_rp::peripherals::PIN_5;
use rtt_target::rprintln as info;

use super::{LedState, CHANNEL};

#[embassy_executor::task]
pub async fn task(mut led: Output<'static, PIN_5>) {
    info!("Hello from core 1");
    loop {
        match CHANNEL.recv().await {
            LedState::On => led.set_high(),
            LedState::Off => led.set_low(),
        }
    }
}
