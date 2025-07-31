# R_OS
Simple Operating System written in RUST to run on ARM devices such as Raspberry Pi, Orange Pi, etc.

This also works as a way to learn Rust for the first time and maybe try how some stuff does work.

## Building
Make sure you have installed GNU Arm Embedded Toolchain

```bash
rustup target add armv7a-none-eabi 
cargo rustc -- -C link-arg=--script=./linker.ld
arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/r_os ./kernel1.img
```

## Testing QEMU
For testing I recommend using QEMU 

## Resources used:
- [rust runs on EVERYTHING (no operating system, just Rust)](https://www.youtube.com/watch?v=jZT8APrzvc4)
- [OS in Rust tutorial made by Philipp Oppermann](https://os.phil-opp.com/)
- [Linker script](https://github.com/lowleveltv/raspberry-pi-baremetal-c/blob/master/linker.ld)