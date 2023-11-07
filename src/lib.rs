#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(stable_features, unknown_lints, async_fn_in_trait)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use embassy_rp::pac;

pub mod core1;
pub mod task;

#[derive(Default)]
pub enum Vreg {
    Vsel800MV = 0b0101,
    Vsel850MV = 0b0110,
    Vsel900MV = 0b0111,
    Vsel950MV = 0b1000,
    Vsel1000MV = 0b1001,
    Vsel1050MV = 0b1010,
    #[default]
    Vsel1100MV = 0b1011,
    Vsel1150MV = 0b1100,
    Vsel1200MV = 0b1101,
    Vsel1250MV = 0b1110,
    Vsel1300MV = 0b1111,
}
#[derive(Default)]
pub enum Bod {
    Bod473MV = 0b0000,
    Bod516MV = 0b0001,
    Bod559MV = 0b0010,
    Bod602MV = 0b0011,
    Bod645MV = 0b0100,
    Bod688MV = 0b0101,
    Bod731MV = 0b0110,
    Bod774MV = 0b0111,
    Bod817MV = 0b1000,
    #[default]
    Bod860MV = 0b1001,
    Bod903MV = 0b1010,
    Bod946MV = 0b1011,
    Bod989MV = 0b1100,
    Bod1032MV = 0b1101,
    Bod1075MV = 0b1110,
    Bod1118MV = 0b1111,
}

pub fn set_default_clock() {
    pac::VREG_AND_CHIP_RESET
        .vreg()
        .modify(|w| w.set_vsel(Vreg::default() as u8));
    pac::VREG_AND_CHIP_RESET
        .bod()
        .modify(|w| w.set_vsel(Bod::default() as u8));

    cortex_m::asm::delay(2800); // at least 350 microsecond

    pac::PLL_SYS.fbdiv_int().modify(|w| w.set_fbdiv_int(125));
    pac::PLL_SYS.prim().modify(|w| {
        w.set_postdiv1(6);
        w.set_postdiv2(2);
    });

    pac::PLL_USB.fbdiv_int().modify(|w| w.set_fbdiv_int(120));
    pac::PLL_USB.prim().modify(|w| {
        w.set_postdiv1(6);
        w.set_postdiv2(5);
    });
}

pub fn set_under_clock() {
    pac::PLL_SYS.fbdiv_int().modify(|w| w.set_fbdiv_int(63));
    pac::PLL_SYS.prim().modify(|w| {
        w.set_postdiv1(7);
        w.set_postdiv2(7);
    });

    pac::PLL_USB.fbdiv_int().modify(|w| w.set_fbdiv_int(63));
    pac::PLL_USB.prim().modify(|w| {
        w.set_postdiv1(7);
        w.set_postdiv2(7);
    });

    cortex_m::asm::delay(2800); // at least 350 microsecond

    pac::VREG_AND_CHIP_RESET
        .bod()
        .modify(|w| w.set_vsel(Bod::Bod817MV as u8));
    pac::VREG_AND_CHIP_RESET
        .vreg()
        .modify(|w| w.set_vsel(Vreg::Vsel850MV as u8));
}

pub fn clear_locks() {
    // https://github.com/rp-rs/rp-hal/blob/main/rp2040-hal-macros/src/lib.rs
    for i in 0..32 {
        embassy_rp::pac::SIO.spinlock(i).write_value(1);
    }
}
