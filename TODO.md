# Todo

- expose certain parts of the api only for testing? Transaction::new, clients_mut, ...
- in bank, can we continue after an error?
- don't accept inputs that are to big for f64.
- what should happen if one line in a CSV file is invalid?
- SourceCsv we can use from_path for csv instead of from_reader.
- rounding errors on the floats, 4 digits of precision after the decimal point required.
- overflow issues on the balance...
- what derived traits do we actually want on each type.
- verify api guidelines
- use OnceCell to get header out of CsvSource and avoid the allocation?
- document
