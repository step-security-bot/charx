# A replacement for char

![crates.io](https://img.shields.io/crates/v/charx.svg)
![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/AliSajid/charx)
[![Continuous integration](https://github.com/AliSajid/charx/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/AliSajid/charx/actions/workflows/ci.yaml)
![Crates.io](https://img.shields.io/crates/l/charx)
[![Contribute with Gitpod](https://img.shields.io/badge/Contribute%20with-Gitpod-908a85?logo=gitpod)](https://gitpod.io/#AliSajid/charx)


[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#AliSajid/charx)

This project aims to develop a small cross-platform command line interface (CLI) for quickly calculating protein abundance using the Pierce BCA Protein Assay. The CLI is written in Rust and is available for Windows, macOS, and Linux.

## Builds

| Platform | Rust Version |Status |
| -------- | ------ | ------ |
| Linux    | stable <br/> beta <br/> nightly <br/> MSRV (1.58.1) | ![Ubuntu x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/ubuntu-stable.json) <br/> ![Ubuntu x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/ubuntu-beta.json) <br/> ![Ubuntu x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/ubuntu-nightly.json) <br/> ![Ubuntu x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/ubuntu-msrv.json) |
| Windows  | stable <br/> beta <br/> nightly <br/> MSRV (1.58.1) | ![macos x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/windows-stable.json) <br/> ![macos x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/windows-beta.json) <br/> ![macos x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/windows-nightly.json) <br/> ![macos x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/windows-msrv.json) |
| macOS    | stable <br/> beta <br/> nightly <br/> MSRV (1.58.1) | ![Windows x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/macos-stable.json) <br/> ![Windows x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/macos-beta.json) <br/> ![Windows x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/macos-nightly.json) <br/> ![Windows x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/macos-msrv.json) |

Because Rust's `char::is_ascii*` family of functions takes `&self`, it's
impossible to use them as patterns. This is inconsistent with the rest of
`char::is_*`, which takes `self`.

This crate provides `char`-taking variants of the `is_ascii*` family of
functions.
