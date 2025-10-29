# Device API and data types

This crate contains common data types and API definitions that are common to both - firmware (server) and driver (client).
This crate is `#![no_std]` by default, but if owned types that use alloc are desired, you can enable them via `std` feature.

Name of this crate (from Cargo.toml) is assumed to be a globally unique identifier (see `ww_version::FullVersion`),
hence it is advised to eventually publish it to crates.io if you are working on an open-source project or ensure to use
unique enough name for internal use.

Version of this crate is used for compatibility checks upon connection to the device. You can use it in your
code as well, to show proper messages to user when interacting with an older or newer firmware from the perspective of the
driver. Normal [SemVer rules](https://semver.org) apply.

For example: firmware on device A is built with api `0.1.0` and on device B with `0.1.1` which introduces `do_x()` function.
Client code can then handle both devices and show a suggestion to update the firmware when user tries to use `do_x()` on `0.1.0`,
while all the previous functionality should still work as expected.

WireWeaver supports both backwards and forwards compatibility at the wire format level, but you need to ensure to follow
the [evolution rules](TODO link to evolution rules) for this to work properly.
Git pre-commit check is setup on this repository that will do these checks automatically
(WARNING: evolution checker is not yet fully featured and can miss errors).
