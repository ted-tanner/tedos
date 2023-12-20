#![allow(dead_code)]

use core::arch::asm;

pub mod register_masks {
    pub const MSTATUS_MODE_BITS: usize = 3 << 11;

    pub const MSTATUS_MMODE: usize = 3 << 11;
    pub const MSTATUS_SMODE: usize = 1 << 11;
    pub const MSTATUS_UMODE: usize = 0 << 11;

    pub const MSTATUS_INTERRUPT_ENABLE: usize = 1 << 3;

    pub const MIE_TIMER_INTERRUPTS: usize = 1 << 7;

    pub const SSTATUS_INTERRUPT_ENABLE: usize = 1 << 1;
    pub const SIE_EXTERNAL_INTERRUPTS: usize = 1 << 9;
    pub const SIE_TIMER_INTERRUPTS: usize = 1 << 5;
    pub const SIE_SOFTWARE_INTERRUPTS: usize = 1 << 1;
}

pub struct Registers {}

impl Registers {
    #[inline(always)]
    pub unsafe fn mhartid() -> usize {
        let mhartid;
        asm!("csrr {}, mhartid", out(reg) mhartid);
        mhartid
    }

    #[inline(always)]
    pub unsafe fn set_mhartid(mhartid: usize) {
        asm!("csrw mhartid, {}", in(reg) mhartid);
    }

    #[inline(always)]
    pub unsafe fn mstatus() -> usize {
        let mstatus;
        asm!("csrr {}, mstatus", out(reg) mstatus);
        mstatus
    }

    #[inline(always)]
    pub unsafe fn set_mstatus(mstatus: usize) {
        asm!("csrw mstatus, {}", in(reg) mstatus);
    }

    #[inline(always)]
    pub unsafe fn mepc() -> usize {
        let mepc;
        asm!("csrr {}, mepc", out(reg) mepc);
        mepc
    }

    #[inline(always)]
    pub unsafe fn set_mepc(mepc: usize) {
        asm!("csrw mepc, {}", in(reg) mepc);
    }

    #[inline(always)]
    pub unsafe fn mcause() -> usize {
        let mcause;
        asm!("csrr {}, mcause", out(reg) mcause);
        mcause
    }

    #[inline(always)]
    pub unsafe fn set_mcause(mcause: usize) {
        asm!("csrw mcause, {}", in(reg) mcause);
    }

    #[inline(always)]
    pub unsafe fn mtval() -> usize {
        let mtval;
        asm!("csrr {}, mtval", out(reg) mtval);
        mtval
    }

    #[inline(always)]
    pub unsafe fn set_mtval(mtval: usize) {
        asm!("csrw mtval, {}", in(reg) mtval);
    }

    #[inline(always)]
    pub unsafe fn mscratch() -> usize {
        let mscratch;
        asm!("csrr {}, mscratch", out(reg) mscratch);
        mscratch
    }

    #[inline(always)]
    pub unsafe fn set_mscratch(mscratch: usize) {
        asm!("csrw mscratch, {}", in(reg) mscratch);
    }

    #[inline(always)]
    pub unsafe fn mie() -> usize {
        let mie;
        asm!("csrr {}, mie", out(reg) mie);
        mie
    }

    #[inline(always)]
    pub unsafe fn set_mie(mie: usize) {
        asm!("csrw mie, {}", in(reg) mie);
    }

    #[inline(always)]
    pub unsafe fn medeleg() -> usize {
        let medeleg;
        asm!("csrr {}, medeleg", out(reg) medeleg);
        medeleg
    }

    #[inline(always)]
    pub unsafe fn set_medeleg(medeleg: usize) {
        asm!("csrw medeleg, {}", in(reg) medeleg);
    }

    #[inline(always)]
    pub unsafe fn mideleg() -> usize {
        let mideleg;
        asm!("csrr {}, mideleg", out(reg) mideleg);
        mideleg
    }

    #[inline(always)]
    pub unsafe fn set_mideleg(mideleg: usize) {
        asm!("csrw mideleg, {}", in(reg) mideleg);
    }

    #[inline(always)]
    pub unsafe fn mtvec() -> usize {
        let mtvec;
        asm!("csrr {}, mtvec", out(reg) mtvec);
        mtvec
    }

    #[inline(always)]
    pub unsafe fn set_mtvec(mtvec: usize) {
        asm!("csrw mtvec, {}", in(reg) mtvec);
    }

    #[inline(always)]
    pub unsafe fn mcounteren() -> usize {
        let mcounteren;
        asm!("csrr {}, mcounteren", out(reg) mcounteren);
        mcounteren
    }

    #[inline(always)]
    pub unsafe fn set_mcounteren(mcounteren: usize) {
        asm!("csrw mcounteren, {}", in(reg) mcounteren);
    }

    #[inline(always)]
    pub unsafe fn pmpcfg0() -> usize {
        let pmpcfg0;
        asm!("csrr {}, pmpcfg0", out(reg) pmpcfg0);
        pmpcfg0
    }

    #[inline(always)]
    pub unsafe fn set_pmpcfg0(pmpcfg0: usize) {
        asm!("csrw pmpcfg0, {}", in(reg) pmpcfg0);
    }

