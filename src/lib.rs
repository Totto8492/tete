#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::_export::StaticCell;
use embassy_executor::{Executor, InterruptExecutor, SendSpawner};
use embassy_rp::interrupt::{self, InterruptExt, Priority};
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
unsafe fn SWI_IRQ_3() {
    EXECUTOR_IDLE.on_interrupt()
}

#[embassy_rp::interrupt]
unsafe fn SWI_IRQ_2() {
    EXECUTOR_LOW.on_interrupt()
}

#[embassy_rp::interrupt]
unsafe fn SWI_IRQ_1() {
    EXECUTOR_MEDIUM.on_interrupt()
}

#[embassy_rp::interrupt]
unsafe fn SWI_IRQ_0() {
    EXECUTOR_HIGH.on_interrupt()
}

pub struct IrqPriority {
    irq: Interrupt,
    executor: &'static InterruptExecutor,
    prio: Priority,
}

impl IrqPriority {
    pub fn idle() -> Self {
        Self {
            irq: Interrupt::SWI_IRQ_3,
            executor: &EXECUTOR_IDLE,
            prio: Priority::P3,
        }
    }

    pub fn low() -> Self {
        Self {
            irq: Interrupt::SWI_IRQ_2,
            executor: &EXECUTOR_LOW,
            prio: Priority::P2,
        }
    }

    pub fn medium() -> Self {
        Self {
            irq: Interrupt::SWI_IRQ_1,
            executor: &EXECUTOR_MEDIUM,
            prio: Priority::P1,
        }
    }

    pub fn high() -> Self {
        Self {
            irq: Interrupt::SWI_IRQ_0,
            executor: &EXECUTOR_HIGH,
            prio: Priority::P0,
        }
    }
}

pub fn run_preemptive_task(prio: IrqPriority, token: impl FnOnce(SendSpawner) + Send + 'static) {
    prio.irq.set_priority(prio.prio);
    let spawner = prio.executor.start(prio.irq);
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
