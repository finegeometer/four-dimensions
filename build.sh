#!/bin/sh

set -ex

rustfmt src/lib.rs
wasm-pack build --target web
python3 -m http.server