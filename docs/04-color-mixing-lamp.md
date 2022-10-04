# 04: Color Mixing Lamp

## Intro

This is where stuff starts getting a bit more complicated.

The [pulse-width modulation](https://www.arduino.cc/en/Tutorial/Foundations/PWM) (PWM) pins on the Arduino UNO let you use a very convenient `analogWrite` function to write an integer corresponding to the duty cycle between 0 and 255 to the pin. We're using a lower abstraction layer and have no such luxury. 

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

We know that the microcontroller unit (MCU) in the Arduino UNO is the ATmega328p, so after figuring out that the pins specified in the task (`~D9`, `~D10` and `~D11`) correspond to the MCU pins `PB1`, `PB2` and `PB3` respectively, we're going to have to find a [datasheet](https://www.google.com/search?q=ATmega328p+datasheet) for it, which I've not hard linked because it's going to get moved anyway so just pick one of the first ones.

After spending an hour trying to make sense of the datasheet (you can skip this step) we can realize that we don't actually know if those timers are being used by the Arduino UNO and we can't faff about with them willy nilly. In researching this we can find the [Secrets of Arduino PWM](https://docs.arduino.cc/tutorials/generic/secrets-of-arduino-pwm) which gives us most of the information we were going to need anyway.

The critical bits of information are:
* `Timer 0` *is* being used internally for `delay()` and `millis()` so we can feel validated. This also means that `Timer 1` and `Timer 2` are fair game for faffing around.
* `~D9`, `~D10` and `~D11` (`PB1`, `PB2` and `PB3`) correspond to timer outputs `OC1A`, `OC1B`, `OC2A`. This means we will be using `Timer 1` and `Timer 2` - the faffable ones, thankfully.
* The UNO `analogWrite()` uses phase-correct PWM on these pins. This isn't mentioned anywhere, but the [docs](https://www.arduino.cc/reference/en/language/functions/analog-io/analogwrite/) for `analogWrite()` tell us that the PWM frequency is 490 Hz for all pins other than 5 and 6, which are 980 Hz. These correspond with the examples (UNO and Duamilanove both have the same 16Hz MCU) in Secrets of Arduino PWM for phase-correct and fast PWM respectively. 
    - *Editor's note (the editor is me):* It totally is mentioned somewhere, namely the document I'm referencing, in plain English.
* The Timer/Counter Control Registers TCCRnA and TCCRnB have control bits to modify how a timer acts; these are just two control registers for the timer and don't have any real relation to the output pins. I.e. `TCCR1A` and `TCCR1B` control things about the behaviour of Timer 1 but *do not* correspond in any way to `OC1A` and `OC1B`. 
    - We should use the Waveform Generation Mode (WGM) bits in these registers to set both of our timers to phase-correct PWM to correspond with the way the default `analogWrite()` would work were we using the Arduino IDE and language.
    - We should use the Compare Match Output bits `COMnA` and `COMnB`, which *do* correspond to `OC1A` and `OC1B` to enable the outputs.
    - We should use the Clock Select (CS) to set the prescaler to 64. This means that each time the 16 Mhz clock of the MCU ticks up 64 times, our clock ticks up once.
    - All of the above might be initialized to these values anyway but let's do it for practice.
* For each timer, the Output Compare Registers `OCRnA` and `OCRnB` *do* correspond with `OCnA` and `OCnB`. Each of these registers contains an 8-bit (0-255) value that controls the actual duty cycle of the output.