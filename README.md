# 10x Cloud Champion

[![pr_check](https://github.com/Enet4/10xCloudChampion/actions/workflows/pr_check.yml/badge.svg)](https://github.com/Enet4/10xCloudChampion/actions/workflows/pr_check.yml)

A Cloud computing simulation/clicker game.

## Setting up

This is a Yew app that's built with [Trunk].

If you don't already have it installed, install Rust: <https://www.rust-lang.org/tools/install>.
The rest of this guide assumes a typical Rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.
If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Then, ensure that you have [Trunk].

```bash
cargo install trunk wasm-bindgen-cli
```

## Running

Use the following command to deploy a local server and rebuild the app whenever a change is detected.

```bash
trunk serve
```

There's also the `trunk watch` command which does the same thing but without hosting it.

## Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

The output will be located in the `dist` directory.

## Test playground

A separate web application is available by enabling the Cargo feature `playground`.
This replaces the game with a different page containing an assortment of components to play around with.

## Licensing and Attribution

All source code is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

Third party content and respective attribution is listed in [SOURCES.md](SOURCES.md).
All original non-code assets other than those described above,
are licensed under a [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/).
![](https://i.creativecommons.org/l/by-sa/4.0/80x15.png)

[trunk]: https://github.com/thedodd/trunk
