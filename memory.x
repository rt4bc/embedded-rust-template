MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 2048K
  RAM : ORIGIN = 0x20000000, LENGTH = 384K
  CCMRAM : ORIGIN = 0x10000000, LENGTH = 64K
  SDRAM : ORIGIN = 0xC0000000, LENGTH = 16M
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This is required only on microcontrollers that store some configuration right
   after the vector table */
/* _stext = ORIGIN(FLASH) + 0x400; */

/* Example of putting non-initialized variables into custom RAM locations. */
/* This assumes you have defined a region called CCMRAM above, and in the 
   Rust privacy crate you have a `static mut` called `MyCCMVar`. */
/* Note that the section name `.uninit.MyCCMVar` must match the name used
   in the Rust declarations. */
/* SECTIONS {
     .uninit.MyCCMVar (NOLOAD) : ALIGN(4) {
       . = ALIGN(4);
       __sMyCCMVar = .;
       *(.uninit.MyCCMVar .uninit.MyCCMVar.*);
       . = ALIGN(4);
       __eMyCCMVar = .;
     } > CCMRAM
   } INSERT AFTER .bss;
*/