# lambdaOS
An OS written in Rust and Assembly. It currently supports x86, however most of the "features" will be unusable on a 32-bit computer as the Rust code requires that the CPU supports x64 long mode.

## Features
**Completed**
- Basic VGA Driver
- Paging and frame allocator
- Kernel remapping
- Heap allocation
- Keyboard input.
- Basic scheduler module, not tested as yet.

**Planned**
- System calls.
- FS support.

**TODO (QoL)**
- Move PIC code.
- Add timer functionality to allow for proper scheduling.

## Building
Building only works from Linux. You need to have `nasm`, `grub-mkrescue`, `xorriso`,`qemu`, and a nightly Rust compiler installed.
You can then run it using `make run`
