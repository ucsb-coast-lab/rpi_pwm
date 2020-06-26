### PWM Startup Sequence for the BlueESC 

Starts the the BlueESC from a Raspberry Pi 4B using the Rust `rppal` library.

Output is from PWM0, or Pin 12 (see [pinout.xyz](https://pinout.xyz)). 

Developed via cross-compiling on Ubuntu 18.04 LTS x86_64 for Raspberry Pi OS. Tooling instructions can be found [here](https://github.com/japaric/rust-cross).