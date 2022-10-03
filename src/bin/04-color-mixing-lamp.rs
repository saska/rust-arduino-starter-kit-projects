#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // stuff is wired weird in the book 
    // so replicating that here
    let mut r = pins.d10.into_output();
    let mut g = pins.d9.into_output();
    let mut b = pins.d11.into_output();

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let rs = pins.a0.into_analog_input(&mut adc);
    let gs = pins.a1.into_analog_input(&mut adc);
    let bs = pins.a2.into_analog_input(&mut adc);

    let tc1 = dp.TC1;
    tc1.tccr1a.write(|w| w.wgm1().bits(0b01).com1a().match_clear());
    tc1.tccr1b.write(|w| w.wgm1().bits(0b01).cs1().prescale_64());

    loop {
        // let red_sensor_val = rs.analog_read(&mut adc);
        // arduino_hal::delay_ms(5);
        // let green_sensor_val = gs.analog_read(&mut adc);
        // arduino_hal::delay_ms(5);
        // let blue_sensor_val = bs.analog_read(&mut adc);
        // ufmt::uwrite!(
        //     &mut serial, 
        //     "red sensor: {} \t green sensor: {} \t blue sensor: {} \t\n",
        //     red_sensor_val,
        //     green_sensor_val,
        //     blue_sensor_val,
        // ).void_unwrap();
        // r.set_high();
        // g.set_low();
        // b.set_high();
        // arduino_hal::delay_ms(1);
        for duty in 0u8..=255u8 {
            ufmt::uwriteln!(&mut serial, "Duty: {}", duty).void_unwrap();
            tc1.ocr1a.write(|w| unsafe { w.bits(duty as u16) });
            arduino_hal::delay_ms(20);
        }
    }
}