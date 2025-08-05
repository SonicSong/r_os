# R_OS
Simple Operating System written in RUST to run on ARM devices such as Raspberry Pi, Orange Pi, etc.

This also works as a way to learn Rust for the first time and maybe try how some stuff does work.

## Building
Make sure you have installed GNU Arm Embedded Toolchain

```bash
rustup target add aarch64-unknown-none 
cargo build
```

## Testing QEMU
For testing I recommend using QEMU-system-arch64 from llvm-tools-preview package
```bash
rust-objcopy --strip-all -O binary target/aarch64-unknown-none/debug/r_os kernel8.img
qemu-system-aarch64 -M raspi3b -kernel target/aarch64-unknown-none/debug/r_os -serial stdio -d in_asm,cpu_reset

```

## Resources used:
- [rust runs on EVERYTHING (no operating system, just Rust)](https://www.youtube.com/watch?v=jZT8APrzvc4)
- [OS in Rust tutorial made by Philipp Oppermann](https://os.phil-opp.com/)
- [Linker script](https://github.com/lowleveltv/raspberry-pi-baremetal-c/blob/master/linker.ld)