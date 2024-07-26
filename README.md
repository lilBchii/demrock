# Demrock

This game is built with the macroquad crate. See the [macroquad repository](https://github.com/not-fl3/macroquad) for more information.

## Installation

```bash
# ubuntu system dependencies
sudo apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

# fedora system dependencies
dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel

# arch linux system dependencies
sudo pacman -S pkg-config libx11 libxi mesa-libgl alsa-lib

# windows or macos
no additional dependencies required

cargo build --release
```

## Running the game
```
cargo run --release
```