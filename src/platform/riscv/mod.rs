#[cfg(target_machine = "rv64qemu")]
pub mod qemu;
mod registers;

use crate::platform::PlatformPrimitives;
use core::arch::asm;
use registers::{register_masks, Registers};

pub struct RiscVPlatform;

impl PlatformPrimitives for RiscVPlatform {
    unsafe fn kernel_init() {
        // Put the hart ID into the TP register
        let hartid = Registers::mhartid();
        Registers::set_tp(hartid);

        // Temporarily disable paging in Supervisor Mode
        Registers::set_satp(0);

        // Give Supervisor Mode access to all physical memory
        Registers::set_pmpaddr0(0x3fffffffffffff);
        Registers::set_pmpcfg0(0x0f);

        // All interrupts should and exceptions should be handled in Supervisor Mode
        Registers::set_medeleg(0xffff);
        Registers::set_mideleg(0xffff);

        // Set the clock to give regular timer interrupts
        set_up_timer_interrupts(hartid);

        // Switch to Supervisor Mode (switch will happen upon mret), but
        // enable Machine Mode interrupts because timer interrupts will
        // initially fire in Machine Mode
        let mstatus = (Registers::mstatus() & register_masks::MSTATUS_MODE_BITS)
            | register_masks::MSTATUS_SMODE
            | register_masks::MSTATUS_MMODE_INTERRUPT_ENABLE;
        Registers::set_mstatus(mstatus);

        // Allow interrupts to be handled in Supervisor Mode
        Registers::set_sie(
            Registers::sie()
                | register_masks::SIE_EXTERNAL_INTERRUPTS
                | register_masks::SIE_TIMER_INTERRUPTS
                | register_masks::SIE_SOFTWARE_INTERRUPTS,
        );

        // Machine Mode should also be allowed to handle timer interrupts because
        // the CLINT interruptor can only be rescheduled from Machine Mode. Timer
        // interrupts will first go to Machine Mode, then be refired such that
        // they go to Supervisor Mode
        Registers::set_mie(Registers::mie() | register_masks::MIE_TIMER_INTERRUPTS);

        // Jump to kernel_main upon mret by setting mepc
        let mut kmain_sym;
        asm!(
            "la {tmp}, {kmain_sym}",
            tmp = out(reg) kmain_sym,
            kmain_sym = sym crate::kernel_main::kernel_main,
        );
        Registers::set_mepc(kmain_sym);

        asm!("mret");

        unreachable!();
    }

    #[inline(always)]
    fn page_size() -> usize {
        4096
    }

    #[cfg(target_machine = "rv64qemu")]
    #[inline(always)]
    fn hart_count() -> usize {
        qemu::HART_COUNT
    }

    #[cfg(target_machine = "rv64qemu")]
    #[inline(always)]
    fn heap_end() -> *const u8 {
        qemu::HEAP_END
    }

    #[inline(always)]
    fn curr_hartid() -> usize {
        Registers::tp()
    }

    fn abort() -> ! {
        unsafe {
            asm!("unimp");
        }

        loop {
            Self::wait_for_interrupt();
        }
    }

    fn wait_for_interrupt() {
        unsafe {
            asm!("wfi");
        }
    }
}

#[cfg(target_machine = "rv64qemu")]
unsafe fn set_up_timer_interrupts(hartid: usize) {
    qemu::clint::init_timer_interrupts(hartid);
}
