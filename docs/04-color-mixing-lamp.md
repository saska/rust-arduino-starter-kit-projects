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

After spending an hour trying to make sense of the datasheet (you can skip this step) we can realize that we don't actually know if those timers are being used by the Arduino UNO and we can't faff about with them willy nilly. In researching this we can find the [Secrets of Arduino PWM](https://docs.arduino.cc/tutorials/generic/secrets-of-arduino-pwm) which gives us all the information we were going to need anyway.

The critical bits of information are:
* Timer 0 *is* being used internally for `delay()` and `millis()` so we can feel validated.
* `~D9`, `~D10` and `~D11` (`PB1`, `PB2` and `PB3`) correspond to timer outputs `OC1A`, `OC1B`, `OC2A`.
* The UNO `analogWrite()` uses phase-correct PWM on these pins. This isn't mentioned anywhere, but the [docs](https://www.arduino.cc/reference/en/language/functions/analog-io/analogwrite/) for `analogWrite()` tell us that the PWM frequency is 490 Hz for all pins other than 5 and 6, which are 980 Hz. These correspond with the UNO examples in Secrets of Arduino PWM for phase-correct and fast PWM respectively.
