#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Executor;
use embassy_executor::_export::StaticCell;
pub use embassy_rp::interrupt::Priority;
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::CORE1;

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

pub fn run_task_on(core1: CORE1, token: impl FnOnce(embassy_executor::Spawner) + Send + 'static) {
    spawn_core1(core1, unsafe { &mut CORE1_STACK }, move || {
        let executor1 = EXECUTOR1.init(Executor::new());
        executor1.run(token);
    });
}
pub fn run_task(token: impl FnOnce(embassy_executor::Spawner) + Send + 'static) -> ! {
    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(token);
}
