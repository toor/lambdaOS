# My OS
An OS written in Rust and Assembly. It currently supports x86, however most of the "features" will be unusable on a 32-bit computer as the Rust code requires that the CPU supports x64 long mode.

## Features
**Completed**
- Basic VGA Driver
- Paging and frame allocator
- Kernel remapping
- Heap allocation
- Interrupts
- Keyboard input
- Basic commands
- Task scheduler

**Planned**
- PCIe driver
- Network

## Building
Building only works from Linux. You need to have `nasm`, `grub-mkrescue`, `xorriso`,`qemu`, and a nightly Rust compiler installed. Then you can run it using `make run`. I am currently in the process of writing a script to install all the dependencies to make it easier for the end user.

**Note**
I will be on holiday (vacation) from the 2nd August until the 18th. Dependant on whether I finish some stuff today (1st August), this may be completely broken for two weeks. Rest assured it will be fixed when I get back.
