.PHONY : kernel bootloader run
.DEFAULT_GOAL := all

kernel:
	cargo xbuild -p kernel --target json\kernel.json --release
	@>nul copy .\target\kernel\release\kernel .\build\EFI
	@>nul strip build/kernel

bootloader:
	cargo xbuild -p bootloader --target x86_64-unknown-uefi --release
	@>nul copy .\target\x86_64-unknown-uefi\release\bootloader.efi .\build\EFI\BootX64.efi
	@>nul copy .\build\OVMF_VARS.backup .\build\OVMF_VARS.fd

run:
	qemu-system-x86_64 \
	-nodefaults \
	-vga std \
	-machine q35,accel=kvm:tcg \
	-m 64M \
	-drive if=pflash,format=raw,readonly,file=build\OVMF_CODE.fd \
	-drive if=pflash,format=raw,file=build\OVMF_VARS.fd \
	-drive format=raw,file=fat:rw:build\EFI \
	-monitor vc:1024x768

all: bootloader run