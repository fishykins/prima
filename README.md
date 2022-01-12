# Prima
<div id="top"></div>

[![Latest Version]][crates.io] 
[![docs]][docs.rs]
[![Minimum Supported Rust Version]][Rust 1.56]

a collection of geometry and graph based utilities with emphasis on simple code and readability.

<!-- ABOUT THE PROJECT -->
## About The Project

Prima is yet another geometry library, built around the core principle of simple code and readability. It provides helper structs
for primative shapes, basic collision detection and a suite of graph structures. The graphs are vector orientated and are built with
procedural world building in mind. Generic floats have been avoided in favour of explicitly typed structs, following the example of [glam](https://crates.io/crates/glam). Support for the crate [vek](https://crates.io/crates/vek) may be implimented in future updates.

<!-- GETTING STARTED -->
## Getting Started

As with most rust crates, this can be imported to a project using [crates.io](https://crates.io/crates). Follow the link for more infomation.

### Optional features
* [`rendering`] - adds functionality for exporting shapes and graphs to image files.


## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT)
  or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Contributions are both welcome and appreciated!

Contributions in any form (issues, pull requests, etc.) to this project must
adhere to Rust's [Code of Conduct].

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

<!-- CONTACT -->
## Contact

Caspar Green - caspar.m.green@gmail.com

Project Link: [https://github.com/fishykins/prima](https://github.com/fishykins/prima)

<p align="right">(<a href="#top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[Latest Version]: https://img.shields.io/crates/v/prima.svg
[crates.io]: https://crates.io/crates/prima/
[Minimum Supported Rust Version]: https://img.shields.io/badge/Rust-1.56.0-blue?color=fc8d62&logo=rust
[Rust 1.56]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1560-2021-10-21
[Code of Conduct]: https://www.rust-lang.org/en-US/conduct.html
[contributors]: https://github.com/bitshifter/glam-rs/graphs/contributors
[docs]: "https://img.shields.io/docsrs/prima/"
[docs.rs]: "https://docs.rs/prima/latest/prima/"