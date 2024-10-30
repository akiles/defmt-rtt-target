# ARCHIVED

Don't use this. `rtt-target` itself has defmt support now, making this crate unnecessary.

- Example: https://github.com/probe-rs/rtt-target/blob/master/examples-cortex-m/src/bin/defmt.rs
- PR that added it: https://github.com/probe-rs/rtt-target/pull/49

# `defmt-rtt-target`

[`defmt`](https://github.com/knurling-rs/defmt) logger implementation using [`rtt-target`](https://github.com/mvirkkunen/rtt-target).

The official [`defmt-rtt`](https://github.com/knurling-rs/defmt/tree/main/firmware/defmt-rtt) crate has its own RTT
implementation that is not externally exposed. This is done so that users can't corrupt the defmt log stream
by manually writing to the RTT channel. This library exists as an alternate option for when you do need
control over RTT, for example to log extra non-defmt data on other channels, or have a host->target "down channel".

If all you need is logging, you should use `defmt-rtt` instead of this.

## Usage

- Initialize `rtt_target` in your code
- Call `defmt_rtt_target::init()` with the `UpChannel` you want to use for RTT printing. This should be channel 0, as that's the one all defmt tools use.
- Now you can log with the standard defmt macros.

Note that all log output before calling `defmt_rtt_target::init()` is discarded.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
