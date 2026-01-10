# OS_in_Rust
Creation of a little kernel in rust language. 
What is a kernel ? Basically, it is the part of the operating system that manages the hardware. 
To create a kernel, we need to create an executable "bare-metal" (without any operating system) so we couldn't use rust libraries (or all lib that are system dependant). 
To make that, we will use the variable 'no_std' to disable the automation of the standard libraries use. 


We will use a bootimage instead of a bootloader (bootloader is laborious and that is not the subject)
  
Parameters : 

- The kernel will use 64-bits registers.
- It will be for x86 architecture CPU's.
- We will use BIOS technology to boot 


Steps :

- Motherboard load the BIOS
- It find a disk and give control to bootloader
- Bootloader load on the 512 first bytes of the disk
- Bootloader finds the kernel image on the disk
- It also switch the CPU from the 16bits real-mode to the 32 bits protected-mode and finally to the 64 bits long-mode (registers and memory are now available)
- Launch of the OS 

Technical notes : 

- To print text on the screen after creating our kernel but let the _start entry point empty, we will use VGA text buffer
- VGA text buffer is a specific memory area that will contain the content to display
- VGA - Video Graphic Array - hardware piece
- To make the kernel running, we need to link it with our bootimage before compile because rust does not support the treatment of files after compiling. 
- When running the build bootimage command: rust program is compiled and bootloader too, then they are linked and it creates a .bin file which contains our basic kernel
- To boot our new kernel, we will use QEMU which is a virtual machine that can emulate a CPU etc.

- We will use existing rust macros to create our own print and println functions (to laborious to do from scratch)

- To create the memory allocator, we will choose a SLAB Allocator
- IDT (Interrupt Descriptor Table) - Each exception has a predefined IDT index
- Rust proposes its own InterruptDescriptorTable struct in the x86_64 crate so we don't need to recreate it
