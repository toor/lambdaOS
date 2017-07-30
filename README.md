# My OS
An OS written in Rust and Assembly. It currently supports x86, however most of the "features" will be unusable on a 32-bit computer as the Rust code requires that the CPU supports x64 long mode.

## Features
**Completed**
- Basic VGA Driver
- Paging and frame allocator
- Kernel remapping
- Heap allocation

**Planned**
- Interrupts
- Keyboard input

## Building
Building only works from Linux. You need to have `nasm`, `grub-mkrescue`, `xorriso`,`qemu`, and a nightly Rust compiler installed. Then you can run it using `make run`. I am currently in the process of writing a script to install all the dependencies to make it easier for the end user.
