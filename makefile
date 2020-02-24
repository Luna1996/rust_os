.PHONY : all, clean
.DEFAULT_GOAL := all

target_dir = target
build_dir := build
efi_target := $(build_dir)/kernel_target.json

all:
	cargo xbuild --target $(efi_target) --release