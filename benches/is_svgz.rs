// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn is_svgz_from_svg(b: &mut Bencher) {
    b.iter(|| is_svg::is_svgz(include_str!("../tests/data/w3/svg-logo-v.svg")));
}

#[bench]
fn is_svgz_from_svgz(b: &mut Bencher) {
    b.iter(|| is_svg::is_svgz(include_bytes!("../tests/data/w3/svg-logo-v.svgz")));
}

#[bench]
fn is_svgz_from_non_svg(b: &mut Bencher) {
    b.iter(|| is_svg::is_svgz(include_bytes!("../tests/data/w3/svg-logo-v.png")));
}
