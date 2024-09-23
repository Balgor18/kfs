# Flags
RUST_FLAGS := #--features gdt_test --features verbose

### KEEP

TARGET_DIR 		= target/kfs/release
RUST_LIB 		= $(TARGET_DIR)/libKFS1.a

KERNEL_BIN		= $(OBJ_DIR)/kfs.bin
ISO_FILE		= kfs.iso

LD_FLAG			= -melf_i386 #-z noexecstack
AS_FLAG			= --32
# --target i686-unknown-linux-gnu
RUST_FLAG		= --target src/arch/kfs.json -Z build-std=core,alloc #--release#--features gdt_test --features verbose
# RUST_FLAG		= --target src/arch/kfs.json -Z build-std=core,alloc #--release#--features gdt_test --features verbose
GRUB_FLAG		= -d /usr/lib/grub/i386-pc

FLAG_CMD_RUST	= +nightly

CMD_RUST		= cargo $(FLAG_CMD_RUST) 
CMD_AS			= as $(AS_FLAG)
CMD_LD			= ld $(LD_FLAG)
CMD_GRUB		= grub-mkrescue $(GRUB_FLAG)
CMD_QEMU		= qemu-system-i386

ISO_DIR			= iso/
SRC_DIR			= $(shell find src -type d)
OBJ_DIR			= obj

OBJS			= $(addprefix $(OBJ_DIR)/, $(SRCS:%.s=%.o))

AS_SRC			= boot.s
LD_SRC			= linker.ld

.phony: stop start connect sim stopsim re fclean clean

vpath %.s $(foreach dir, $(SRC_DIR), $(dir):)

all : $(ISO_FILE)

# $(ISO_FILE): $(OBJS)
# # cargo build
# 	cargo build --target thumbv7em-none-eabihf
# 	grub-file --is-x86-multiboot $(ISO_FILE)

qemu: $(ISO_FILE)
	$(CMD_QEMU) -name kfs -cdrom $(ISO_FILE) -boot c -curses
# $(QEMU) -cdrom $(ISO_FILE) -m 64M -curses

# qemu_dbg: $(ISO_FILE)
#         @echo "Starting with QEMU in debug mode"
#         $(QEMU) -cdrom $(ISO_FILE) -s -S -serial stdio

stopsim:
	kill -9 $$(top -bn1 | grep qemu | awk '{ print $$1 }')

start:
	vagrant up

connect :
	vagrant ssh

stop: 
	vagrant halt


$(OBJ_DIR)/%.o: %.s | $(OBJ_DIR)
	$(CMD_AS) -o $@ $<
# # Build rules

$(RUST_LIB): $(RUST_SRCS) | $(TARGET_DIR)
	@echo "Building Rust"
	$(CMD_RUST) build $(RUST_FLAG) --release
# $(CC) build -Z build-std=core,alloc --target=$(TARGET_TRIPLE).json --release
# @touch $(RUST_LIB)

$(KERNEL_BIN): $(OBJ_DIR)/boot.o $(RUST_LIB) | $(OBJ_DIR)
	@echo "Linking kernel"
	$(CMD_LD) -o $@ -T src/linker.ld $^
	# cp iso/boot/kernel.bin .


$(ISO_FILE): $(KERNEL_BIN) | $(ISO_DIR)/boot/grub
	@echo "Making ISO"
	cp src/grub/grub.cfg $(ISO_DIR)/boot/grub
	cp $(KERNEL_BIN) $(ISO_DIR)/boot/kernel.bin
	$(CMD_GRUB) -o $@ $(ISO_DIR) 2> /dev/null

# Directory creation
$(OBJ_DIR) $(ISO_DIR)boot/grub $(TARGET_DIR):
	mkdir -p $@

clean:
	$(RM) -r $(OBJ_DIR)

fclean: clean
	cargo clean
	$(RM) -r $(ISO_DIR)
	$(RM) -r $(ISO_FILE)

re: fclean all

# Compil the as file
# i686-elf-as boot.s -o boot.o

# Compil in rust
# cargo build --target thumbv7em-none-eabihf
# cargo rustc -- -C link-arg=-nostartfiles


# Verif multiboot
# grub-file --is-x86-multiboot myos.bin

# ```
# if grub-file --is-x86-multiboot myos.bin; then
#   echo multiboot confirmed
# else
#   echo the file is not multiboot
# fi
# ```


# Command to launch the iso
# kvm -m 2048 -s -hda ./test.img -redir tcp:2323::23 -curses
# sudo kvm -m 2048 -s -hda ./test.img -redir tcp:2323::23 -curses

# qemu-system-i386 -kernel myos.bin -m 64M -curses
# qemu-system-i386 -cdrom myos.iso -m 64M -curses -redir tcp:2323::23

# qemu-system-i386 -cdrom myos.iso -m 64M -curses
# qemu-system-i386 -cdrom myos.iso -m 64M -curses -netdev user,id=mynet0,hostfwd=tcp::2323-:23 -device e1000,netdev=mynet0
