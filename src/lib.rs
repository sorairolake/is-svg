// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The `is-svg` crate is a library for testing whether a given data is a [SVG]
//! image.
//!
//! # Examples
//!
//! ```
//! assert_eq!(
//!     is_svg::is_svg(include_str!("../tests/data/w3/svg-logo-v.svg")),
//!     true
//! );
//! assert_eq!(
//!     is_svg::is_svg(include_bytes!("../tests/data/w3/svg-logo-v.png")),
//!     false
//! );
//!
//! // `.svgz` is also supported.
//! assert_eq!(
//!     is_svg::is_svg(include_bytes!("../tests/data/w3/svg-logo-v.svgz")),
//!     true
//! );
//! ```
//!
//! [SVG]: https://www.w3.org/Graphics/SVG/

#![doc(html_root_url = "https://docs.rs/is-svg/0.1.0/")]
// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, missing_docs)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

use usvg::{Options, Tree};

/// Magic number of gzip defined in [RFC 1952].
///
/// [RFC 1952]: https://datatracker.ietf.org/doc/html/rfc1952
const GZIP_MAGIC_NUMBER: [u8; 2] = [0x1f, 0x8b];

/// Returns [`true`] if `data` is a valid SVG data, and [`false`] otherwise.
///
/// This function also supports the gzip-compressed SVG image (`.svgz`).
///
/// # Examples
///
/// ```
/// assert_eq!(
///     is_svg::is_svg(include_str!("../tests/data/w3/svg-logo-v.svg")),
///     true
/// );
/// assert_eq!(
///     is_svg::is_svg(include_bytes!("../tests/data/w3/svg-logo-v.png")),
///     false
/// );
///
/// assert_eq!(
///     is_svg::is_svg(include_bytes!("../tests/data/w3/svg-logo-v.svgz")),
///     true
/// );
/// ```
pub fn is_svg(data: impl AsRef<[u8]>) -> bool {
    let inner = |data: &[u8]| -> bool {
        let opt = Options::default();
        Tree::from_data(data, &opt).is_ok()
    };
    inner(data.as_ref())
}

/// Returns [`true`] if `data` is a valid non gzip-compressed SVG data (`.svg`),
/// and [`false`] otherwise.
///
/// This function returns [`false`] if `data` is a valid SVG data, but
/// gzip-compressed (`.svgz`).
///
/// # Examples
///
/// ```
/// assert_eq!(
///     is_svg::is_svg_string(include_str!("../tests/data/w3/svg-logo-v.svg")),
///     true
/// );
/// assert_eq!(
///     is_svg::is_svg_string(include_bytes!("../tests/data/w3/svg-logo-v.png")),
///     false
/// );
///
/// assert_eq!(
///     is_svg::is_svg_string(include_bytes!("../tests/data/w3/svg-logo-v.svgz")),
///     false
/// );
/// ```
pub fn is_svg_string(data: impl AsRef<[u8]>) -> bool {
    let inner = |data: &[u8]| -> bool { is_svg(data) && !data.starts_with(&GZIP_MAGIC_NUMBER) };
    inner(data.as_ref())
}

/// Returns [`true`] if `data` is a valid gzip-compressed SVG data (`.svgz`),
/// and [`false`] otherwise.
///
/// This function returns [`false`] if `data` is a valid SVG data, but non
/// gzip-compressed (`.svg`).
///
/// # Examples
///
/// ```
/// assert_eq!(
///     is_svg::is_svgz(include_bytes!("../tests/data/w3/svg-logo-v.svgz")),
///     true
/// );
/// assert_eq!(
///     is_svg::is_svgz(include_bytes!("../tests/data/w3/svg-logo-v.png")),
///     false
/// );
///
/// assert_eq!(
///     is_svg::is_svgz(include_str!("../tests/data/w3/svg-logo-v.svg")),
///     false
/// );
/// ```
pub fn is_svgz(data: impl AsRef<[u8]>) -> bool {
    let inner = |data: &[u8]| -> bool { is_svg(data) && data.starts_with(&GZIP_MAGIC_NUMBER) };
    inner(data.as_ref())
}
