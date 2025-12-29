# OS_in_Rust
Creation of a little kernel in rust language. 
To create a kernel, we need to create an executable "bare-metal" (without any operating system) so we couldn't use rust libraries (or all lib that are system dependant). 
To make that, we will use the variable 'no_std' to disable the automation of the standard libraries use. 
