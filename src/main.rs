#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;
use ufmt_float::uFmt_f32;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());


    let a0 = pins.a0.into_analog_input(&mut adc);

    loop {
        let temp = a0.analog_read(&mut adc);
        let voltage: f32 = (temp as f32 / 1024.0) * 5.0;
        let voltage_str = uFmt_f32::Three(voltage);
        ufmt::uwrite!(&mut serial, "temp: {} ", temp).void_unwrap();
        ufmt::uwrite!(&mut serial, "voltage: {} ", voltage_str).void_unwrap();
        ufmt::uwriteln!(&mut serial, "").void_unwrap();
        arduino_hal::delay_ms(1000);
    }
}