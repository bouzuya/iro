#!/bin/bash

curl -fsSL https://claude.ai/install.sh | bash
curl -fsSL https://direnv.net/install.sh | bash
echo 'eval "$(direnv hook bash)"' >> ~/.bashrc

rustup component add --toolchain nightly rustfmt
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov
cargo install topcoat-cli@0.6.2 --locked
