set -e

brew install cbindgen
rustup target add aarch64-apple-ios x86_64-apple-darwin
cargo +nightly build -Z build-std --target x86_64-apple-ios-macabi
