run:
    cargo build --target=x86_64-pc-windows-gnu
    exec target/x86_64-pc-windows-gnu/debug/bevy-boids.exe "$@"