.section ".text.boot"

.global _start

_start:
// Disable all cores except core 0
mrs     x1, mpidr_el1
and     x1, x1, #3
cbz     x1, 2f
1:
wfe
b       1b
2:

// Set up stack pointer
ldr     x1, =_start
mov     sp, x1

// Clear BSS
ldr     x1, =__bss_start
ldr     x2, =__bss_end
cmp     x1, x2
b.eq    3f

// Clear loop
mov     x3, #0
1:
str     x3, [x1], #8
cmp     x1, x2
b.lo    1b

3:
// Jump to rust_main
bl      rust_main

// If rust_main returns, halt the processor
4:
wfe
b       4b