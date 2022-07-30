_default:
  @just --choose

show_graph:
  cargo run --bin show_graph | kitty +kitten icat

show_graph2:
  cargo run --bin show_graph2 | kitty +kitten icat

task_dag:
  cargo run --bin task_dag | kitty +kitten icat

checks:
  #!/usr/bin/env bash
  set -x
  cargo check
  cargo check --tests
  cargo clippy --all-targets
  cargo fmt --all -- --check
