# tiger-chip8

## Desktop
```
A CHIP-8 emulator written in Rust

Usage: tiger-chip8 [OPTIONS] <ROM_FILE>

Arguments:
  <ROM_FILE>  

Options:
  -s, --scale <SCALE>                  [default: 15]
  -o, --ops-per-frame <OPS_PER_FRAME>  [default: 10]
  -h, --help                           Print help
  -V, --version                        Print version
```

## Running the Emulator

### Prerequisites
You need to have SDL2 installed on your machine.

Instruction can be found in the [sdl2 github readme](https://github.com/Rust-SDL2/rust-sdl2).

#### MacOS
```
brew install sdl2
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

### Without Compiling an Executable
```
cargo run --release -- desktop/roms/PONG -s 10 -o 8
```

### With Compiling an Executable
```
cargo build --release
cp target/release/tiger-chip8 .
./tiger-chip8 desktop/roms/PONG
```
