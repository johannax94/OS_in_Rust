# OS_in_Rust
Creation of a little kernel in rust language. 
What is the kernel ? Basically, it is the part of the operating system that manages the hardware. 
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
