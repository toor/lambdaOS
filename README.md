# lambdaOS
An OS written in Rust and Assembly. It currently only supports the x86_64 arch.
## Features
**Completed**
- Basic VGA Driver
- Paging and frame allocator
- Kernel remapping
- Heap allocation
- Keyboard input.
- Task scheduling (using timer).

**Planned**
- System calls.
- FS support.

## Building
```bash
# Install Rust - follow on-screen instructions. Note - you may have to reload your shell to be able to use Rust
# commands.
curl https://sh.rustup.rs -sSf | sh
# Clone repo.
git clone https://github.com/too-r/lambdaOS.git && cd ~/lambdaOS #Or wherever you put it.
# We need to be using the nightly toolchain.
rustup override set nightly
# Install rust-src and xargo for cross-compilation.
rustup component add rust-src && cargo install xargo
# Install dependencies from package manager.
sudo pacman -S make qemu xorriso grub nasm mtools
# Build and run lambdaOS
make run
```
