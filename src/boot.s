/* Declare constants for the multiboot header. */
.set ALIGN,    1<<0             /* align loaded modules on page boundaries */
.set MEMINFO,  1<<1             /* provide memory map */
.set FLAGS,    ALIGN | MEMINFO  /* this is the Multiboot 'flag' field */
.set MAGIC,    0x1BADB002       /* 'magic number' lets bootloader find the header */
.set CHECKSUM, -(MAGIC + FLAGS) /* checksum of above, to prove we are multiboot */

.section .multiboot
.global _multiboot_header
_multiboot_header:
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

/* Stack set up */
.section .bss
.align 16
stack_bottom:
/*.skip 16384 # 16 KiB */
.skip 32768 # 32 KiB
stack_top:

.section .text
.global _start
.type _start, @function
_start:
	mov $stack_top, %esp
	push %ebx
	call kernel_main
