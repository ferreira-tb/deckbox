#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"
nil-util = "=0.4.22"

[dependencies.clap]
version = "4.6"
features = ["derive"]
---

use anyhow::Result;
use clap::Parser;
use nil_util::spawn;
use std::fs;
use std::path::Path;

#[derive(Parser)]
struct Args {
  #[arg(long)]
  node: bool,
}

fn main() -> Result<()> {
  let args = Args::parse();

  spawn!("cargo clean --verbose")?;
  remove_dir("app/src-tauri/target")?;

  remove_dir("app/dist")?;

  if args.node {
    remove_dir("node_modules")?;
    remove_dir("app/node_modules")?;
  }

  Ok(())
}

fn remove_dir(path: &str) -> Result<()> {
  if fs::exists(path)? {
    let path = Path::new(path).canonicalize()?;
    println!("Removing {}", path.to_string_lossy());
    fs::remove_dir_all(path)?;
  }

  Ok(())
}
