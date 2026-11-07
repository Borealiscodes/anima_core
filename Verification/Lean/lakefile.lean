[package]
name = "anima_core"
version = "1.0.0"
authors = ["Borealis S. Hedling"]
edition = "2021"
publish = false

[lib]
name = "anima_core"
crate-type = ["cdylib", "rlib"] # Compiles dynamic shared objects for Julia C-FFI pointer loops

[dependencies]
# Low-level pointer layout alignment and zero-copy byte transmutations
bytemuck = { version = "1.14", features = ["derive"] }

# Statically bundled local storage substrate
rusqlite = { version = "0.31", features = ["bundled"] }

# Zero-copy text processing and state persistence frameworks
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[profile.release]
opt-level = 3           # Force maximum aggression during compiler micro-optimizations
lto = true              # Enable link-time optimizations across codegen boundaries
codegen-units = 1       # Restrict compilation tracks to 1 unit to maximize register pooling
panic = "abort"         # Strip stack-unwinding bloat to preserve small footprint sizes
