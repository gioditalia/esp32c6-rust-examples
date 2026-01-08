//! # ESP32-C6 Hello World Template
//!
//! Esempio base che stampa messaggi periodici sulla console UART.
//! Questo è un template minimale per iniziare lo sviluppo su ESP32-C6 con Rust.
//!
//! ## Hardware richiesto
//! - ESP32-C6 DevKit o compatibile
//!
//! ## Collegamenti
//! - UART TX: GPIO16 (default, USB-Serial integrato)
//! - UART RX: GPIO17 (default, USB-Serial integrato)
//!
//! ## Build e Flash
//! ```bash
//! cargo run --release
//! ```
//!
//! ## Concetti dimostrati
//! - Inizializzazione base dell'HAL ESP32
//! - Configurazione CPU alla massima velocità
//! - Output su UART tramite println!
//! - Loop infinito con delay timing-safe
//! - Gestione panic handler per embedded

#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::clock::{Clock, CpuClock};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println::println;

/// Panic handler per gestire errori fatali
/// In caso di panic, stampa l'errore su UART e entra in loop infinito
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("\n!!! PANIC !!!");
    esp_println::println!("{}", info);
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // Configura la CPU alla massima velocità (160 MHz per ESP32-C6)
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());

    // Inizializza l'HAL e ottiene accesso ai periferici hardware
    // Usiamo _ perché in questo esempio non accediamo direttamente ai periferici
    let _peripherals = esp_hal::init(config);

    // Stampa messaggio di avvio
    println!("\n=== ESP32-C6 Hello World ===");
    println!("Sistema inizializzato correttamente!");
    println!("CPU Clock: {} MHz\n", CpuClock::max().mhz());

    // Contatore per i messaggi
    let mut counter = 0u32;

    // Loop infinito principale
    loop {
        // Marca l'inizio del ciclo per timing preciso
        let delay_start = Instant::now();

        // Stampa messaggio con contatore
        println!("[{}] Ciao da ESP32-C6 🚀", counter);
        counter = counter.wrapping_add(1);

        // Attende esattamente 500ms prima del prossimo messaggio
        // Questo metodo è più preciso di un semplice delay perché compensa
        // il tempo impiegato dal codice sopra
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
