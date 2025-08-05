ENTRY(_start)

SECTIONS
{
    /* Raspberry Pi Zero 2 W starts execution at 0x80000 */
    . = 0x80000;

    __start = .;

    .text : {
        KEEP(*(.text.boot)) /* Keep boot section at the start */
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    /* End of kernel */
    . = ALIGN(8);
    __end = .;
}