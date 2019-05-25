VERSION=0.1.0-Dev

ARCH?=x86_64
export ARCH

BIN = bin/$(ARCH)

BUILD = build/$(ARCH)
export BUILD

CONFIG = cfg

GRUB = $(BUILD)/isofiles/boot/grub

KERNEL = $(BIN)/kernel.bin

TakOS = $(BIN)/TakOS.iso

LOGS = logs


.PHONY: all clean help


help:
	@echo
	@echo "\033[32m Makefile for build Tako Operating System."
	@echo " Please see LICENSE for licensing information."
	@echo " Usage: make [ all | clean | help ]"
	@echo ""
	@echo " Version" $(VERSION) "\033[0m"
	@echo


all: $(TakOS)

$(KERNEL):
	mkdir -p $(BUILD)
	@echo "Building Bootloader..."
	make -C ./boot
	mkdir -p $(BIN)
	mkdir -p $(BUILD)
	@echo "Building Kernel..."
	cargo xbuild
	cp target/$(ARCH)-takos/debug/libtakos.a $(BUILD)/libtakos.a
	ld -n --gc-sections -o $(KERNEL) -T $(CONFIG)/linker.ld $(BUILD)/multiboot_header.o $(BUILD)/boot.o $(BUILD)/long_mode_init.o $(BUILD)/libtakos.a

$(TakOS): $(KERNEL)
	mkdir -p $(GRUB)
	@echo "Build GRUB ISO..."
	cp $(CONFIG)/grub.cfg $(GRUB)
	cp $(KERNEL) $(GRUB)/../
	grub-mkrescue -o $@ $(GRUB)/../../


qemu: $(TakOS)
	mkdir -p $(LOGS)
	qemu-system-$(ARCH) -curses -m 8G -serial file:$(LOGS)/serial-$(shell date +"%d%m%y-%Hh%Mm%Ss").log -cdrom $^
#qemu-system-x86_64 -curses -m 8G -serial file:logs/serial.log -cdrom bin/x86_64/TakOS.iso

clean:
	make -C ./boot clean
	rm -rf bin
	rm -rf build
	rm -rf logs
	cargo clean
