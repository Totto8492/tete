#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::_export::StaticCell;
use embassy_executor::{Executor, InterruptExecutor, SendSpawner};
pub use embassy_rp::interrupt::Priority;
use embassy_rp::interrupt::{self, InterruptExt};
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::pac::Interrupt;
use embassy_rp::peripherals::CORE1;

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();
static EXECUTOR_IDLE: InterruptExecutor = InterruptExecutor::new();
static EXECUTOR_LOW: InterruptExecutor = InterruptExecutor::new();
static EXECUTOR_MEDIUM: InterruptExecutor = InterruptExecutor::new();
static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();

#[embassy_rp::interrupt]
unsafe fn SWI_IRQ_0() {
    EXECUTOR_IDLE.on_interrupt()
}

#[embassy_rp::interrupt]
unsafe fn SWI_IRQ_1() {
    EXECUTOR_LOW.on_interrupt()
}

#[embassy_rp::interrupt]
unsafe fn SWI_IRQ_2() {
    EXECUTOR_MEDIUM.on_interrupt()
}

#[embassy_rp::interrupt]
unsafe fn SWI_IRQ_3() {
    EXECUTOR_HIGH.on_interrupt()
}

pub fn run_preemptive_task(prio: Priority, token: impl FnOnce(SendSpawner) + Send + 'static) {
    let (irq, executor) = match prio {
        Priority::P0 => (Interrupt::SWI_IRQ_0, &EXECUTOR_IDLE),
        Priority::P1 => (Interrupt::SWI_IRQ_1, &EXECUTOR_LOW),
        Priority::P2 => (Interrupt::SWI_IRQ_2, &EXECUTOR_MEDIUM),
        Priority::P3 => (Interrupt::SWI_IRQ_3, &EXECUTOR_HIGH),
    };
    irq.set_priority(prio);
    let spawner = executor.start(irq);
    token(spawner);
}

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
