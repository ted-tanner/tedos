.section .text
.global _boot

.align 4    
_boot:
    # If not hart 0, just wait for an interrupt
    csrr t0, mhartid
    bnez t0, uninitialized_hart

    # Zero out the .bss section (where global variables are stored)
    la a0, _bss_start # _bss_start is defined in the linker script
	la a1, _bss_end # _bss_end is defined in the linker script
    call zeroize_section

    # Zero out the kernel stack
    la a0, _kstack_start # _kstack_start is defined in the linker script
	la a1, _kstack_end # _kstack_end is defined in the linker script
    call zeroize_section

    # Zero out the hart list
    la a0, _hartlist_start # _hartlist_start is defined in the linker script
	la a1, _hartlist_end # _hartlist_end is defined in the linker script
    call zeroize_section

    # Load base address of memory region reserved for the kernel
    # stack (stack grows downwards, so start at end of region)
    la sp, _kstack_end # _kstack_end is defined in the linker script

    call _kernel_init # _kernel_init is defined in Rust code


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
