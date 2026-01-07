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
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Coucou kernel!\n").unwrap();
    write!(vga_buffer::WRITER.lock(), "42 = {}\n", 6 * 7).unwrap();

    loop {}
}

/* POUR LA SUITE (IMPORTANT POUR LE KERNEL)

Après compilation réussie, il faudra :

Désactiver les interruptions SSE au boot

Initialiser FPU/SSE AVANT toute interruption

Sauvegarder/restaurer les registres SSE dans le scheduler (plus tard)
Configurer le linker script pour réserver l'espace nécessaire aux structures SSE*/
