use stm32f1xx_hal::{
    pac,
    prelude::*,
    timer::{Channel, Tim2NoRemap},
};

// Gradual darkening, gradual brightening
pub fn breathing_light() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();

    let clock = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = cp.SYST.delay(&clock);

    let mut afio = p.AFIO.constrain();
    let mut gpioa = p.GPIOA.split();

    let led = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);

    let mut pwm = p
        .TIM2
        .pwm_hz::<Tim2NoRemap, _, _>(led, &mut afio.mapr, 1.kHz(), &clock);

    pwm.enable(Channel::C1);

    let max: u16 = pwm.get_max_duty() / 5;
    let one_percent = max / 100;

    pwm.set_duty(Channel::C1, 0);

    let mut current_duty: u16 = 0;
    let mut is_up = true;

    loop {
        pwm.set_duty(Channel::C1, current_duty);

        if is_up {
            let val = current_duty + one_percent;
            if val < max {
                current_duty = val;
            } else {
                current_duty = max;
                is_up = false;
            }
        } else {
            let val = current_duty - one_percent;
            if val > 0 {
                current_duty = val
            } else {
                current_duty = 0;
                is_up = true;
            }
        }

        delay.delay_ms(8u8);
    }
}
