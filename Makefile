QEMU = qemu
GRUB = grub
NASM = nasm
LD = ld

arch ?= x86_64
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso
target ?= $(arch)-lambda
rust_os := target/$(target)/debug/liblambdaOS.a

linker_script := src/arch/$(arch)/asm/linker.ld
grub_cfg := src/arch/$(arch)/asm/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/asm/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/asm/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

CARGOFLAGS :=

ifdef FEATURES
	CARGOFLAGS += --no-default-features --features $(FEATURES)
endif

.PHONY: all clean run iso kernel

all: $(kernel)

clean:
	@rm -r build
	@cargo clean

run: $(iso)
	@$(QEMU)-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@$(GRUB)-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles

$(kernel): kernel $(rust_os) $(assembly_object_files) $(linker_script)
	@$(LD) -n --gc-sections -T $(linker_script) -o $(kernel) \
		$(assembly_object_files) $(rust_os)

kernel:
	@RUST_TARGET_PATH="$(shell pwd)" xargo build --target $(target) $(CARGOFLAGS)

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/asm/%.asm
	@mkdir -p $(shell dirname $@)
	@$(NASM) -felf64 $< -o $@
