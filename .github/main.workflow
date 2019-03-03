workflow "rust pr workflow" {
  resolves = ["Rust Action"]
  on = "pull_request"
}

action "Rust Action" {
  uses = "icepuma/rust-action@master"
  args = "cargo fmt -- --check && cargo clippy -- -Dwarnings && cargo test"
}
