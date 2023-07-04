# PreMiD-rs
Alternative [PreMiD](https://premid.app/) backend written in Rust. Ditch that bloated [official Electron-based backend](https://github.com/PreMiD/PreMiD) and free up your precious RAM and disk space! 

Please note that this program doesn't include any bells and whistles, it just provides Discord rich presence.

> :warning: This project has not yet reached a stable release. Use at your own discretion.

## Comparison
Rust backend vs Electron-based backend:

| Feature             | Rust Backend | Official Backend |
|---------------------|--------------|------------------|
| Disk space required | ~11MB        | ~200MB           |
| RAM usage           | ~8MB         | ~20MB            |
| GUI                 | No           | Yes              |
| Tray icon           | No           | Yes              |
| Auto updater        | No           | Yes              |

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
