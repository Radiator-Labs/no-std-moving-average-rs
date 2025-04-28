# No-Std-Moving-Average [![Cloud-CI](https://github.com/Radiator-Labs/no-std-moving-average-rs/actions/workflows/cloud-ci.yml/badge.svg)](https://github.com/Radiator-Labs/no-std-moving-average-rs/actions/workflows/cloud-ci.yml)

Guard-clause library, providing syntactic sugar to improve readability of
[guard-clauses](https://en.wikipedia.org/wiki/Guard_(computer_science))
in Rust code.

## Intention

This supplies a [moving average](https://en.wikipedia.org/wiki/Moving_average) algorithm that can be used in a nostd environment.

To ensure that the behavior is consistent, the first insertion stuffs the entire buffer with the first value.

## Attribution

Work creating this library was performed as part of commercial development
by [Kelvin](https://kel.vin/) (formerly Radiator Labs), a green energy company
dedicated to decarbonizing the world's legacy buildings.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.86 and up. It *might*
compile with older versions but that may change in any new patch release.
Changes in Clippy support are the main factor driving the version dependency.

See [here](../docs/msrv.md) for details on how the MSRV may be upgraded.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## end
