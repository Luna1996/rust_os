.PHONY : all, clean
.DEFAULT_GOAL := all

target_dir = .\target
build_dir := .\build
efi_target := $(build_dir)\kernel_target.json
bin_path := $(target_dir)\kernel_target\release\kernel

all:
	cargo xbuild --target $(efi_target) --release
	strip $(bin_path)
	move /y $(bin_path) $(build_dir)\