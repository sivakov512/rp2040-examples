# Project template for rp2040-hal

This template is intended as a starting point for developing firmware based on the rp2040-hal.


## Requirements

- The standard Rust tooling (cargo, rustup) which you can install from https://rustup.rs/

- Toolchain support for the cortex-m0+ processors in the rp2040 (thumbv6m-none-eabi)

- flip-link - this allows you to detect stack-overflows on the first core, which is the only supported target for now.

- A [`probe-rs` installation](https://probe.rs/docs/getting-started/installation/)

- A [`probe-rs` compatible](https://probe.rs/docs/getting-started/probe-setup/) probe

You can use a second [Pico as a CMSIS-DAP debug probe](debug_probes.md#raspberry-pi-pico). Details on other supported debug probes can be found in [debug_probes.md](debug_probes.md)


## Installation of development dependencies

```sh
rustup target install thumbv6m-none-eabi
cargo install flip-link
# Installs the probe-rs tools
cargo install probe-rs --features=cli --locked
```


## Building
  
For a debug build
```sh
cargo build
```
For a release build
```sh
cargo build --release
```


## Flashing and debugging

To compile debug build and flash you can run 
```sh
cargo flash
```
For a release build
```sh
cargo flash --release
```
See [the `cargo-flash` tool docs page](https://probe.rs/docs/tools/cargo-flash/) for more information.
  
To compile, flash device and start configuration specified in Embed.toml run
```sh
cargo embed
```
By default it runs with RTT logging and debugging session after flashing, so you can attach with `gdb` on port 1337. Logging level is `debug` by default, but you can override this with `DEFMT_LOG` environment variable or specify it right in `.cargo/config.toml`
```toml
[env]
DEFMT_LOG = "off"
```

You can find all the settings for [Embed.toml](./Embed.toml) and their meanings [in the probe-rs repo](https://github.com/probe-rs/probe-rs/blob/c0610e98008cbb34d0dc056fcddff0f2d4f50ad5/probe-rs/src/bin/probe-rs/cmd/cargo_embed/config/default.toml)

See [the `cargo-embed` tool docs page](https://probe.rs/docs/tools/cargo-embed/) for more information. 


## Notes on using rp2040_hal and rp2040_boot2

The second-stage boot loader must be written to the .boot2 section. That is usually handled by the board support package (e.g.`rp-pico`). If you don't use one, you should initialize the boot loader manually. This can be done by adding the following to the beginning of main.rs:

```rust
use rp2040_boot2;
#[link_section = ".boot2"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
```

## Credits

Highly inspired by https://github.com/rp-rs/rp2040-project-template
