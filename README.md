# rgb-controller

A small rgb controller built for my devices for an OpenRGB server.

## Compiling

This project is built in rust and makes use of `cargo`.

To build:

- Clone this repo `git clone https://github.com/pi4erd/rgb-controller.git`
- `cd rgb-controller` and `cargo build --release`
- Everything should be compiled in `target/release` directory

## Support

App was tested only in Arch Linux so no support for other OSs.

## Configuration

Configuration is done in default configuration folder 
(`~/.config/rgb_controller/config.toml` for POSIX systems and
`%AppData%/rgb_controller/config.toml` for Windows)

TODO: Documentation on config options
