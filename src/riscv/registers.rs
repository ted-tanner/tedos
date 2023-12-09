#![allow(dead_code)]

use core::arch::asm;

struct MRegisters {}
struct SRegisters {}
struct URegisters {}

impl MRegisters {
    #[inline(always)]
    pub fn mhartid() -> usize {
        let mhartid: usize;
        unsafe {
            asm!("csrr {}, mhartid", out(reg) mhartid);
        }
        mhartid
    }

    #[inline(always)]
    pub unsafe fn set_mhartid(mhartid: usize) {
        asm!("csrw mhartid, {}", in(reg) mhartid);
    }

    #[inline(always)]
    pub fn mstatus() -> usize {
        let mstatus: usize;
        unsafe {
            asm!("csrr {}, mstatus", out(reg) mstatus);
        }
        mstatus
    }

    #[inline(always)]
    pub unsafe fn set_mstatus(mstatus: usize) {
        asm!("csrw mstatus, {}", in(reg) mstatus);
    }

    #[inline(always)]
    pub fn mepc() -> usize {
        let mepc: usize;
        unsafe {
            asm!("csrr {}, mepc", out(reg) mepc);
        }
        mepc
    }

    #[inline(always)]
    pub unsafe fn set_mepc(mepc: usize) {
        asm!("csrw mepc, {}", in(reg) mepc);
    }

    #[inline(always)]
    pub fn mcause() -> usize {
        let mcause: usize;
        unsafe {
            asm!("csrr {}, mcause", out(reg) mcause);
        }
        mcause
    }

    #[inline(always)]
    pub unsafe fn set_mcause(mcause: usize) {
        asm!("csrw mcause, {}", in(reg) mcause);
    }

    #[inline(always)]
    pub fn mtval() -> usize {
        let mtval: usize;
        unsafe {
            asm!("csrr {}, mtval", out(reg) mtval);
        }
        mtval
    }

    #[inline(always)]
    pub unsafe fn set_mtval(mtval: usize) {
        asm!("csrw mtval, {}", in(reg) mtval);
    }

    #[inline(always)]
    pub fn mscratch() -> usize {
        let mscratch: usize;
        unsafe {
            asm!("csrr {}, mscratch", out(reg) mscratch);
        }
        mscratch
    }

    #[inline(always)]
    pub unsafe fn set_mscratch(mscratch: usize) {
        asm!("csrw mscratch, {}", in(reg) mscratch);
    }

    #[inline(always)]
    pub fn mie() -> usize {
        let mie: usize;
        unsafe {
            asm!("csrr {}, mie", out(reg) mie);
        }
        mie
    }

    #[inline(always)]
    pub unsafe fn set_mie(mie: usize) {
        asm!("csrw mie, {}", in(reg) mie);
    }

    #[inline(always)]
    pub fn medeleg() -> usize {
        let medeleg: usize;
        unsafe {
            asm!("csrr {}, medeleg", out(reg) medeleg);
        }
        medeleg
    }

    #[inline(always)]
    pub unsafe fn set_medeleg(medeleg: usize) {
        asm!("csrw medeleg, {}", in(reg) medeleg);
    }

    #[inline(always)]
    pub fn mideleg() -> usize {
        let mideleg: usize;
        unsafe {
            asm!("csrr {}, mideleg", out(reg) mideleg);
        }
        mideleg
    }

    #[inline(always)]
    pub unsafe fn set_mideleg(mideleg: usize) {
        asm!("csrw mideleg, {}", in(reg) mideleg);
    }

    #[inline(always)]
    pub fn mtvec() -> usize {
        let mtvec: usize;
        unsafe {
            asm!("csrr {}, mtvec", out(reg) mtvec);
        }
        mtvec
    }

    #[inline(always)]
    pub unsafe fn set_mtvec(mtvec: usize) {
        asm!("csrw mtvec, {}", in(reg) mtvec);
    }

    #[inline(always)]
    pub fn mcounteren() -> usize {
        let mcounteren: usize;
        unsafe {
            asm!("csrr {}, mcounteren", out(reg) mcounteren);
        }
        mcounteren
    }

    #[inline(always)]
    pub unsafe fn set_mcounteren(mcounteren: usize) {
        asm!("csrw mcounteren, {}", in(reg) mcounteren);
    }

    #[inline(always)]
    pub fn pmpcfg0() -> usize {
        let pmpcfg0: usize;
        unsafe {
            asm!("csrr {}, pmpcfg0", out(reg) pmpcfg0);
        }
        pmpcfg0
    }

