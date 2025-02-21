use stm32f1xx_hal::{gpio::PinState, pac, prelude::*};

const SEGMENTS: [u8; 10] = [
    0b1111110, 0b0110000, 0b1101101, 0b1111001, 0b0110011, 0b1011011, 0b1011111, 0b1110000,
    0b1111111, 0b1111011,
];

pub fn digital_tube() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();

    let clock = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = cp.SYST.delay(&clock);

    let mut gpioa = p.GPIOA.split();

    let mut pin_1 = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    let mut pin_2 = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);
    let mut pin_3 = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    let mut pin_4 = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);

    let mut pin_a = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    let mut pin_b = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let mut pin_c = gpioa.pa11.into_push_pull_output(&mut gpioa.crh);
    let mut pin_d = gpioa.pa9.into_push_pull_output(&mut gpioa.crh);
    let mut pin_e = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);
    let mut pin_f = gpioa.pa3.into_push_pull_output(&mut gpioa.crl);
    let mut pin_g = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
    let mut pin_dp = gpioa.pa10.into_push_pull_output(&mut gpioa.crh);

    let pin_state_from_bool = |b: bool| -> PinState {
        if b {
            PinState::Low
        } else {
            PinState::High
        }
    };

    let mut display_number = |num: i8| {
        let pattern = if num == -1 {
            0
        } else {
            SEGMENTS[num.clamp(0, 9) as usize]
        };

        pin_a.set_state(pin_state_from_bool(pattern & (1 << 6) != 0));
        pin_b.set_state(pin_state_from_bool(pattern & (1 << 5) != 0));
        pin_c.set_state(pin_state_from_bool(pattern & (1 << 4) != 0));
        pin_d.set_state(pin_state_from_bool(pattern & (1 << 3) != 0));
        pin_e.set_state(pin_state_from_bool(pattern & (1 << 2) != 0));
        pin_f.set_state(pin_state_from_bool(pattern & (1 << 1) != 0));
        pin_g.set_state(pin_state_from_bool(pattern & (1 << 0) != 0));
    };
    let mut display = |_dig: u8, num: i8, dp: bool| {
        let dig = _dig.clamp(1, 4);

        display_number(-1);
        pin_dp.set_high();
        match dig {
            1 => {
                pin_1.set_high();
                pin_2.set_low();
                pin_3.set_low();
                pin_4.set_low();
            }
            2 => {
                pin_1.set_low();
                pin_2.set_high();
                pin_3.set_low();
                pin_4.set_low();
            }
            3 => {
                pin_1.set_low();
                pin_2.set_low();
                pin_3.set_high();
                pin_4.set_low();
            }
            4 => {
                pin_1.set_low();
                pin_2.set_low();
                pin_3.set_low();
                pin_4.set_high();
            }
            _ => unreachable!(),
        };

        display_number(num);
        pin_dp.set_state(if dp { PinState::Low } else { PinState::High });
    };

    loop {
        display(1, 1, false);
        delay.delay_us(100u8);

        display(2, 2, true);
        delay.delay_us(100u8);

        display(3, 3, false);
        delay.delay_us(100u8);

        display(4, 4, false);
        delay.delay_us(100u8);
    }
}
