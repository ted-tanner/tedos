use crate::alloc::KinitHeap;

const CYCLES_BETWEEN_INTERRUPTS: usize = 2_500_000;

const CLINT_BASE: *mut u8 = 0x0200_0000 as *mut u8;
const CLINT_INTERRUPTOR_BASE: *mut u8 = 0x0200_4000 as *mut u8;
const CLINT_CYCLES_SINCE_BOOT: *const usize = 0x0200_bff8 as *const usize;

pub struct Clint {}

impl Clint {
    /// Must be called from Machine Mode
    pub unsafe fn init_timer_interrupts(hartid: usize) {
        let next_interrupt_time =
            CLINT_CYCLES_SINCE_BOOT.read_volatile() + CYCLES_BETWEEN_INTERRUPTS;

        let register_scratch: &mut [u8; 5] = KinitHeap::alloc();

        // From xv6-riscv:
        // // prepare information in scratch[] for timervec.
        // // scratch[0..2] : space for timervec to save registers.
        // // scratch[3] : address of CLINT MTIMECMP register.
        // // scratch[4] : desired interval (in cycles) between timer interrupts.
        // uint64 *scratch = &timer_scratch[id][0];
        // scratch[3] = CLINT_MTIMECMP(id);
        // scratch[4] = interval;
        // w_mscratch((uint64)scratch);

        // // set the machine-mode trap handler.
        // w_mtvec((uint64)timervec);

        // // enable machine-mode interrupts.
        // w_mstatus(r_mstatus() | MSTATUS_MIE);

        // // enable machine-mode timer interrupts.
        // w_mie(r_mie() | MIE_MTIE);

        get_hart_interruptor(hartid).write_volatile(1);
    }
}

#[inline]
fn get_hart_interruptor(hartid: usize) -> *mut u8 {
    unsafe { CLINT_INTERRUPTOR_BASE.add(8 * hartid) }
}
