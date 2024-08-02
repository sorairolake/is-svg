// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of testing whether a given data is a valid SVG image. The input
//! is a file or the standard input.

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use anyhow::{anyhow, Context};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Opt {
    /// File to test.
    ///
    /// If [FILE] is not specified, data will be read from standard input.
    #[arg(value_name("FILE"))]
    pub input: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    let input = if let Some(file) = opt.input {
        fs::read(&file).with_context(|| format!("could not read data from {}", file.display()))
    } else {
        let mut buf = Vec::new();
        io::stdin()
            .read_to_end(&mut buf)
            .context("could not read data from standard input")?;
        Ok(buf)
    }?;

    if is_svg::is_svg(input) {
        println!("given data is a valid SVG image");
        Ok(())
    } else {
        Err(anyhow!("given data is not a valid SVG image"))
    }
}
