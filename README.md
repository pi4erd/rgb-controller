# rgb-controller

A small rgb controller built for my devices for an OpenRGB server.

## Compiling

This project is built in rust and makes use of `cargo`.

To build:

- Clone this repo `git clone https://github.com/pi4erd/rgb-controller.git`
- `cd rgb-controller` and `cargo build --release`
- Everything should be compiled in `target/release` directory

Additionally, you can install this directly to the /usr/local/bin/ directory
by using `install.sh` script. Works only for POSIX systems and tested only
on Arch.

## Support

App was tested only in Arch Linux so no support for other OSs.

## Configuration

Configuration is done in default configuration folder 
(`~/.config/rgb_controller/config.toml` for POSIX systems and
`%AppData%/rgb_controller/config.toml` for Windows)

Example configuration:

```toml
# Format info, used to discern versions of configs
[format_info]
version = 2

# Controller configs
# Each config has a user-given name and has a few properties:
# - controller_id (int) - id of the OpenRGB controller
# - selected_mode (int) - selected preset from existing ones
[controller_configs.my_config]
controller_id = 0
selected_mode = 1
```
