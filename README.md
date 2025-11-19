ainakan-rust
==========

Rust bindings for [Ainakan](http://www.ainakan.re/).

## Install

- Build Ainakan, or download the devkits for your system (see `ainakan-gum` or `ainakan-core` README for version)
- For crate installation:
    - Move the ainakan-gum and ainakan-core devkits into `rustc-link-search`, e.g.: `/usr/local/{include, lib}` on Unix
- For local development:
    - Move the ainakan-gum devkit into `ainakan-gum-sys`, and the ainakan-core devkit into `ainakan-sys` and `cargo build` in the root
