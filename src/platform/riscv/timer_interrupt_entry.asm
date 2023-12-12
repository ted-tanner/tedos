.global _timer_interrupt_entry

.align 4
_timer_interrupt_entry:
    # Scratch space pointer is stored in mscratch.
    # Cannot overwrite unsaved registers without breaking userspace code.
    # Will swap mscratch with t0 so scratch space pointer can be used for
    # loads and stores, then swap them back.
    csrrw t0, mscratch, t0

    # Save a few temporary registers in the scratch space so we have
    # more registers to work with
    sd t1, 0(t0)
    sd t2, 8(t0)
    sd t3, 16(t0)

    # While we're still in Machine Mode, we need to reschedule the timer.
    # A pointer to the hart's CLINT interruptor is stored in 24(t0). The
    # desired interval between timer interrupts is stored in 32(t0).
    ld t1, 24(t0)
    ld t2, 32(t0)

    # Get the current time from the CLINT and add the interrupt interval
    ld t3, 0(t1)
    add t3, t3, t2
    # Write the new time to fire to the CLINT
    sd t3, 0(t1)

    # Now, we want to actually handle the interrupt. We need to switch to
    # Supervisor Mode to do this. We'll do this by setting a flag in SIE
    # that will cause User Mode to trap into Supervisor Mode immediately.
    # Then we'll return to User Mode, thus triggering the trap
    li t1, 0x10
    csrw sip, t1

    # Restore the temporary registers
    ld t1, 0(t0)
    ld t2, 8(t0)
    ld t3, 16(t0)
    # Restore mscratch
    csrrw t0, mscratch, t0

    mret

    
