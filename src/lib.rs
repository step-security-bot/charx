// Copyright (C) 2022 Soni L.
// SPDX-License-Identifier: 0BSD
//
// This work is released under the BSD Zero Clause License.
//
// See https://spdx.org/licenses/0BSD.html or the LICENSE file for more information.

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
    ($funcname:ident) => {
        #[doc=concat!("Same as [`char::", stringify!($funcname), "`] but takes `char` instead of `&char`.")]
        ///
        /// # Examples
        ///
        /// This doesn't compile:
        ///
        /// ```rust compile_fail
        #[doc=concat!("\"hello\".trim_start_matches(char::", stringify!($funcname), ");")]
        /// ```
        ///
        /// But this does:
        ///
        /// ```rust
        #[doc=concat!("\"hello\".trim_start_matches(charx::", stringify!($funcname), ");")]
        /// ```
        #[inline(always)]
        pub fn $funcname(ch: char) -> bool {
            char::$funcname(&ch)
        }
        mod $funcname {
            #[test]
            fn test() {
                for ch in '\0'..='\u{10FFFF}' {
                    assert_eq!(char::$funcname(&ch), crate::$funcname(ch));
                }
            }
        }
    };
    ($($funcname:ident)*) => {
        $(charx_fn!($funcname);)*
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
