#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}

/* POUR LA SUITE (IMPORTANT POUR LE KERNEL)

Après compilation réussie, il faudra :

Désactiver les interruptions SSE au boot

Initialiser FPU/SSE AVANT toute interruption

Sauvegarder/restaurer les registres SSE dans le scheduler (plus tard)
Configurer le linker script pour réserver l'espace nécessaire aux structures SSE*/
