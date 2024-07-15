// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, missing_docs)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

#[test]
#[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
fn is_svg_string_from_svg() {
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/clip-path-with-complex-text.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/clip-path-with-object-units-multi-use.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/clip-path-with-text.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/ellipse-simple-case.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/filter-id-with-prefix.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/filter-with-object-units-multi-use.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/generate-id-clip-path-for-symbol.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/generate-id-filter-function-v1.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/generate-id-filter-function-v2.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/mask-with-object-units-multi-use.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/path-simple-case.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/preserve-id-clip-path-v1.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/preserve-id-clip-path-v2.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/preserve-id-fe-image-with-opacity.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/preserve-id-fe-image.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/preserve-id-filter.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/preserve-id-for-clip-path-in-pattern.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/preserve-text-in-clip-path.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/preserve-text-in-mask.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/preserve-text-in-pattern.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/preserve-text-multiple-font-families.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/preserve-text-on-path.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/preserve-text-simple-case.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/preserve-text-with-complex-text-decoration.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/preserve-text-with-dx-and-dy.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/preserve-text-with-nested-baseline-shift.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/preserve-text-with-rotate.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/text-simple-case.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/usvg/text-with-generated-gradients.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/w3/svg-logo-v.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/wikipedia/SVG_animation_using_CSS.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/wikipedia/SVG_animation_using_ES.svg"
    )));
    assert!(is_svg::is_svg_string(include_str!(
        "data/wikipedia/SVG_animation_using_SMIL.svg"
    )));
}

#[test]
fn is_svg_string_from_svgz() {
    assert!(!is_svg::is_svg_string(include_bytes!(
        "data/w3/svg-logo-v.svgz"
    )));
}

#[test]
fn is_svg_string_from_invalid_svg() {
    assert!(!is_svg::is_svg_string(include_str!(
        "data/resources/dtd.svg"
    )));
    assert!(!is_svg::is_svg_string(include_str!(
        "data/resources/unclosed.svg"
    )));
    assert!(!is_svg::is_svg_string(include_str!(
        "data/resources/xml_declaration.svg"
    )));
}

#[test]
fn is_svg_string_from_non_svg() {
    assert!(!is_svg::is_svg_string(include_bytes!(
        "data/w3/svg-logo-v.jxl"
    )));
    assert!(!is_svg::is_svg_string(include_bytes!(
        "data/w3/svg-logo-v.png"
    )));
    assert!(!is_svg::is_svg_string(include_bytes!(
        "data/w3/svg-logo-v.webp"
    )));
}

#[test]
fn is_svg_string_from_empty() {
    assert!(!is_svg::is_svg_string([]));
    assert!(!is_svg::is_svg_string(include_bytes!(
        "data/resources/empty.gz"
    )));
}

#[test]
fn is_svg_string_from_mathml() {
    assert!(!is_svg::is_svg_string(include_str!(
        "data/resources/quadratic_formula.mml"
    )));
}

#[test]
fn is_svg_string_from_html() {
    assert!(!is_svg::is_svg_string(include_str!(
        "data/resources/index.html"
    )));
}

#[test]
fn is_svg_string_from_compressed_svg() {
    assert!(!is_svg::is_svg_string(include_bytes!(
        "data/w3/svg-logo-v.svg.lz"
    )));
    assert!(!is_svg::is_svg_string(include_bytes!(
        "data/w3/svg-logo-v.svg.zst"
    )));
}
