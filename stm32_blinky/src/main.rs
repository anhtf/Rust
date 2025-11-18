#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();

    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the
    // corresponding HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store
    // the frozen frequencies in `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split();

    // Configure GPIO pin PC13 as a push-pull output. The `crh` register is passed to the function
    // because PC13 is in the high part of the GPIO port.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // Create a delay abstraction based on the system timer (SysTick)
    let mut delay = cp.SYST.delay(&clocks);

    // Blink the LED in a loop
    loop {
        // Turn the LED on by setting the pin low (the LED on the Blue Pill is active low)
        led.set_low();
        delay.delay_ms(1000_u16);

        // Turn the LED off by setting the pin high
        led.set_high();
        delay.delay_ms(1000_u16);
    }
}