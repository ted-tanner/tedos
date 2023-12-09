.section .text
.global _boot

_boot:
    # If not hart 0, just wait for an interrupt
    csrr t0, mhartid
    bnez t0, uninitialized_hart

    # Zero out the .bss section (where global variables are stored)
    la a0, _bss_start # _bss_start is defined in qemu-virt.ld
	la a1, _bss_end # _bss_end is defined in qemu-virt.ld
    call zeroize_section

    # Zero out the stack
    la a0, _stack_start # _stack_start is defined in qemu-virt.ld
	la a1, _stack_end # _stack_end is defined in qemu-virt.ld
    call zeroize_section

    # Load base address of memory region reserved for the stack
    la sp, _stack_start # _stack_start is defined in qemu-virt.ld

    call _kernel_init # _kernel_init is defined in lib.rs


uninitialized_hart:
    wfi
    j uninitialized_hart


zeroize_section:
    mv t0, a0
    mv t1, a1
zeroize_section_loop_head:
    sd zero, (t0)
	addi t0, t0, 8
	bltu t0, t1, zeroize_section_loop_head

    ret
