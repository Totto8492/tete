#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Executor;
use embassy_executor::_export::StaticCell;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::PIN_5;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};
#[allow(unused)]
use rtt_target::rprintln as info;
#[allow(unused)]
use rtt_target::{rdbg, rprint, rprintln, rtt_init_print};
use {panic_rtt_target as _, rtt_target as _};

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();
static CHANNEL: Channel<CriticalSectionRawMutex, LedState, 1> = Channel::new();

enum LedState {
    On,
    Off,
}

#[embassy_executor::entry]
fn main() -> ! {
    rtt_init_print!();
    let p = embassy_rp::init(Default::default());
    let led = Output::new(p.PIN_5, Level::Low);

    spawn_core1(p.CORE1, unsafe { &mut CORE1_STACK }, move || {
        let executor1 = EXECUTOR1.init(Executor::new());
        executor1.run(|spawner| (spawner.spawn(core1_task(led))).unwrap());
    });

    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| (spawner.spawn(core0_task())).unwrap());
}

#[embassy_executor::task]
async fn core0_task() {
    info!("Hello from core 0");
    loop {
        CHANNEL.send(LedState::On).await;
        Timer::after(Duration::from_millis(100)).await;
        CHANNEL.send(LedState::Off).await;
        Timer::after(Duration::from_millis(400)).await;
    }
}

#[embassy_executor::task]
async fn core1_task(mut led: Output<'static, PIN_5>) {
    info!("Hello from core 1");
    loop {
        match CHANNEL.recv().await {
            LedState::On => led.set_high(),
            LedState::Off => led.set_low(),
        }
    }
}
