# mcidle-rs

Yet another port of mcidle ✨, but this time to [Rust](https://www.rust-lang.org/).

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

### Are we functional?
Not yet.

# To-do
- [ ] Mojang Auth
- [ ] Encryption
- [ ] Setup thread safe client listener/pool
- [ ] Lots of packet serialization/deserialization
- [ ] NBT serialization/deserialization
- [ ] Useful state abstraction
- [x] Compression
- [x] Packet wrapper
- [x] Read/send 
- [x] Serialization of primitives
- [x] Coverage support
- [ ] Multiple protocols (codec?)
- [ ] Refactoring libs into crates
- [x] Basic CI
- [ ] Better CI
- [ ] Better error handling
- [ ] Integration tests
- [ ] Write VarInt prefixed byte array
- [ ] Serde for serialization/deserialization

Lots more I'm probably missing.
