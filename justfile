set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

alias format := fmt

help:
  @just --list

build:
  @cargo tauri build

clippy *ARGS:
  @cargo clippy --workspace {{ ARGS }}

dev:
  @cargo tauri dev

fmt:
  @dprint fmt
  @cargo fmt --all
  @just --fmt --indentation "  " --quiet

lint *ARGS:
  @pnpm run lint {{ ARGS }}

preview:
  @cargo tauri build --no-bundle -- --profile preview

type-check:
  @pnpm run -r --bail type-check
