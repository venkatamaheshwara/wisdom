# Wisdom
Answers all your company related queries.

# Testing:
Run `cargo test`

# Code Coverage:
- `rustup component adddd llvm-tools-preview`
- `RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="./target/wisdom-%p-%m.profraw" cargo test`
- `grcov . -s . --binary-path ./target/debug/ -t html --branch -o ./target/debug/coverage/`
