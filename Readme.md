# TedOS

Experimental toy OS written in Rust

## Installing RiscV assembler and linker on macOS

`brew install riscv-tools`

## Creating a QEMU disck image

`qemu-img create tedos-fs.img 32M`

## TODO

* Initialize harts one-by-one, using the same kstack for each and then using KinitHeap to allocate a new stack for each (unless system allocator is initialized, in which case the allocator should be used in case harts come online after the KinitHeap is locked)
  - Each hart calls a Rust (extern "C") function that registers the hart
    - The registration is done like so: There is a single-item channel based off an `AtomicUszie` (perhaps two atomics, one for a lock - 0 for locked and 1 for ready). If the item is occupied (non-zero, containing the hart ID), a hart wishing to send over the channel must spin. hart0 will receive the hartid and swap in a 0
* Learning resources
  - https://osblog.stephenmarz.com/ch1.html
  - https://github.com/mit-pdos/xv6-riscv/tree/riscv
  - https://www.meyerzinn.tech/posts/2023/03/05/running-rust-code-on-risc-v-in-qemu/
  - https://os.phil-opp.com
