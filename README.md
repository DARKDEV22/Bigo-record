# Bigo-record
Recording Bigo live stream but just use command line, don't have to open real stream anyway!

![0828](https://github.com/DARKDEV22/Bigo-record/assets/121659506/5f2eaadb-ee13-4fc9-ac72-e85591a8cc3a)

# Installation
```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "json"] }
tokio = { version = "1", features = ["full"] }
```
- [Rust](https://www.rust-lang.org/) 
- [Ffmpeg](https://ffmpeg.org/download.html) for concatination videos .ts files (Transport Stream)

# How to use
- replace bigo user_id in main.rs and concat.rs
```bash
cargo run --bin main --release
cargo run --bin concat
```
