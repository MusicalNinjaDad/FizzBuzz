#!/bin/bash
cargo fmt --check || true
cargo clippy || true
cargo test || true