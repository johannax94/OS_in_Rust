#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
///on créer notre propre point d'entrée 
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

/* POUR LA SUITE (IMPORTANT POUR LE KERNEL)

Après compilation réussie, il faudra :

Désactiver les interruptions SSE au boot

Initialiser FPU/SSE AVANT toute interruption

Sauvegarder/restaurer les registres SSE dans le scheduler (plus tard)
Configurer le linker script pour réserver l'espace nécessaire aux structures SSE*/
