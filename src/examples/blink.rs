use stm32f1xx_hal::{pac, prelude::*};

// blink led on pa0
pub fn blink() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();

    let clock = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = cp.SYST.delay(&clock);

    let mut gpioa = p.GPIOA.split();

    let mut led = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);

    loop {
        led.set_high();
        delay.delay_ms(1000u16);
        led.set_low();
        delay.delay_ms(1000u16);
    }
}
