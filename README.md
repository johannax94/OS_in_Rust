# OS_in_Rust
Creation of a little kernel in rust language. 
What is the kernel ? Basically, it is the part of the operating system that manages the hardware. 
To create a kernel, we need to create an executable "bare-metal" (without any operating system) so we couldn't use rust libraries (or all lib that are system dependant). 
To make that, we will use the variable 'no_std' to disable the automation of the standard libraries use. 
The kernel will use 64-bits registers.
It will be for x86 architecture CPU's.
