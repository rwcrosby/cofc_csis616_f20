# Options in `Rust`

## `Option` is an enum with two named values

- `Some` which can contiain the value of interest
- `None` which indicates that there is no value present


## To run

The `src/lib.rs` file contains the final version from class. The other files (`src/lib_1.rs` through `src/lib_5.rs`) are the intermediate versions.


This is setup as a lib rather than a main program. To run:

```
cargo test
```

or to run and see the output of the println! macros:
```
cargo test -- --nocapture
```