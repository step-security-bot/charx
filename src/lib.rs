// Copyright (C) 2022 Soni L.
// SPDX-License-Identifier: 0BSD

//! A replacement for char
//! ======================
//!
//! Because Rust's `char::is_ascii*` family of functions takes `&self`, it's
//! impossible to use them as patterns. This is inconsistent with the rest of
//! `char::is_*`, which takes `self`.
//!
//! This crate provides `char`-taking variants of the `is_ascii*` family of
//! functions.
//!
//! # Examples
//!
//! This works:
//!
//! ```rust
//! "hello".trim_start_matches(char::is_numeric);
//! ```
//!
//! This doesn't:
//!
//! ```rust compile_fail
//! "hello".trim_start_matches(char::is_ascii_digit);
//! ```
//!
//! This crate provides an alternative:
//!
//! ```rust
//! "hello".trim_start_matches(charx::is_ascii_digit);
//! ```

macro_rules! charx_fn {
    ($name:ident) => {
        #[doc=concat!("Same as [`char::", stringify!($name), "`] but takes `char` instead of `&char`.")]
        ///
        /// # Examples
        ///
        /// This doesn't compile:
        ///
        /// ```rust compile_fail
        #[doc=concat!("\"hello\".trim_start_matches(char::", stringify!($name), ");")]
        /// ```
        ///
        /// But this does:
        ///
        /// ```rust
        #[doc=concat!("\"hello\".trim_start_matches(charx::", stringify!($name), ");")]
        /// ```
        #[inline(always)]
        pub fn $name(c: char) -> bool {
            char::$name(&c)
        }
        mod $name {
            #[test]
            fn test() {
                for c in '\0'..='\u{10FFFF}' {
                    assert_eq!(char::$name(&c), crate::$name(c));
                }
            }
        }
    };
    ($($name:ident)*) => {
        $(charx_fn!($name);)*
    };
}

charx_fn!(is_ascii
is_ascii_alphabetic
is_ascii_alphanumeric
is_ascii_control
is_ascii_digit
is_ascii_graphic
is_ascii_hexdigit
is_ascii_lowercase
is_ascii_punctuation
is_ascii_uppercase
is_ascii_whitespace);