    #[inline(always)]
    pub unsafe fn pmpaddr0() -> usize {
        let pmpaddr0;
        asm!("csrr {}, pmpaddr0", out(reg) pmpaddr0);
        pmpaddr0
    }

    #[inline(always)]
    pub unsafe fn set_pmpaddr0(pmpaddr0: usize) {
        asm!("csrw pmpaddr0, {}", in(reg) pmpaddr0);
    }

    #[inline(always)]
    pub unsafe fn sstatus() -> usize {
        let sstatus;
        asm!("csrr {}, sstatus", out(reg) sstatus);
        sstatus
    }

    #[inline(always)]
    pub unsafe fn set_sstatus(sstatus: usize) {
        asm!("csrw sstatus, {}", in(reg) sstatus);
    }

    #[inline(always)]
    pub unsafe fn sepc() -> usize {
        let sepc;
        asm!("csrr {}, sepc", out(reg) sepc);
        sepc
    }

    #[inline(always)]
    pub unsafe fn set_sepc(sepc: usize) {
        asm!("csrw sepc, {}", in(reg) sepc);
    }

    #[inline(always)]
    pub unsafe fn scause() -> usize {
        let scause;
        asm!("csrr {}, scause", out(reg) scause);
        scause
    }

    #[inline(always)]
    pub unsafe fn set_scause(scause: usize) {
        asm!("csrw scause, {}", in(reg) scause);
    }

    #[inline(always)]
    pub unsafe fn stval() -> usize {
        let stval;
        asm!("csrr {}, stval", out(reg) stval);
        stval
    }

    #[inline(always)]
    pub unsafe fn set_stval(stval: usize) {
        asm!("csrw stval, {}", in(reg) stval);
    }

    #[inline(always)]
    pub unsafe fn sscratch() -> usize {
        let sscratch;
        asm!("csrr {}, sscratch", out(reg) sscratch);
        sscratch
    }

    #[inline(always)]
    pub unsafe fn set_sscratch(sscratch: usize) {
        asm!("csrw sscratch, {}", in(reg) sscratch);
    }

    #[inline(always)]
    pub unsafe fn sip() -> usize {
        let sip;
        asm!("csrr {}, sip", out(reg) sip);
        sip
    }

    #[inline(always)]
    pub unsafe fn set_sip(sip: usize) {
        asm!("csrw sip, {}", in(reg) sip);
    }

    #[inline(always)]
    pub unsafe fn sedeleg() -> usize {
        let sedeleg;
        asm!("csrr {}, sedeleg", out(reg) sedeleg);
        sedeleg
    }

    #[inline(always)]
    pub unsafe fn set_sedeleg(sedeleg: usize) {
        asm!("csrw sedeleg, {}", in(reg) sedeleg);
    }

    #[inline(always)]
    pub unsafe fn sideleg() -> usize {
        let sideleg;
        asm!("csrr {}, sideleg", out(reg) sideleg);
        sideleg
    }

    #[inline(always)]
    pub unsafe fn set_sideleg(sideleg: usize) {
        asm!("csrw sideleg, {}", in(reg) sideleg);
    }

    #[inline(always)]
    pub unsafe fn stvec() -> usize {
        let stvec;
        asm!("csrr {}, stvec", out(reg) stvec);
        stvec
    }

    #[inline(always)]
    pub unsafe fn set_stvec(stvec: usize) {
        asm!("csrw stvec, {}", in(reg) stvec);
    }

    #[inline(always)]
    pub unsafe fn satp() -> usize {
        let satp;
        asm!("csrr {}, satp", out(reg) satp);
        satp
    }

    #[inline(always)]
    pub unsafe fn set_satp(satp: usize) {
        asm!("csrw satp, {}", in(reg) satp);
    }

    #[inline(always)]
    pub unsafe fn sie() -> usize {
        let sie;
        asm!("csrr {}, sie", out(reg) sie);
        sie
    }

    #[inline(always)]
    pub unsafe fn set_sie(sie: usize) {
        asm!("csrw sie, {}", in(reg) sie);
    }

    #[inline(always)]
    pub unsafe fn sfence_vma_flush_tlb() {
        asm!("sfence.vma zero, zero");
    }

    #[inline(always)]
    pub fn tp() -> usize {
        let tp;
        unsafe {
            asm!("mv {}, tp", out(reg) tp);
        }
        tp
    }

    #[inline(always)]
    pub unsafe fn set_tp(tp: usize) {
        asm!("mv tp, {}", in(reg) tp);
    }

    #[inline(always)]
    pub fn sp() -> usize {
        let sp;
        unsafe {
            asm!("mv {}, sp", out(reg) sp);
        }
        sp
    }

    #[inline(always)]
    pub unsafe fn set_sp(sp: usize) {
        asm!("mv sp, {}", in(reg) sp);
    }

    #[inline(always)]
    pub fn ra() -> usize {
        let ra;
        unsafe {
            asm!("mv {}, ra", out(reg) ra);
        }
        ra
    }

    #[inline(always)]
    pub unsafe fn set_ra(ra: usize) {
        asm!("mv ra, {}", in(reg) ra);
    }
}
