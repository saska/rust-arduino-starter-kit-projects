# Arduino starter kit projects in Rust

Projects from the [arduino starter kit](https://store.arduino.cc/products/arduino-starter-kit-multi-language) in rust.

I try to document most of the stuff I did *after* I read the corresponding project description, so a lot of things might not make sense if you don't have the book. I'm under the impression that the [license](https://creativecommons.org/licenses/by-nc-sa/3.0/) for the book is quite permissible so you can probably find a pdf and that's probably okay. #notlegaladvice

Disclaimer: I don't know embedded or rust as of the beginning this project so stuff might be wack. I welcome any comments / feedback on how to do it better.

## Glossary

If you run into an abbreviation you have no idea about, please check [The Embedded Rust Book glossary](https://doc.rust-lang.org/beta/embedded-book/appendix/glossary.html?highlight=pac#pac). I also try to write out the whole term when I first use it in a file, so `ctrl+f` might also help.

## Running

Follow dependency installation instructions in [avr-hal](https://github.com/Rahix/avr-hal). Then you can run each project with `cargo run --bin <project-name>` (you can find the projects in `src/bin`). 

## Notes

- Weird linker error without `"no-builtins": true` in `avr-atmega328p.json` when using `ufmt_float` crate.

## Shoutouts

* To Arduino for licensing [the contents of their documentation](https://github.com/arduino/docs-content) with a [permissible license](https://github.com/arduino/docs-content/blob/main/LICENSE.md) so I could yoink some of their helpful pictures and stuff. 
