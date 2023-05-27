#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

pub mod core0;
pub mod core1;

use embassy_executor::Executor;
use embassy_executor::_export::StaticCell;
use embassy_rp::multicore::Stack;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;

pub enum LedState {
    On,
    Off,
}

pub static mut CORE1_STACK: Stack<4096> = Stack::new();
pub static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
pub static EXECUTOR1: StaticCell<Executor> = StaticCell::new();
pub static CHANNEL: Channel<CriticalSectionRawMutex, LedState, 1> = Channel::new();
