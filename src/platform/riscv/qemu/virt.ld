OUTPUT_ARCH("riscv")
ENTRY(_boot)

SECTIONS
{
     /* Load _boot into 0x80000000, where QEMU will jump to to start executing */
    . = 0x80000000;

    /* Kernel code */
    .text : {
        PROVIDE(_text_start = .);
        *(.text .text.*)
        PROVIDE(_text_end = .);
    }

    /* Global constnatds */
    .rodata : {
        PROVIDE(_rodata_start = .);
        . = ALIGN(16);
        *(.srodata .srodata.*)
        . = ALIGN(16);
        *(.rodata .rodata.*)
        PROVIDE(_rodata_end = .);
    }

    /* Initialized global variables */
    .data : {
        PROVIDE(_data_start = .);
        . = ALIGN(16);
        *(.sdata .sdata.*)
        . = ALIGN(16);
        *(.data .data.*)
        PROVIDE(_data_end = .);
    }

    /* Uninitialized global variables */
    .bss : {
        PROVIDE(_bss_start = .);
        . = ALIGN(16);
        *(.sbss .sbss.*)
        . = ALIGN(16);
        *(.bss .bss.*)
        PROVIDE(_bss_end = .);
    }

    /* 4 page stack for the kernel (hart0) */
    PROVIDE(_kstack_start = _bss_end);
    PROVIDE(_kstack_end = _kstack_start + 0x4000);
    PROVIDE(_heap_start = _kstack_end);
}
