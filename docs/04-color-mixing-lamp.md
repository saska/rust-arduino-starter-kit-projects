# 04: Color Mixing Lamp

## Intro

This is where stuff starts getting a bit more complicated.

The [PWM](https://www.arduino.cc/en/Tutorial/Foundations/PWM) pins on the Arduino UNO let you use a very convenient `analogWrite` function to write an integer corresponding to the duty cycle between 0 and 255 to the pin. We're using a lower abstraction layer and have no such luxury. 

There's always the option of being lazy and implementing the duty cycle manually.

```rust
loop {
    pin.set_high();
    arduino_hal::delay_us(100);
    pin.set_low();
    arduino_hal::delay_us(100);
}
```

The above (approximately) corresponds to a 50% duty cycle. Laziness, I've learned, is often good when doing things and bad when learning things, so on we go with the hard way.

## Using the registers directly

![arduino uno rev3 pinout](img/pinout.png "ARDUINO UNO REV3 PINOUT")

We know that the microcontroller in the Arduino UNO is the ATmega328p, so after figuring out that the pins specified in the task (`~D9`, `~D10` and `~D11`) correspond to the MCU pins `PB1`, `PB2` and `PB3` respectively, we're going to have to find a [datasheet](https://www.google.com/search?q=ATmega328p+datasheet) for it, which I've not hard linked because it's going to get moved anyway so just pick one of the first ones.

The datasheet will tell us ([here](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#G1183241) if it won't move) that the timer output pins for `PB1`, `PB2` and `PB3` are `OC1A`, `OC1B` and `OC2A` respectively.