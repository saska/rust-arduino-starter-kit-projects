#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;
use ufmt_float::uFmt_f32;

fn get_temp_TMP36(sensor_value: u16) -> f32 {
    let voltage: f32 = (sensor_value as f32 / 1024.0) * 5.0;
    return (voltage - 0.5) * 100.0;
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut d2 = pins.d2.into_output();
    let mut d3 = pins.d3.into_output();
    let mut d4 = pins.d4.into_output();

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let a0 = pins.a0.into_analog_input(&mut adc);
    let baseline_temp = get_temp_TMP36(a0.analog_read(&mut adc));

    loop {
        d2.set_low();
        d3.set_low();
        d4.set_low();
        let temp = get_temp_TMP36(a0.analog_read(&mut adc));
        let diff = temp - baseline_temp;
        if diff > 2.0 {
            d2.set_high();
        }
        if diff > 4.0 {
            d3.set_high();
        }
        if diff > 6.0 {
            d4.set_high();
        }
        let temp_str = uFmt_f32::Three(temp);
        ufmt::uwrite!(&mut serial, "temp (C): {} ", temp_str).void_unwrap();
        ufmt::uwriteln!(&mut serial, "").void_unwrap();
        arduino_hal::delay_ms(1);
    }
}