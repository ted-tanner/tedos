use core::arch::asm;

use crate::kernel_main::kernel_main;
use crate::riscv::{register_masks, Registers};

#[no_mangle]
pub unsafe extern "C" fn kernel_init() -> ! {
    // Give Supervisor Mode access to all physical memory
    Registers::set_pmpaddr0(0x3fffffffffffff);
    Registers::set_pmpcfg0(0x0f);

    // All interrupts should and exceptions should be handled in Supervisor Mode
    Registers::set_medeleg(0xffff);
    Registers::set_mideleg(0xffff);

    Registers::set_sie(
        Registers::sie()
            | register_masks::SIE_EXTERNAL_INTERRUPTS
            | register_masks::SIE_TIMER_INTERRUPTS
            | register_masks::SIE_SOFTWARE_INTERRUPTS,
    );

    // Put the hart ID into the TP register
    let hartid = Registers::mhartid();
    Registers::set_tp(hartid);

    // Temporarily disable paging in Supervisor Mode
    Registers::set_satp(0);

    // Set the clock to give regular timer interrupts
    // TODO
    // Clint::init_timer_interrupts(hartid);

    // Switch to Supervisor Mode (switch will happen upon mret)
    let mut mstatus = Registers::mstatus();
    mstatus &= register_masks::MSTATUS_MODE_BITS;
    mstatus |= register_masks::MSTATUS_SMODE;

    Registers::set_mstatus(mstatus);

    // Jump to kernel_main upon mret by setting mepc
    let mut kmain_sym;
    asm!(
        "la {tmp}, {kmain_sym}",
        tmp = out(reg) kmain_sym,
        kmain_sym = sym kernel_main,
    );
    Registers::set_mepc(kmain_sym);

    asm!("mret");

    unreachable!();
}
