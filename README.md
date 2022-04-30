# transact

> Coding challenge financial transactions

Processing lib/bin for financial transactions. The crate contains a library with the bulk of the functionality and a small CLI front in `main.rs`. 
The crate uses a newtype `Balance`. This type guarantees that no account balances can ever be negative, NaN or Infinite. Any transactions that might
cause a balance to become any of these is rejected and an error returned.

The crate has no optional features.

`Cargo.toml` is auto generated from `Cargo.yml`. The latter will prove more readable for humans.


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

- [What Every Programmer Should Know About Floating-Point Arithmetic](https://floating-point-gui.de/)
- [What Every Computer Scientist Should Know About Floating-Point Arithmetic](https://docs.oracle.com/cd/E19957-01/806-3568/ncg_goldberg.html)
- [IEEE Standard for Floating-Point Arithmetic](https://irem.univ-reunion.fr/IMG/pdf/ieee-754-2008.pdf)


### Review the csv crate

The _csv_ crate initially takes our input. Is it fuzz tested? What could an attacker possibly throw at it that makes it choke/segfault/corrupt memory. I have only included some very basic tests. 
There are no cargo-crev reviews for _csv_. Note that _csv_ causes over 3k lines of unsafe code to be included into the build. If the input is as simple as in this exercise I would strongly recommend looking for either a CSV crate without unsafe, or hand rolling a simple parser without unsafe code. BurntSushi's own review of the _memchr_ crate, the biggest source of unsafe in _csv_, can be found [here](https://web.crev.dev/rust-reviews/crate/memchr/). The point of _memchr_ is to search in strings, using SIMD. However a CSV parser does not need search, especially for a well defined format like we use here. A hand rolled parser could very well be more performant as well as safer.

### std::io::Error

This is a bit of an annoying type. It is not `Clone`, nor `UnwindSafe`. This is contagious to `csv::Error` as well as our own `TransErr` and thus also `Bank`. For a more serious library I would propose replacing these error types with a simpler version that drops the internal pointer to the libc error and thus can be `Clone` and `UnwindSafe`. For this exercise I have not bothered however. It is also a point that the trait `UnwindSafe` is kind of meaningless right now, as it gives both false positives, false negatives and nobody else corrects that in their public types.

### Input

The application assumes that headers are present in the csv file. This has some implications. First of all, the first line of the input file is always ignored. If it did not actually contain headers, the first record just get's lost. There is currently no detection or verification that the first row is actually headers. Secondly, invalid utf8 in the first line is ignored and the file will be processed.

Invalid utf8 in other rows will just ignore that transaction (and report an error) but process the rest of the file.

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


