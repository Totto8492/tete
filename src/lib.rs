#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

pub mod core0;
pub mod core1;
mod state;
pub mod timeout;
use embassy_executor::Executor;
use embassy_executor::_export::StaticCell;
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use state::LedState;

pub static mut CORE1_STACK: Stack<4096> = Stack::new();
pub static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
pub static EXECUTOR1: StaticCell<Executor> = StaticCell::new();
pub static CHANNEL: Channel<CriticalSectionRawMutex, LedState, 1> = Channel::new();

pub fn run_task_with(
    core1: embassy_rp::peripherals::CORE1,
    token: impl FnOnce(embassy_executor::Spawner) + Send + 'static,
) {
    spawn_core1(core1, unsafe { &mut CORE1_STACK }, move || {
        let executor1 = EXECUTOR1.init(Executor::new());
        executor1.run(token);
    });
}
pub fn run_task(token: impl FnOnce(embassy_executor::Spawner) + Send + 'static) -> ! {
    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(token);
}
