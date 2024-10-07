TARGET_DIR 		= target/kfs/debug/

KERNEL_BIN		= $(TARGET_DIR)/kfs

RUST_FLAG		= --target kfs.json -Z build-std=core#,alloc #--release#--features gdt_test --features verbose

CMD_RUST		= cargo
CMD_QEMU		= qemu-system-i386

.phony: qemu stopsim start connect stop clean fclean re

all : $(KERNEL_BIN)

$(KERNEL_BIN):
	$(CMD_RUST) build $(RUST_FLAG) 

qemu: $(KERNEL_BIN)
	$(CMD_QEMU) -name kfs -kernel $(KERNEL_BIN) -boot c -curses

stopsim:
	kill -9 $$(top -bn1 | grep qemu | awk '{ print $$1 }')

start:
	vagrant up

connect :
	vagrant ssh

stop: 
	vagrant halt

clean:
	cargo clean

fclean: clean

re: fclean all
