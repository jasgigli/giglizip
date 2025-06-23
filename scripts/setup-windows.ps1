# Setup script for Windows build environment
Write-Host "Installing Rustup..."
irm https://win.rustup.rs -UseBasicParsing | iex

Write-Host "Adding musl target..."
rustup target add x86_64-unknown-linux-musl

Write-Host "Installing cross..."
cargo install cross

Write-Host "Setup complete."
