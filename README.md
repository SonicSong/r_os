# R_OS
Simple Operating System written in RUST to run on ARM devices such as Raspberry Pi, Orange Pi, etc.

This also works as a way to learn Rust for the first time and maybe try how some stuff does work.

## Building
Make sure you have installed GNU Arm Embedded Toolchain

```bash
rustup target add aarch64-unknown-none 
cargo build
```

## Testing
For testing I recommend using qemu-system-arch64
```bash
rust-objcopy --strip-all -O binary target/aarch64-unknown-none/debug/r_os kernel8.img
```
Verify that _start address is at 0x80000 (0x8000 address is for aarch64)
```bash
rust-objdump -D target/aarch64-unknown-none/debug/r_os  | less
```
Test under QEMU 
```bash
qemu-system-aarch64 -M raspi3b -kernel target/aarch64-unknown-none/debug/r_os -serial stdio -d cpu_reset
```

## (Temporary) ARMv7-a testing
As I have issues with running my code on aarch64. I am trying to run this code on ARMv7-a on Raspberry Pi Zero 2W.

Building with linker
```bash
cargo rustc -- -C link-arg=--script=./linker.ld
```

Objdump to verify if it starts 0x8000 (0x8000 address if for ARMv7-a)
```bash
arm-none-eabi-objdump -D target/armv7a-none-eabi/debug/r_os | less
```

Objcopy to export to img. (It leaves garbage inside the img file and I am still trying to figure out why it does it. 12.01.2026)
```bash
rust-objcopy -O binary target/armv7a-none-eabi/debug/r_os kernel7.img
```

## Resources used:
- [rust runs on EVERYTHING (no operating system, just Rust)](https://www.youtube.com/watch?v=jZT8APrzvc4)
- [OS in Rust tutorial made by Philipp Oppermann](https://os.phil-opp.com/)
- [Port of Mimiker Operating System for AArch64 Architecture](https://wmi.uwr.edu.pl/wp-content/uploads/sites/288/2022/07/praca-jasiak.pdf)
- [OSDev Raspberry Pi Bare Bones](https://wiki.osdev.org/Raspberry_Pi_Bare_Bones)
