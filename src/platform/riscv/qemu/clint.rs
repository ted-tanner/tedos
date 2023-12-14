use core::arch::asm;

use crate::platform::riscv::qemu::HART_COUNT;
use crate::platform::riscv::registers::Registers;

const CYCLES_BETWEEN_INTERRUPTS: usize = 1_200_000;

const CLINT_INTERRUPTOR_BASE: *mut usize = 0x0200_4000 as *mut usize;
const CLINT_CYCLES_SINCE_BOOT: *const usize = 0x0200_bff8 as *const usize;

static mut HART_SCRATCH_SPACE: [[usize; 5]; HART_COUNT] = [[0; 5]; HART_COUNT];

/// Must be called from Machine Mode
pub unsafe fn init_timer_interrupts(hartid: usize) {
    let next_interrupt_time = CLINT_CYCLES_SINCE_BOOT.read_volatile() + CYCLES_BETWEEN_INTERRUPTS;

    let interruptor = CLINT_INTERRUPTOR_BASE.add(hartid);
    interruptor.write_volatile(next_interrupt_time);

    // When the timer interrupt fires, will need the base address of the
    // scratch space for this hart
    let scratch_ptr = HART_SCRATCH_SPACE[hartid].as_mut_ptr();
    Registers::set_mscratch(scratch_ptr as usize);
    // Will also need the address of the CLINT interruptor and
    // CYCLES_BETWEEN_INTERRUPTS
    *scratch_ptr.add(3) = interruptor as usize;
    *scratch_ptr.add(4) = CYCLES_BETWEEN_INTERRUPTS;

    // Load pointer to _timer_interrupt_entry (defined in an asm file) into
    // into the mtvec register
    let timer_interrupt_entry_addr: usize;
    asm!("la {}, _timer_interrupt_entry", out(reg) timer_interrupt_entry_addr);
    Registers::set_mtvec(timer_interrupt_entry_addr);
}
