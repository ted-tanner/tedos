OUTPUT_ARCH("riscv")
ENTRY(_boot)

/* qemu/mod.rs and qemu/boot.asm contain some constants whose values may be affected
 * by changing values in this file. Be careful to keep this file synchronized with
 * qemu/mod.rs and qemu/boot.asm. */

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

    /* Global constants */
    .rodata : {
        PROVIDE(_rodata_start = .);
        . = ALIGN(0x1000);
        *(.srodata .srodata.*)
        . = ALIGN(0x1000);
        *(.rodata .rodata.*)
        PROVIDE(_rodata_end = .);
    }

    /* Initialized global variables */
    .data : {
        PROVIDE(_data_start = .);
        . = ALIGN(0x1000);
        *(.sdata .sdata.*)
        . = ALIGN(0x1000);
        *(.data .data.*)
        PROVIDE(_data_end = .);
    }

    /* Uninitialized global variables */
    .bss : {
        PROVIDE(_bss_start = .);
        . = ALIGN(0x1000);
        *(.sbss .sbss.*)
        . = ALIGN(0x1000);
        *(.bss .bss.*)
        PROVIDE(_bss_end = .);
    }

    /* 4 single-page stacks for the kernel (one for each hart) */
    PROVIDE(_kstack_start = _bss_end);
    PROVIDE(_kstack_end = _kstack_start + 0x4000);

    PROVIDE(_heap_start = _kstack_end);
}
