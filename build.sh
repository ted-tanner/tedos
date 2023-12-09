if [[ $2 = "release" ]]; then
	BUILD_PROFILE=release
elif [[ $2 = "debug" || $2 = "" ]]; then
	BUILD_PROFILE=debug
else
    echo "Invalid build profile: $2"
    exit 1
fi

LD_SCRIPT=-T./src/riscv/qemu-virt.ld
ASM_SRCS=./src/riscv/boot.S

CC=riscv64-unknown-elf-gcc
CFLAGS="-Wall -Wextra -Wextra -static -ffreestanding -nostdlib -fno-rtti -fno-exceptions -march=rv64gc -mabi=lp64"
ELF_OUT=tedos-kernel.elf

RS_OUT_DIR=./target/riscv64gc-unknown-none-elf/$BUILD_PROFILE
KERNEL_LIB=$RS_OUT_DIR/libtedoskernel.a

QEMU=qemu-system-riscv64
QEMU_CPU_COUNT=4
QEMU_MEM=128M
QEMU_DISK=tedos-fs.img

function build {
	if [[ $BUILD_PROFILE = "debug" ]]; then
        (PS4="\000" set -x;
	     cargo build
	     $CC $CFLAGS -O0 -g $LD_SCRIPT -o $ELF_OUT $KERNEL_LIB $ASM_SRCS -L$RS_OUT_DIR) || exit 1
	elif [[ $BUILD_PROFILE = "release" ]]; then
        (PS4="\000" set -x;
	     cargo build --release
	     $CC $CFLAGS -O3 $LD_SCRIPT -o $ELF_OUT $KERNEL_LIB $ASM_SRCS -L$RS_OUT_DIR) || exit 1
    fi
}

if [[ $1 = "build" ]]; then
	build
elif [[ $1 = "run" ]]; then
    build &&
    $QEMU -machine virt -cpu rv64 \
        -smp $QEMU_CPU_COUNT -m $QEMU_MEM \
        -nographic -serial mon:stdio \
        -bios none \
        -kernel $ELF_OUT \
        -drive file=$QEMU_DISK,if=none,format=raw,id=dsk0 \
        -device virtio-blk-device,drive=dsk0,bus=virtio-mmio-bus.0
elif [[ $1 = "clean" ]]; then
    cargo clean
    rm -f $ELF_OUT
else
    echo "Usage: ./$(basename $0) <build|run|clean> <optional:debug|release>"
    exit 1
fi
