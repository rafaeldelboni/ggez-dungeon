# GGEZ Dungeon
[![Status][badge-status]][badge-status]
#### A *VERY* experimental project to test GGEZ and Rust on a simple Dungeon Crawler.

My goal is learn Rust while making a game and, after I archive and working MVP, I will try to refactor the all the code in a Functional way.

### Controls

| Action | Command |
| ------ | ------ |
| Walk | W A S D |

# Setup

## 1. Clone this repo:

Navigate into your workspace directory.

Run:
```bash
git clone https://github.com/rafaeldelboni/ggez-dungeon.git
```

## 2. Install Rust and SDL2 libraries:
  - [Rust](https://www.rust-lang.org/)
  - [SDL2 Libraries](https://github.com/Rust-SDL2/rust-sdl2#user-content-requirements)

## 3. Compile:
Navigate to the cloned repo’s directory.

Run:

```bash
cargo build
```

## 4. Run:
Run in release mode:
```bash
cargo run --release
```

Run in debug mode:
```bash
cargo run
```

## Update Spritesheets:
Be sure you're running this code at the root of the project directory!
```
  cargo run -p spritesheet
```

## Credits
Assets: https://opengameart.org/users/calciumtrice

Engine: https://github.com/ggez/ggez

[badge-status]: https://img.shields.io/badge/status-work%20in%20progress-lightgrey.svg
