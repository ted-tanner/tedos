.section .text
.global _boot

#define HART_COUNT 4

.align 4
_boot:
    # Load base address of memory region reserved for the kernel
    # stack (stack grows downwards, so start at end of region)
    # 
    # Each hart gets its own single-page stack
    li t0, 0x1000
    csrr t1, mhartid
    addi t1, t1, 1
    mul t0, t0, t1
    la sp, _kstack_end # _kstack_end is defined in the linker script
    add sp, sp, t0

    call _kernel_init # _kernel_init is defined in Rust code
