#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // need to set these into outputs so they output stuff
    pins.d9.into_output(); // green
    pins.d10.into_output(); // red
    pins.d11.into_output(); // blue

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let rs = pins.a0.into_analog_input(&mut adc);
    let gs = pins.a1.into_analog_input(&mut adc);
    let bs = pins.a2.into_analog_input(&mut adc);

    let tc1 = dp.TC1;
    tc1.tccr1a
        .write(|w| w.wgm1().bits(0b01).com1a().bits(0b10).com1b().bits(0b10));
    tc1.tccr1b.write(|w| w.wgm1().bits(0b00).cs1().bits(0b011));

    let tc2 = dp.TC2;
    tc2.tccr2a.write(|w| w.wgm2().bits(0b01).com2a().bits(0b10));
    tc2.tccr2b.write(|w| {
        w.wgm22()
            .bit(false) // single bit -> different name and pattern
            .cs2()
            .bits(0b100)
    });

    let mut r = &tc1.ocr1b; // pin ~D10
    let mut g = &tc1.ocr1a; // pin ~D9
    let mut b = &tc2.ocr2a; // pin ~D11

    loop {
        let red_sensor_val = rs.analog_read(&mut adc);
        arduino_hal::delay_ms(5);
        let green_sensor_val = gs.analog_read(&mut adc);
        arduino_hal::delay_ms(5);
        let blue_sensor_val = bs.analog_read(&mut adc);
        arduino_hal::delay_ms(5);
        ufmt::uwrite!(
            &mut serial,
            "red sensor: {} \t green sensor: {} \t blue sensor: {} \t\n",
            red_sensor_val,
            green_sensor_val,
            blue_sensor_val,
        )
        .void_unwrap();

        let red_val = red_sensor_val / 4;
        let green_val = green_sensor_val / 4;
        let blue_val = blue_sensor_val / 4;
        ufmt::uwrite!(
            &mut serial,
            "red value: {} \t green value: {} \t blue value: {} \t\n",
            red_val,
            green_val,
            blue_val,
        )
        .void_unwrap();

        r.write(|w| unsafe { w.bits(red_val) });
        g.write(|w| unsafe { w.bits(green_val) });
        b.write(|w| unsafe { w.bits(blue_val.try_into().unwrap()) });
    }
}
