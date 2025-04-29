OUTPUT_ARCH(riscv)
ENTRY(_start);

SECTIONS {
  . = 0x80000000;
  .text : { *(.text.entry) *(.text*) }
  .rodata : { *(.rodata*) }
  .data : { *(.data*) }
  .bss : {
    *(.bss*)
    *(COMMON)
  }
  /DISCARD/ : { *(.eh_frame) *(.comment) }
}
