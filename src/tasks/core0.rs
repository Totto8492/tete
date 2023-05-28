use embassy_time::{Duration, Instant, Timer, TICK_HZ};
use rtt_target::rprintln;

use crate::tasks::state::*;

#[embassy_executor::task]
pub async fn task() {
    rprintln!("Hello from core 0");
    loop {
        CHANNEL.send(LedState::On).await;
        Timer::after(Duration::from_millis(100)).await;
        CHANNEL.send(LedState::Off).await;
        Timer::after(Duration::from_millis(400)).await;
    }
}

#[embassy_executor::task]
pub async fn run_high() {
    loop {
        let start = Instant::now();
        rprintln!("    [high] Starting long computation");

        cortex_m::asm::delay(12_000_000);

        let end = Instant::now();
        let ms = end.duration_since(start).as_ticks() * 1000 / TICK_HZ;
        rprintln!("    [high] done in {} ms", ms);

        Timer::after(Duration::from_ticks(753421)).await;
    }
}

#[embassy_executor::task]
pub async fn run_med() {
    loop {
        let start = Instant::now();
        rprintln!("    [med] Starting long computation");

        // Spin-wait to simulate a long CPU computation
        cortex_m::asm::delay(125_000_000); // ~1 second

        let end = Instant::now();
        let ms = end.duration_since(start).as_ticks() * 1000 / TICK_HZ;
        rprintln!("    [med] done in {} ms", ms);

        Timer::after(Duration::from_ticks(53421)).await;
    }
}
