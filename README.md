# Arduino starter kit projects in Rust

Projects from the [arduino starter kit](https://store.arduino.cc/products/arduino-starter-kit-multi-language) in rust.

## Running

Follow dependency installation instructions in [avr-hal](https://github.com/Rahix/avr-hal). Then you can run each project with `cargo run --bin <project-name>` (you can find the projects in `src/bin`). 

## Notes

- Weird linker error without `"no-builtins": true` in `avr-atmega328p.json` when using `ufmt_float` crate.