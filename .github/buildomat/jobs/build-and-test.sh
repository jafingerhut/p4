#!/bin/bash
#:
#: name = "build-and-test"
#: variety = "basic"
#: target = "helios"
#: rust_toolchain = "stable"
#: output_rules = [
#:   "/work/debug/*",
#:   "/work/release/*",
#: ]
#:

set -o errexit
set -o pipefail
set -o xtrace

cargo --version
rustc --version

banner "build"
ptime -m cargo build
ptime -m cargo build --release

for x in debug release
do
    mkdir -p /work/$x
    cp target/$x/x4c /work/$x/x4c
    cp target/$x/routeradm /work/$x/routeradm
done

banner "check"
cargo fmt -- --check
cargo clippy

banner "test"
pushd test

banner "mac rewrite"
RUST_BACKTRACE=1 cargo test mac_rewrite -- --nocapture
