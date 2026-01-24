# wire_weaver_template

This template showcases how to use [WireWeaver](https://github.com/vhrdtech/wire_weaver) on real hardware, connected via USB.
Only basic functionality is used to keep things simple.

## Overview
* `api` - Common data types and API description, used by fw driver crates.
* `fw` - Example firmwares for several development boards with USB support.
  * `fw/nucleo_h743zi2` - works on Nucleo H743ZI2 and H753ZI boards
  * `fw/cannify_micro_g0b1cetxn` - vhrd.tech internal dev board, but can be easily adapted to STM32G0 boards
  * Any other board with USB supported by embassy should work as well! The only dependency is on embassy-usb.
* `driver` - Rust driver with API client.
  * `driver/examples/simple` - Basic API client usage example.
  * `driver/examples/repl` - REPL based on clap, supports command history and auto-complete.

Note that `fw` folder is excluded from Cargo workspace. If your IDE is struggling to properly parse this setup, open
fw projects separately.

## Step-by-step guide
* [ ] Checkout the `api` crate [README](api/README.md)
* [ ] Rename `api` crate to your name (this will break the code in several places, TODO: add rename script)
  * For example: `api` -> `my_device_api`
  * `driver` -> `my_device_driver` or `my_device`
  * `driver_py` -> `my_device_py`
* [ ] Run one of the firmwares
  * [ ] If you do not have hardware to run it - adapt to other dev. board with USB that embassy supports
  * [ ] TODO: Alternatively - run the virtual device emulator (can later be used for integration tests as well)
* [ ] Run driver/examples/simple: `cargo run -p driver --example simple`
* [ ] Optionally checkout the REPL example in driver/examples/repl: `cargo run -p driver --example repl`
* [ ] Optionally inspect `target/generated_std_client.rs` and `fw/_/target/generated_no_std_server.rs`

## Syncing upstream template changes
Add upstream:
`git remote add upstream https://github.com/vhrdtech/wire_weaver_template.git`

Fetch changes from upstream:
`git fetch upstream`

Merge:
`git merge upstream/main --allow-unrelated-histories`

If you did not made any local changes, merge accepting remote changes:
Make sure to commit and push or backup your changes before executing this command.
`git merge upstream/main --allow-unrelated-histories --strategy-option theirs`
