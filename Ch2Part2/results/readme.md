# Results in `Rust`

## `Result` is an enum with two named values

- `Ok` which can contain a value
- `Err` which can also contain a value, usually an error message


## To run

This is setup as a lib rather than a main program. To run:

```
cargo test
```

or to run and see the output of the println! macros:
```
cargo test -- --nocapture
```