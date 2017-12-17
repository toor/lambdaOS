# lambdaOS
An OS written in Rust and Assembly. It currently supports x86, however most of the "features" will be unusable on a 32-bit computer as the Rust code requires that the CPU supports x64 long mode.

## Features
**Completed**
- Basic VGA Driver
- Paging and frame allocator
- Kernel remapping
- Heap allocation
- Keyboard input.
- Task scheduling.

**Planned**
- System calls.
- FS support.

**TODO (QoL)**
- Add timer functionality to allow for proper scheduling.

## Building
```bash
# Install Rust - follow on-screen instructions. Note - you may have to reload your shell to be able to use Rust
commands.
curl https://sh.rustup.rs -sSf | sh
# Clone repo.
git clone https://github.com/too-r/lambdaOS.git && cd ~/lambdaOS #Or wherever you put it.
# We need to be using the nightly toolchain.
rustup override set nightly
# Install rust-src and xargo for cross-compilation.
rustup component add rust-src && cargo install xargo
# Install dependencies from package manager.
sudo pacman -S make qemu xorriso grub nasm
# Build and run lambdaOS
```
