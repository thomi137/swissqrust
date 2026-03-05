# swissqrust

[![swiss_qrust](https://github.com/thomi137/swissqrust/actions/workflows/rust.yml/badge.svg)](https://github.com/thomi137/swissqrust/actions/workflows/rust.yml) 
[![swiss_qrust](https://github.com/thomi137/swissqrust/actions/workflows/ci.yml/badge.svg)](https://github.com/thomi137/swissqrust/actions/workflows/ci.yml) 

[![Rust 1.90.0+](https://img.shields.io/badge/rust-1.87.0+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# Swiss QR Payment Slips in Rust

This is for the moment an experimental project to generate Swiss payment slips using Rust. The main target was for
me to learn Rust, but since I started Software Development, I was obsessed with speed.
So Java an Python versions, while more easily integratable, to my mind are just slow.  Feel free to fork and send PRs.

Currently, the core (everyting in src) is producing valid Swiss QR payment slips.
There are some rough edges and test cases that are not yet implemented.

## Examples
In the examples directory, you can find two files:

- `cli.rs` is a simple example that generates a payment slip and saves it to a file. You can select input and output files. It works with both .toml and .json \
How these need to be formatted is shown in the data directory.
- `gui.rs` is a fully functional native GUI application. At the moment it is only configured for macOS and does not support cross compilation for all mac processors (that would be a nice PR, if you would like to help)
There are icons for macOS, Windows and Linux. so using packager on your machine should give you a styled application. The developer got carried away so he built a styled gui compiling down into a .dmg for mac, completely
sidetracking the actual showcase. But it was fun to learn (iced)[https://docs.rs/iced/0.14.0/iced/index.html]  🤷‍♂️

### Executing
however, you do not need to run packager. To see how this works, run

```
cargo run --example cli -- --input data/robert_schneider.json --output output/bill.pdf --lang fr # or de or it or en, default is en
```

And for the GUI version:

```
cargo run --example gui # GUI version
```

If you want to see how to package that into a working application,
run 

```
cargo packager --release
```

It will compile to a release version and package it into a .dmg file for macOS.
Currently, the developer does not have access to a windows or linux machine, so if you would like
to help, please let him know. Or better yet, send a PR. Icons and assets are there and hooked up in the Cargo.toml.

## Outlook

1. Implement the rest of the Swiss QR specification
2. Add more examples, particularly a WASM example.
3. Add more tests
4. Add more documentation or better yet, start the documentaton...
