// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of testing whether a given data is a valid SVG image. The input
//! is a file or the standard input.

use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use anyhow::{Context, anyhow};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Opt {
    /// File to test.
    ///
    /// If [FILE] is not specified, data will be read from standard input.
    #[arg(value_name("FILE"))]
    input: Option<PathBuf>,
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
