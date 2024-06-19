# Rust Decimal Clock for the [M5 Dial](https://shop.m5stack.com/products/m5stack-dial-esp32-s3-smart-rotary-knob-w-1-28-round-touch-screen)

This is my first foray into Rust & I'm attempting to write a clock for the [M5 Dial](https://shop.m5stack.com/products/m5stack-dial-esp32-s3-smart-rotary-knob-w-1-28-round-touch-screen) where the day is divided into ten hours of 100 minutes and 100 seconds. The time is, in essence, the percentage of the day that has passed.

I've not been able to find any examples of controlling a M5 Dial using Rust, and, as aforementioned, this is my first Rust program, so I make no claims as to the code quality. 😼

## Running

1. Install Rust using [rustup](https://rustup.rs).
2. `cargo build`
3. `cargo run` # with the M5 Dial connected via USB
