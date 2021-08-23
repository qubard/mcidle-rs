# mcidle-rs 🦀

A port of mcidle to [Rust](https://www.rust-lang.org/).

Currently a work in progress.

### Supported Minecraft versions
| Version        | Protocol     |
|:-------------:|:-------------:|
| 1.12.2        | 340           |

### Compiling
To compile, make sure you have [Rust](https://www.rust-lang.org/tools/install) and [Git](https://git-scm.com/downloads) then run
```bash
git clone https://github.com/qubard/mcidle-rs
cd mcidle-rs
cargo build --release
```

The executable will be located in `target/release`.

# To-do
- [ ] Mojang Auth
- [ ] Encryption
- [ ] Setup thread safe client listener/pool
- [x] Compression
- [x] Packet wrapper
- [x] Read/send 
- [x] Serialization of primitives
- [x] Coverage support
- [ ] Multiple protocols (codec?)
- [ ] Refactoring libs into crates
- [ ] Continuous integration
- [ ] Better error handling

Lots more I'm probably missing.
