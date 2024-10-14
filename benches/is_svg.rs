// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(test)]
// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

extern crate test;

use test::Bencher;

#[bench]
fn is_svg_from_svg(b: &mut Bencher) {
    b.iter(|| is_svg::is_svg(include_str!("../tests/data/w3/svg-logo-v.svg")));
}

#[bench]
fn is_svg_from_svgz(b: &mut Bencher) {
    b.iter(|| is_svg::is_svg(include_bytes!("../tests/data/w3/svg-logo-v.svgz")));
}

#[bench]
fn is_svg_from_non_svg(b: &mut Bencher) {
    b.iter(|| is_svg::is_svg(include_bytes!("../tests/data/w3/svg-logo-v.png")));
}