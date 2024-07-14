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
//!     is_svg::is_svg(include_str!("../tests/data/resvg/image.svg")),
//!     true
//! );
//! assert_eq!(
//!     is_svg::is_svg(include_bytes!("../tests/data/resvg/image.png")),
//!     false
//! );
//!
//! // `.svgz` is also supported.
//! assert_eq!(
//!     is_svg::is_svg(include_bytes!("../tests/data/resvg/image.svgz")),
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
///     is_svg::is_svg(include_str!("../tests/data/resvg/image.svg")),
///     true
/// );
/// assert_eq!(
///     is_svg::is_svg(include_bytes!("../tests/data/resvg/image.png")),
///     false
/// );
///
/// assert_eq!(
///     is_svg::is_svg(include_bytes!("../tests/data/resvg/image.svgz")),
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
///     is_svg::is_svg_string(include_str!("../tests/data/resvg/image.svg")),
///     true
/// );
/// assert_eq!(
///     is_svg::is_svg_string(include_bytes!("../tests/data/resvg/image.png")),
///     false
/// );
///
/// assert_eq!(
///     is_svg::is_svg_string(include_bytes!("../tests/data/resvg/image.svgz")),
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
///     is_svg::is_svgz(include_bytes!("../tests/data/resvg/image.svgz")),
///     true
/// );
/// assert_eq!(
///     is_svg::is_svgz(include_bytes!("../tests/data/resvg/image.png")),
///     false
/// );
///
/// assert_eq!(
///     is_svg::is_svgz(include_str!("../tests/data/resvg/image.svg")),
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
    #[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
    fn is_svg_from_svg() {
        assert!(super::is_svg(include_str!(
            "../tests/data/resvg/image-033.svg"
        )));
        assert!(super::is_svg(include_str!("../tests/data/resvg/image.svg")));
        assert!(super::is_svg(include_str!(
            "../tests/data/resvg/simple-text.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/clip-path-with-complex-text-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/clip-path-with-complex-text.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/clip-path-with-object-units-multi-use-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/clip-path-with-object-units-multi-use.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/clip-path-with-text-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/clip-path-with-text.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/ellipse-simple-case-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/ellipse-simple-case.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/filter-id-with-prefix-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/filter-id-with-prefix.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/filter-with-object-units-multi-use-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/filter-with-object-units-multi-use.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/generate-id-clip-path-for-symbol-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/generate-id-clip-path-for-symbol.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/generate-id-filter-function-v1-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/generate-id-filter-function-v1.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/generate-id-filter-function-v2-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/generate-id-filter-function-v2.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/mask-with-object-units-multi-use-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/mask-with-object-units-multi-use.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/path-simple-case-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/path-simple-case.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-id-clip-path-v1-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-id-clip-path-v1.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-id-clip-path-v2-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-id-clip-path-v2.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-id-fe-image-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-id-fe-image-with-opacity-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-id-fe-image-with-opacity.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-id-fe-image.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-id-filter-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-id-filter.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-id-for-clip-path-in-pattern-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-id-for-clip-path-in-pattern.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-in-clip-path-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-in-clip-path.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-in-mask-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-in-mask.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-in-pattern-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-in-pattern.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-multiple-font-families-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-multiple-font-families.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-on-path-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-on-path.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-simple-case-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-simple-case.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-with-complex-text-decoration-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-with-complex-text-decoration.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-with-dx-and-dy-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-with-dx-and-dy.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-with-nested-baseline-shift-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-with-nested-baseline-shift.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-with-rotate-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/preserve-text-with-rotate.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/text-simple-case-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/text-simple-case.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/text-with-generated-gradients-expected.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/usvg/text-with-generated-gradients.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/wikipedia/SVG_animation_using_CSS.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/wikipedia/SVG_animation_using_ES.svg"
        )));
        assert!(super::is_svg(include_str!(
            "../tests/data/wikipedia/SVG_animation_using_SMIL.svg"
        )));
    }

    #[test]
    fn is_svg_from_svgz() {
        assert!(super::is_svg(include_bytes!(
            "../tests/data/resvg/image.svgz"
        )));
    }

    #[test]
    fn is_svg_from_non_svg() {
        assert!(!super::is_svg(include_str!(
            "../tests/data/resvg/green.css"
        )));
        assert!(!super::is_svg(include_bytes!(
            "../tests/data/resvg/image-63x61.png"
        )));
        assert!(!super::is_svg(include_bytes!(
            "../tests/data/resvg/image.gif"
        )));
        assert!(!super::is_svg(include_bytes!(
            "../tests/data/resvg/image.jpg"
        )));
        assert!(!super::is_svg(include_bytes!(
            "../tests/data/resvg/image.png"
        )));
    }

    #[test]
    #[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
    fn is_svg_string_from_svg() {
        assert!(super::is_svg_string(include_str!(
            "../tests/data/resvg/image-033.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/resvg/image.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/resvg/simple-text.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/clip-path-with-complex-text-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/clip-path-with-complex-text.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/clip-path-with-object-units-multi-use-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/clip-path-with-object-units-multi-use.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/clip-path-with-text-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/clip-path-with-text.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/ellipse-simple-case-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/ellipse-simple-case.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/filter-id-with-prefix-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/filter-id-with-prefix.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/filter-with-object-units-multi-use-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/filter-with-object-units-multi-use.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/generate-id-clip-path-for-symbol-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/generate-id-clip-path-for-symbol.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/generate-id-filter-function-v1-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/generate-id-filter-function-v1.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/generate-id-filter-function-v2-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/generate-id-filter-function-v2.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/mask-with-object-units-multi-use-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/mask-with-object-units-multi-use.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/path-simple-case-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/path-simple-case.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-id-clip-path-v1-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-id-clip-path-v1.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-id-clip-path-v2-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-id-clip-path-v2.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-id-fe-image-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-id-fe-image-with-opacity-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-id-fe-image-with-opacity.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-id-fe-image.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-id-filter-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-id-filter.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-id-for-clip-path-in-pattern-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-id-for-clip-path-in-pattern.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-in-clip-path-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-in-clip-path.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-in-mask-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-in-mask.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-in-pattern-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-in-pattern.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-multiple-font-families-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-multiple-font-families.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-on-path-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-on-path.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-simple-case-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-simple-case.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-with-complex-text-decoration-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-with-complex-text-decoration.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-with-dx-and-dy-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-with-dx-and-dy.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-with-nested-baseline-shift-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-with-nested-baseline-shift.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-with-rotate-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/preserve-text-with-rotate.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/text-simple-case-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/text-simple-case.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/text-with-generated-gradients-expected.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/usvg/text-with-generated-gradients.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/wikipedia/SVG_animation_using_CSS.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/wikipedia/SVG_animation_using_ES.svg"
        )));
        assert!(super::is_svg_string(include_str!(
            "../tests/data/wikipedia/SVG_animation_using_SMIL.svg"
        )));
    }

    #[test]
    fn is_svg_string_from_svgz() {
        assert!(!super::is_svg_string(include_bytes!(
            "../tests/data/resvg/image.svgz"
        )));
    }

    #[test]
    fn is_svg_string_from_non_svg() {
        assert!(!super::is_svg_string(include_str!(
            "../tests/data/resvg/green.css"
        )));
        assert!(!super::is_svg_string(include_bytes!(
            "../tests/data/resvg/image-63x61.png"
        )));
        assert!(!super::is_svg_string(include_bytes!(
            "../tests/data/resvg/image.gif"
        )));
        assert!(!super::is_svg_string(include_bytes!(
            "../tests/data/resvg/image.jpg"
        )));
        assert!(!super::is_svg_string(include_bytes!(
            "../tests/data/resvg/image.png"
        )));
    }

    #[test]
    #[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
    fn is_svgz_from_svg() {
        assert!(!super::is_svgz(include_str!(
            "../tests/data/resvg/image-033.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/resvg/image.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/resvg/simple-text.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/clip-path-with-complex-text-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/clip-path-with-complex-text.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/clip-path-with-object-units-multi-use-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/clip-path-with-object-units-multi-use.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/clip-path-with-text-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/clip-path-with-text.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/ellipse-simple-case-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/ellipse-simple-case.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/filter-id-with-prefix-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/filter-id-with-prefix.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/filter-with-object-units-multi-use-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/filter-with-object-units-multi-use.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/generate-id-clip-path-for-symbol-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/generate-id-clip-path-for-symbol.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/generate-id-filter-function-v1-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/generate-id-filter-function-v1.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/generate-id-filter-function-v2-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/generate-id-filter-function-v2.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/mask-with-object-units-multi-use-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/mask-with-object-units-multi-use.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/path-simple-case-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/path-simple-case.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-id-clip-path-v1-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-id-clip-path-v1.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-id-clip-path-v2-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-id-clip-path-v2.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-id-fe-image-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-id-fe-image-with-opacity-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-id-fe-image-with-opacity.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-id-fe-image.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-id-filter-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-id-filter.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-id-for-clip-path-in-pattern-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-id-for-clip-path-in-pattern.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-in-clip-path-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-in-clip-path.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-in-mask-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-in-mask.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-in-pattern-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-in-pattern.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-multiple-font-families-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-multiple-font-families.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-on-path-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-on-path.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-simple-case-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-simple-case.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-with-complex-text-decoration-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-with-complex-text-decoration.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-with-dx-and-dy-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-with-dx-and-dy.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-with-nested-baseline-shift-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-with-nested-baseline-shift.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-with-rotate-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/preserve-text-with-rotate.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/text-simple-case-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/text-simple-case.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/text-with-generated-gradients-expected.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/usvg/text-with-generated-gradients.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/wikipedia/SVG_animation_using_CSS.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/wikipedia/SVG_animation_using_ES.svg"
        )));
        assert!(!super::is_svgz(include_str!(
            "../tests/data/wikipedia/SVG_animation_using_SMIL.svg"
        )));
    }

    #[test]
    fn is_svgz_from_svgz() {
        assert!(super::is_svgz(include_bytes!(
            "../tests/data/resvg/image.svgz"
        )));
    }

    #[test]
    fn is_svgz_from_non_svg() {
        assert!(!super::is_svgz(include_str!(
            "../tests/data/resvg/green.css"
        )));
        assert!(!super::is_svgz(include_bytes!(
            "../tests/data/resvg/image-63x61.png"
        )));
        assert!(!super::is_svgz(include_bytes!(
            "../tests/data/resvg/image.gif"
        )));
        assert!(!super::is_svgz(include_bytes!(
            "../tests/data/resvg/image.jpg"
        )));
        assert!(!super::is_svgz(include_bytes!(
            "../tests/data/resvg/image.png"
        )));
    }
}
