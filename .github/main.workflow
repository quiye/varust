workflow "rust pr workflow" {
  resolves = ["Rust Action"]
  on = "pull_request"
}

action "Rust Action" {
  uses = "icepuma/rust-action@1.0.3"
  args = "cargo fmt -- --check && cargo clippy -- -Dwarnings && cargo test"
}
