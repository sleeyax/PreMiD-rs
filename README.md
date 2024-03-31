# PreMiD-rs
Alternative [PreMiD](https://premid.app/) backend written in Rust. Ditch that bloated [official Electron-based backend](https://github.com/PreMiD/PreMiD) and free up your precious RAM and disk space! 

Please note that this program doesn't include any bells and whistles, it just provides Discord rich presence.

> [!WARNING]  
> This project uses relateively unstable libraries. It should work but use at your own discretion until we publish a stable release.

## Comparison
Rust backend vs Electron-based backend:

| Feature             | Rust Backend | Official Backend |
|---------------------|--------------|------------------|
| Disk space required | ~11MB        | ~200MB           |
| RAM usage           | ~8MB         | ~20MB            |
| GUI                 | No           | Yes              |
| Tray icon           | No           | Yes              |
| Auto updater        | No           | Yes              |

## Installation

1. Download the [latest release](https://github.com/sleeyax/PreMiD-rs/releases/latest).
2. (optional) Add the program to startup so it launches on PC boot/login.
3. Execute the program from a terminal/cmd. You won't see any output by default. Specify an environment variable [below](#commands) for more output options.

## Missing features
Known missing features:

- Settings
- Local presence (for presence development)

I'm undecided whether these features should be added. Open an issue if you wish to discuss this!

## Development
Requires [rust](https://www.rust-lang.org/tools/install) to be installed on your system (duh).

### Commands

Run with basic logging:
```bash
$ RUST_LOG="info" cargo run
```

Run with verbose logging:
```bash
$ RUST_LOG="debug" cargo run
```

Run with default logging level:
```bash
$ cargo run
```
