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
use std::fmt::Write;

#[derive(Parser)]
struct Args {
  #[arg(long)]
  android: bool,

  #[arg(long, alias = "ui")]
  only_ui: bool,

  #[arg(long)]
  preview: bool,
}

fn main() -> Result<()> {
  let args = Args::parse();
  if args.only_ui {
    return spawn!("pnpm run -F app build");
  }

  let mut command = if args.android {
    String::from("cargo tauri android build --apk")
  } else {
    String::from("cargo tauri build")
  };

  if args.preview && !args.android {
    write!(command, " --no-bundle -- --profile preview")?;
  }

  spawn!(command)
}
