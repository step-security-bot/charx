# A replacement for char

Because Rust's `char::is_ascii*` family of functions takes `&self`, it's
impossible to use them as patterns. This is inconsistent with the rest of
`char::is_*`, which takes `self`.

This crate provides `char`-taking variants of the `is_ascii*` family of
functions.
