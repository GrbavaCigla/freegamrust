# freegamrust ![Rust](https://github.com/GrbavaCigla/freegamrust/workflows/Rust/badge.svg)

Conkly like program to fetch free games and display it with ~~xosd~~ x11rb. Made in rust.

## Configuration file
```
# Offset
horizontal-offset = 30
vertical-offset = 120

refresh-min = 60
```
Everything here is self-explanatory.

## Installation

```sh
git clone https://github.com/GrbavaCigla/freegamrust.git
cd freegamrust
cargo build --release
cargo install --path .
```
Make sure you have cargo bin folder in your path

## Usage
Start `freegamrust` on WM startup, for example in `.xinitrc`

## TODO
- Add color in config (~~not sure if xosd has that~~, x11rb can do that)
- Add support for more sources
- Add support for thread skipping
- Add signal for hiding

## License
Project is licensed under GPLv3
