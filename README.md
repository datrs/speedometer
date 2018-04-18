# speedometer
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Measure throughput per second. Adapted from
[mafintosh/speedometer](https://github.com/mafintosh/speedometer).

- [Documentation][8]
- [Crates.io][2]

## Why?
It's often useful to figure out the average over a sliding timeframe. For
example: "how many bytes did we receive on average over the last 5 seconds?".
Or anything similar. This module allows you to do so in synchronous code.

## Usage
```rust
extern crate speedometer;
use speedometer::Speedometer;
use std::time::Duration;

let window_size = Duration::from_secs(5); // default is 5 second window size
let mut meter = Speedometer::new(window_size);
meter.entry(10);

println!("{:?} bytes/second!", meter.measure().unwrap());
```

## Installation
```sh
$ cargo add speedometer
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/speedometer.svg?style=flat-square
[2]: https://crates.io/crates/speedometer
[3]: https://img.shields.io/travis/datrs/speedometer.svg?style=flat-square
[4]: https://travis-ci.org/datrs/speedometer
[5]: https://img.shields.io/crates/d/speedometer.svg?style=flat-square
[6]: https://crates.io/crates/speedometer
[7]: https://docs.rs/speedometer/badge.svg
[8]: https://docs.rs/speedometer
