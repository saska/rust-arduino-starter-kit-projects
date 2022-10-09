#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let button = pins.d2;
    let mut green = pins.d3.into_output();
    let mut red1 = pins.d4.into_output();
    let mut red2 = pins.d5.into_output();

    loop {
        if button.is_low() {
            red1.set_low();
            red2.set_low();
            green.set_high();
        } else {
            red1.set_low();
            red2.set_high();
            green.set_low();
            arduino_hal::delay_ms(250);
            red1.set_high();
            red2.set_low();
            arduino_hal::delay_ms(250);
        }
    }
}
