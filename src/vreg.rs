use embassy_rp::pac;

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

/// Reduce clock frequency and voltage.
///
/// # Safety
///
/// Temperature, voltage, and other factors may damage the memory.
pub unsafe fn set_under_clock() {
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
        .vreg()
        .modify(|w| w.set_vsel(Vreg::Vsel900MV as u8));
}