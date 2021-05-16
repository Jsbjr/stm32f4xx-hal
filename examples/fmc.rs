//! Simple example on how to use the fmc module
//! tested on stm32f469i-discovery board

#![no_main]
#![no_std]

use core::slice;
use core::mem;

use cortex_m;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{prelude::*, stm32};
use stm32f4xx_hal::delay::Delay;
use stm32_fmc::devices::mt48lc4m32b2_6;

use stm32f4xx_hal::fmc::FmcExt;
use stm32f4xx_hal::gpio::Speed;

/// Configre a pin for the FMC controller
macro_rules! fmc_pins {
    ($($pin:expr),*) => {
        (
            $(
                $pin.into_push_pull_output()
                    .set_speed(Speed::VeryHigh)
                    .into_alternate_af12()
                    .internal_pull_up(true)
            ),*
        )
    };
}

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), cortex_m::Peripherals::take()) {
        let rcc = p.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(25.mhz())
            .require_pll48clk()
            .sysclk(84.mhz())
            .hclk(84.mhz())
            .pclk1(42.mhz())
            .pclk2(84.mhz())
            .freeze();
        let mut delay = Delay::new(cp.SYST, clocks);

        // Get IO
        let gpioc = p.GPIOC.split();
        let gpiod = p.GPIOD.split();
        let gpioe = p.GPIOE.split();
        let gpiof = p.GPIOF.split();
        let gpiog = p.GPIOG.split();
        let gpioh = p.GPIOH.split();
        let gpioi = p.GPIOI.split();

        let sdram_size = 32 * 1024 * 1024;
        let sdram_pins = fmc_pins!(
            // A0-A11 pins
            gpiof.pf0, gpiof.pf1, gpiof.pf2, gpiof.pf3, gpiof.pf4, gpiof.pf5, gpiof.pf12,
            gpiof.pf13, gpiof.pf14, gpiof.pf15, gpiog.pg0, gpiog.pg1, gpiog.pg4, gpiog.pg5,
            // B0-B13 pins
            gpiod.pd14, gpiod.pd15, gpiod.pd0, gpiod.pd1, gpioe.pe7, gpioe.pe8, gpioe.pe9,
            gpioe.pe10, gpioe.pe11, gpioe.pe12, gpioe.pe13, gpioe.pe14, gpioe.pe15, gpiod.pd8,
            gpiod.pd9, gpiod.pd10, gpioh.ph8, gpioh.ph9, gpioh.ph10, gpioh.ph11, gpioh.ph12,
            gpioh.ph13, gpioh.ph14, gpioh.ph15, gpioi.pi0, gpioi.pi1, gpioi.pi2, gpioi.pi3,
            gpioi.pi6, gpioi.pi7, gpioi.pi9, gpioi.pi10, 

            gpioe.pe0, gpioe.pe1, gpioi.pi4, gpioi.pi5, // NL0-NL3
            // SDCKE, SDCLK, SDNCAS, SDNE, SDRAS, SDNWE
            gpioh.ph2, gpiog.pg8, gpiog.pg15, gpioh.ph3, gpiof.pf11, gpioc.pc0
        );
        let mut sdram_chip = p.FMC.sdram(sdram_pins, mt48lc4m32b2_6::Mt48lc4m32b2 {}, &clocks);

        // Initialise the controller and the SDRAM
        let ram_ptr : *mut u32 = sdram_chip.init(&mut delay);

        // Get 16-bit words
        let ram_ptr = ram_ptr as *mut u16;

        // Convert raw pointer to slice
        let ram_slice = unsafe {slice::from_raw_parts_mut(ram_ptr, sdram_size)};

        // Return a 4-word slice
        let size = mem::size_of::<u16>() * 4usize;
        let mut chunks = ram_slice.chunks_exact_mut(size);
        let ram_slice = chunks.next().unwrap();

        // Use memory in SDRAM
        
        ram_slice[0] = 1u16;
        ram_slice[1] = 2;
        ram_slice[2] = 3;
        ram_slice[3] = 4;
        ram_slice[4] = 5;

        assert_eq!(ram_slice[0], 1);
        assert_eq!(ram_slice[1], 2);
        assert_eq!(ram_slice[2], 3);
        assert_eq!(ram_slice[3], 4);
        assert_eq!(ram_slice[4], 5);

        loop {
            cortex_m::asm::nop()
        }
    }
    loop {
        cortex_m::asm::nop()
    }
}
