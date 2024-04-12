#![no_std]
#![no_main]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use embassy_executor::Executor;
use embassy_rp::config::Config;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::Timer;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use static_cell::StaticCell;

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();
static CHANNEL: Channel<CriticalSectionRawMutex, LedState, 1> = Channel::new();

enum LedState {
    On,
    Off,
}

#[cortex_m_rt::entry]
fn main() -> ! {
    for i in 0..32 {
        embassy_rp::pac::SIO.spinlock(i).write_value(1);
    }

    rtt_init_print!();

    let p = embassy_rp::init(Config::default());
    let led = Output::new(p.PIN_0, Level::Low);

    spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(CORE1_STACK) },
        move || {
            let executor1 = EXECUTOR1.init(Executor::new());
            executor1.run(|spawner| {
                spawner.must_spawn(core1_task(led));
            });
        },
    );

    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| {
        spawner.must_spawn(core0_task());
    });
}

#[embassy_executor::task]
async fn core0_task() {
    rprintln!("Hello from core 0");
    loop {
        CHANNEL.send(LedState::On).await;
        Timer::after_millis(500).await;
        CHANNEL.send(LedState::Off).await;
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::task]
async fn core1_task(mut led: Output<'static>) {
    rprintln!("Hello from core 1");
    loop {
        match CHANNEL.receive().await {
            LedState::On => led.set_high(),
            LedState::Off => led.set_low(),
        }
    }
}
