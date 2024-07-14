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
//!     is_svg::is_svg(include_str!("../tests/data/image.svg")),
//!     true
//! );
//! assert_eq!(
//!     is_svg::is_svg(include_bytes!("../tests/data/image.png")),
//!     false
//! );
//!
//! // `.svgz` is also supported.
//! assert_eq!(
//!     is_svg::is_svg(include_bytes!("../tests/data/image.svgz")),
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
///     is_svg::is_svg(include_str!("../tests/data/image.svg")),
///     true
/// );
/// assert_eq!(
///     is_svg::is_svg(include_bytes!("../tests/data/image.png")),
///     false
/// );
///
/// assert_eq!(
///     is_svg::is_svg(include_bytes!("../tests/data/image.svgz")),
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
///     is_svg::is_svg_string(include_str!("../tests/data/image.svg")),
///     true
/// );
/// assert_eq!(
///     is_svg::is_svg_string(include_bytes!("../tests/data/image.png")),
///     false
/// );
///
/// assert_eq!(
///     is_svg::is_svg_string(include_bytes!("../tests/data/image.svgz")),
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
///     is_svg::is_svgz(include_bytes!("../tests/data/image.svgz")),
///     true
/// );
/// assert_eq!(
///     is_svg::is_svgz(include_bytes!("../tests/data/image.png")),
///     false
/// );
///
/// assert_eq!(
///     is_svg::is_svgz(include_str!("../tests/data/image.svg")),
///     false
/// );
/// ```
pub fn is_svgz(data: impl AsRef<[u8]>) -> bool {
    let inner = |data: &[u8]| -> bool { is_svg(data) && data.starts_with(&GZIP_MAGIC_NUMBER) };
    inner(data.as_ref())
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_svg() {
        assert!(super::is_svg(include_str!("../tests/data/image-033.svg")));
        assert!(super::is_svg(include_str!("../tests/data/image.svg")));
        assert!(super::is_svg(include_str!("../tests/data/simple-text.svg")));
        assert!(super::is_svg(include_str!(
            "../tests/data/SVG_animation_using_CSS.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/SVG_animation_using_ES.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/SVG_animation_using_SMIL.svg"
        )));

        assert!(super::is_svg(include_bytes!("../tests/data/image.svgz")));

        assert!(!super::is_svg(include_str!("../tests/data/green.css")));
        assert!(!super::is_svg(include_bytes!(
            "../tests/data/image-63x61.png"
        )));
        assert!(!super::is_svg(include_bytes!("../tests/data/image.gif")));
        assert!(!super::is_svg(include_bytes!("../tests/data/image.jpg")));
        assert!(!super::is_svg(include_bytes!("../tests/data/image.png")));
    }

    #[test]
    fn is_svg_string() {
        assert!(super::is_svg_string(include_str!(
            "../tests/data/image-033.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/image.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/simple-text.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/SVG_animation_using_CSS.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/SVG_animation_using_ES.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/SVG_animation_using_SMIL.svg"
        )));

        assert!(!super::is_svg_string(include_bytes!(
            "../tests/data/image.svgz"
        )));

        assert!(!super::is_svg_string(include_str!(
            "../tests/data/green.css"
        )));
        assert!(!super::is_svg_string(include_bytes!(
            "../tests/data/image-63x61.png"
        )));
        assert!(!super::is_svg_string(include_bytes!(
            "../tests/data/image.gif"
        )));
        assert!(!super::is_svg_string(include_bytes!(
            "../tests/data/image.jpg"
        )));
        assert!(!super::is_svg_string(include_bytes!(
            "../tests/data/image.png"
        )));
    }

    #[test]
    fn is_svgz() {
        assert!(!super::is_svgz(include_str!("../tests/data/image-033.svg")));
        assert!(!super::is_svgz(include_str!("../tests/data/image.svg")));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/simple-text.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/SVG_animation_using_CSS.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/SVG_animation_using_ES.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/SVG_animation_using_SMIL.svg"
        )));

        assert!(super::is_svgz(include_bytes!("../tests/data/image.svgz")));

        assert!(!super::is_svgz(include_str!("../tests/data/green.css")));
        assert!(!super::is_svgz(include_bytes!(
            "../tests/data/image-63x61.png"
        )));
        assert!(!super::is_svgz(include_bytes!("../tests/data/image.gif")));
        assert!(!super::is_svgz(include_bytes!("../tests/data/image.jpg")));
        assert!(!super::is_svgz(include_bytes!("../tests/data/image.png")));
    }
}
