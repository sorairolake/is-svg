// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The `is-svg` crate is a library for testing whether a given data is a [SVG]
//! image.
//!
//! This crate assumes the given data to be a valid SVG image if
//! [`usvg::Tree::from_data`] returns [`Ok`], and an invalid SVG image if it
//! returns [`Err`]. It supports both a SVG string and a [gzip-compressed] SVG
//! data.
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
//! [gzip-compressed]: https://datatracker.ietf.org/doc/html/rfc1952

#![doc(html_root_url = "https://docs.rs/is-svg/0.2.0/")]
// Lint levels of rustc.
#![deny(missing_docs)]

use usvg::{Options, Tree};

/// Magic number of gzip defined in [RFC 1952].
///
/// [RFC 1952]: https://datatracker.ietf.org/doc/html/rfc1952
const GZIP_MAGIC_NUMBER: [u8; 2] = [0x1f, 0x8b];

/// Returns [`true`] if `data` is a valid [SVG] data, and [`false`] otherwise.
///
/// This function also supports the [gzip-compressed] SVG image (`.svgz`).
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
///
/// [SVG]: https://www.w3.org/Graphics/SVG/
/// [gzip-compressed]: https://datatracker.ietf.org/doc/html/rfc1952
#[inline]
pub fn is_svg(data: impl AsRef<[u8]>) -> bool {
    let inner = |data: &[u8]| -> bool {
        let opt = Options::default();
        Tree::from_data(data, &opt).is_ok()
    };
    inner(data.as_ref())
}

/// Returns [`true`] if `data` is a valid non [gzip-compressed] [SVG] data
/// (`.svg`), and [`false`] otherwise.
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
///
/// [gzip-compressed]: https://datatracker.ietf.org/doc/html/rfc1952
/// [SVG]: https://www.w3.org/Graphics/SVG/
#[inline]
pub fn is_svg_string(data: impl AsRef<[u8]>) -> bool {
    let inner = |data: &[u8]| -> bool { is_svg(data) && !data.starts_with(&GZIP_MAGIC_NUMBER) };
    inner(data.as_ref())
}

/// Returns [`true`] if `data` is a valid [gzip-compressed] [SVG] data
/// (`.svgz`), and [`false`] otherwise.
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
///
/// [gzip-compressed]: https://datatracker.ietf.org/doc/html/rfc1952
/// [SVG]: https://www.w3.org/Graphics/SVG/
#[inline]
pub fn is_svgz(data: impl AsRef<[u8]>) -> bool {
    let inner = |data: &[u8]| -> bool { is_svg(data) && data.starts_with(&GZIP_MAGIC_NUMBER) };
    inner(data.as_ref())
}
