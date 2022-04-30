# transact

> Coding challenge financial transactions

Processing lib/bin for financial transactions. The crate contains a library with the bulk of the functionality and a small CLI front in `main.rs`. 

The crate has no optional features.


## Usage

`cargo run -- "path/to/file.csv"`

## Api docs

Can be generated with `cargo +nightly doc --no-deps --open`.


## Review questions

### Tests/Docs

The integration tests are not complete. Eg. the error conditions in `Bank` are not all tested in `tests/errors.rs`, `tests/parce_csv.rs` is not exhaustive either. For real production code I would suggest making the tests exhaustive but for the sake of the coding challenge I cut it short.

Also, documentation is minimal. There are no examples, nor inline, nor in the `/examples` directory.

### Floating point arithmetic

The application currently uses f64 floats to represent account balance. Floats have a risk of both rounding errors and overflow. I cannot detect any problems as long as numbers are limited to a maximum of 12 places before and 4 places after the decimal point. The specification did not mention anything about the maximum expected account balance.

If more precision is required, a third party solution is probably necessary. Possibly the _decimal_ crate. It uses the C library [decNumber](http://speleotrove.com/decimal/decnumber.html) under the hood and would require reviewing.

#### References

[What Every Programmer Should Know About Floating-Point Arithmetic](https://floating-point-gui.de/)
[What Every Computer Scientist Should Know About Floating-Point Arithmetic](https://docs.oracle.com/cd/E19957-01/806-3568/ncg_goldberg.html)
[IEEE Standard for Floating-Point Arithmetic](https://irem.univ-reunion.fr/IMG/pdf/ieee-754-2008.pdf)


### Review the csv crate

The csv crate initially takes our input. Is it fuzz tested? What could an attacker possibly throw at it that makes it choke/segfault/corrupt memory. I have only included some very basic tests. 
There are no cargo-crev reviews for csv. Note that csv causes over 3k lines of unsafe code to be included into the build. If the input is as simple as in this exercise I would strongly recommend looking for either a CSV crate without unsafe, or hand rolling a simple parser without unsafe code. BurntSushi's own review of the memchr crate, the biggest source of unsafe, can be found [here](https://web.crev.dev/rust-reviews/crate/memchr/). The point of memchr is to search in strings, using SIMD. However a CSV parser does not need search, especially for a well defined format like we use here. A handrolled parser could very well be more performant as well as safer.

### Input

The application assumes that headers are present in the csv file. 

### Ambiguities

- what to do in case of a dispute when not enough money is available (withdraws have happened since)? 
  For now I reject the dispute, but several alternatives are possible:
  - put the available account at a negative balance
  - allow disputing up to the sum available, even if that is less than the amount of the transaction that is disputed.

## Contributing

This project does not accept contributions.

## Code of conduct

Any of the behaviors described in [point 4 "Unacceptable Behavior" of the Citizens Code of Conduct](https://github.com/stumpsyn/policies/blob/master/citizen_code_of_conduct.md#4-unacceptable-behavior) are not welcome here and might get you banned. If anyone, including maintainers and moderators of the project, fail to respect these/your limits, you are entitled to call them out.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.


