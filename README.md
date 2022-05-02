# transact

> Coding challenge financial transactions

Processing lib/bin for financial transactions. The crate contains a library with the bulk of the functionality and a small CLI front in `main.rs`. 

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

Floating point arithmetic can cause rounding and overflow errors which are undesirable in a financial application. Since no upper bound on account balances is specified, I just used the _bigdecimal_ crate which has arbitrary precision with up to 2^63 decimal places. If this is overkill and more performance is desired, potentially the _rust_decimal_ crate can be considered as it uses just one `u128` for storage.


### Review the csv crate

The _csv_ crate initially takes our input. Is it fuzz tested? What could an attacker possibly throw at it that makes it choke/segfault/corrupt memory. I have only included some very basic tests. 
There are no cargo-crev reviews for _csv_. Note that _csv_ causes over 3k lines of unsafe code to be included into the build. If the input is as simple as in this exercise I would strongly recommend looking for either a CSV crate without unsafe, or hand rolling a simple parser without unsafe code. BurntSushi's own review of the _memchr_ crate, the biggest source of unsafe in _csv_, can be found [here](https://web.crev.dev/rust-reviews/crate/memchr/). The point of _memchr_ is to search in strings, using SIMD. However a CSV parser does not need search, especially for a well defined format like we use here. A hand rolled parser could very well be more performant as well as safer and relatively trivial to implement.

### std::io::Error

This is a bit of an annoying type. It is not `Clone`, nor `UnwindSafe`. This is contagious to `csv::Error` as well as our own `TransErr` and thus also `Bank`. For a more serious library I would propose replacing these error types with a simpler version that drops the internal pointer to the libc error and thus can be `Clone` and `UnwindSafe`. For this exercise I have not bothered however. It is true though that the trait `UnwindSafe` is kind of meaningless right now, as it gives both false positives, false negatives and nobody else corrects that in their public types.

### Input

The application only accepts input with a header line.
Invalid utf8 in other rows will just ignore that transaction (and report an error) but process the rest of the file.

### Ambiguities

- what to do in case of a dispute when not enough money is available (withdraws have happened since)? 
  For now I reject the dispute, but several alternatives are possible:
  - put the available account at a negative balance
  - allow disputing up to the sum available, even if that is less than the amount of the transaction that is disputed.
  

### Performance

No optimization has been done. The crate tries not to waste performance for no reason, but no benchmarks where run.
The _bigdecimal_ crate might not be the most performant way of representing account balances for example.

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


