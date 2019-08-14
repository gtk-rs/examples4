# gtk-rs examples for gtk4 [![Build Status](https://travis-ci.org/gtk-rs/examples4.png?branch=master)](https://travis-ci.org/gtk-rs/examples4)

A few gtk-rs examples for __gtk4__. To build, just do:

```Shell
> cargo build
```

or to enable GTK 4.x depending on the version needed by the example (check Cargo.toml `[features]` to see all specific GTK compile features available):

```Shell
> cargo build --features [feature]
> cargo build --all-features
```

And then run the executables with:

``` Shell
./target/debug/EXAMPLE-NAME
```

or with cargo run (repeating the compilation features used above), example:

``` Shell
cargo run --all-features --bin EXAMPLE-NAME
```

Please be sure to have installed all the required libraries before building examples (the list is available on the [gtk-rs](https://github.com/gtk-rs/gtk4/) repository).

## LICENSE
The gtk-rs examples repository is licensed under the MIT license, please refer to the LICENSE and COPYRIGHT files for more information.