    #[inline(always)]
    pub unsafe fn set_pmpcfg0(pmpcfg0: usize) {
        asm!("csrw pmpcfg0, {}", in(reg) pmpcfg0);
    }

    #[inline(always)]
    pub fn pmpaddr0() -> usize {
        let pmpaddr0: usize;
        unsafe {
            asm!("csrr {}, pmpaddr0", out(reg) pmpaddr0);
        }
        pmpaddr0
    }

    #[inline(always)]
    pub unsafe fn set_pmpaddr0(pmpaddr0: usize) {
        asm!("csrw pmpaddr0, {}", in(reg) pmpaddr0);
    }
}

impl SRegisters {
    #[inline(always)]
    pub fn sstatus() -> usize {
        let sstatus: usize;
        unsafe {
            asm!("csrr {}, sstatus", out(reg) sstatus);
        }
        sstatus
    }

    #[inline(always)]
    pub unsafe fn set_sstatus(sstatus: usize) {
        asm!("csrw sstatus, {}", in(reg) sstatus);
    }

    #[inline(always)]
    pub fn sepc() -> usize {
        let sepc: usize;
        unsafe {
            asm!("csrr {}, sepc", out(reg) sepc);
        }
        sepc
    }

    #[inline(always)]
    pub unsafe fn set_sepc(sepc: usize) {
        asm!("csrw sepc, {}", in(reg) sepc);
    }

    #[inline(always)]
    pub fn scause() -> usize {
        let scause: usize;
        unsafe {
            asm!("csrr {}, scause", out(reg) scause);
        }
        scause
    }

    #[inline(always)]
    pub unsafe fn set_scause(scause: usize) {
        asm!("csrw scause, {}", in(reg) scause);
    }

    #[inline(always)]
    pub fn stval() -> usize {
        let stval: usize;
        unsafe {
            asm!("csrr {}, stval", out(reg) stval);
        }
        stval
    }

    #[inline(always)]
    pub unsafe fn set_stval(stval: usize) {
        asm!("csrw stval, {}", in(reg) stval);
    }

    #[inline(always)]
    pub fn sscratch() -> usize {
        let sscratch: usize;
        unsafe {
            asm!("csrr {}, sscratch", out(reg) sscratch);
        }
        sscratch
    }

    #[inline(always)]
    pub unsafe fn set_sscratch(sscratch: usize) {
        asm!("csrw sscratch, {}", in(reg) sscratch);
    }

    #[inline(always)]
    pub fn sip() -> usize {
        let sip: usize;
        unsafe {
            asm!("csrr {}, sip", out(reg) sip);
        }
        sip
    }

    #[inline(always)]
    pub unsafe fn set_sip(sip: usize) {
        asm!("csrw sip, {}", in(reg) sip);
    }

    #[inline(always)]
    pub fn sedeleg() -> usize {
        let sedeleg: usize;
        unsafe {
            asm!("csrr {}, sedeleg", out(reg) sedeleg);
        }
        sedeleg
    }

    #[inline(always)]
    pub unsafe fn set_sedeleg(sedeleg: usize) {
        asm!("csrw sedeleg, {}", in(reg) sedeleg);
    }

    #[inline(always)]
    pub fn sideleg() -> usize {
        let sideleg: usize;
        unsafe {
            asm!("csrr {}, sideleg", out(reg) sideleg);
        }
        sideleg
    }

    #[inline(always)]
    pub unsafe fn set_sideleg(sideleg: usize) {
        asm!("csrw sideleg, {}", in(reg) sideleg);
    }

    #[inline(always)]
    pub fn stvec() -> usize {
        let stvec: usize;
        unsafe {
            asm!("csrr {}, stvec", out(reg) stvec);
        }
        stvec
    }

    #[inline(always)]
    pub unsafe fn set_stvec(stvec: usize) {
        asm!("csrw stvec, {}", in(reg) stvec);
    }

    #[inline(always)]
    pub fn satp() -> usize {
        let satp: usize;
        unsafe {
            asm!("csrr {}, satp", out(reg) satp);
        }
        satp
    }

    #[inline(always)]
    pub unsafe fn set_satp(satp: usize) {
        asm!("csrw satp, {}", in(reg) satp);
    }

    #[inline(always)]
    pub fn sfence_vma_flush_tlb() {
        unsafe {
            asm!("sfence.vma zero, zero");
        }
    }
}

impl URegisters {
    #[inline(always)]
    pub fn tp() -> usize {
        let tp: usize;
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
        let sp: usize;
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
        let ra: usize;
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
