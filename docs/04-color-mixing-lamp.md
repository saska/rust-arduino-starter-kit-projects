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
    - We should use the Compare Match Output bits `COMnA` and `COMnB`, which *do* correspond to `OC1A` and `OC1B` to enable the outputs. We also learn that the normal, non-inverted phase-correct PWM turns off the output on the way up and on on the way down.
    - We should use the Clock Select (CS) to set the prescaler to 64. This means that each time the 16 Mhz clock of the MCU ticks 64 times, our clock ticks up once.
    - All of the above might be initialized to these values anyway but let's do it to make sure and learn.
* For each timer, the Output Compare Registers `OCRnA` and `OCRnB` *do* correspond with `OCnA` and `OCnB`. Each of these registers contains an 8-bit (0-255) value that controls the actual duty cycle of the output.

At this point, we do need to consult the datasheet to know which bits to flip where since the examples are a bit incomplete. `Timer 1` and `Timer 2` also have a different amount of WGM bits, with `Timer 1` being seemingly more configurable; we want them both to work identically.

## Setting the TCCRnX register bits

In summary, we want to set (for both timers):
* (8-bit) phase-correct PWM in the WGM bits
* Non-inverted phase-correction PWM output in the COM bits
* The prescaler to 64 in the CS bits.

I searched the datasheet and made this handy-dandy table to see what bit needs to be set to what. I'll have the spots I found them in on the datasheet at the bottom of the document. We also have to take into account that the WGM bits are split between the two registers on both timers, because of course they are. 

### Register A (TCCRnA)

|           | WGM1 | WGM0 | COMnA1 | COMnA0 | COMnB1 | COMnB0 |
| --------- | ---- | ---- | ------ | ------ | ------ | ------ |
| `Timer 1` | 0    | 1    | 1      | 0      | 1      | 0      |
| `Timer 2` | 0    | 1    | 1      | 0      | n/a    | n/a    |

### Register B (TCCRnB)

|           | WGM3 | WGM2 | CSn2 | CSn1 | CSn0 |
| --------- | ---- | ---- | ---- | ---- | ---- |
| `Timer 1` | 0    | 0    | 0    | 1    | 1    |
| `Timer 2` | n/a  | 0    | 1    | 0    | 0    |

Note that the `WGM3` bit is not applicable on Timer 2 because it doesn't exist, while `COMnB1` and `COMnB0` because we don't need them since we're not using that output. 


## Putting all of this into code

The `avr-hal` thankfully gives us convenient access to these registers and bits by name so we don't need to know their exact addresses. They sometimes also give us convenience functions so we don't need to know the exact bits either, just the behaviour we want - these function names were kind of short and I really needed to look at the datasheet anyway to understand what I'm actually doing though so they ended up being of little use. Inexperience, probably.

Anyway, here's a bit of code that just takes the two tables above and just sends the stuff that's in there to the places it's supposed to be sent to.

```rust
    let dp = arduino_hal::Peripherals::take().unwrap();
    let tc1 = dp.TC1;
    tc1.tccr1a.write(|w| w
        .wgm1().bits(0b01)
        .com1a().bits(0b10)
        .com1b().bits(0b10)
    );
    tc1.tccr1b.write(|w| w
        .wgm1().bits(0b00)
        .cs1().bits(0b011)
    );

    let tc2 = dp.TC2;
    tc2.tccr2a.write(|w| w
        .wgm2().bits(0b01)
        .com2a().bits(0b10)
    );
    tc2.tccr2b.write(|w| w
        .wgm22().bit(false) // single bit -> different name and pattern
        .cs2().bits(0b100)
    );
```

We can then control the duty cycles (128 in this example) with

```rust
tc1.ocr1a.write(|w| unsafe { w.bits(128u16) });
tc1.ocr1b.write(|w| unsafe { w.bits(128u16) });
tc2.ocr2a.write(|w| unsafe { w.bits(128u8) }); // smaller register
```

## Datasheet source tables
* [TCCR1A bits](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#G1189341)
* [TCCR1B bits](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#G1190212)
* [TCCR1x WGM](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#G1189825)
* [TCCR1x COM](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#G1189695)
* [TCCR1x CS](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#G1190212) (direct link not available, table 15-6)
* [TCCR2A bits](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#G1192535)
* [TCCR2B bits](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#G1189341)
* [TCCR2x WGM](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#G1192935)
* [TCCR2x COM](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#G1189825)
* [TCCR2x CS](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#G1193246)