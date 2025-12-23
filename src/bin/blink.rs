//! # ESP32-C6 Blink LED
//!
//! Esempio classico "blink" che fa lampeggiare un LED collegato a un GPIO.
//! Dimostra il controllo base dei GPIO in modalità output.
//!
//! ## Hardware richiesto
//! - ESP32-C6 DevKit
//! - LED + resistenza 220Ω (o LED integrato se disponibile)
//!
//! ## Collegamenti
//! - LED positivo (+) → GPIO2 → Resistenza 220Ω → GND
//! - Oppure usa il LED integrato sulla board (spesso GPIO8 o GPIO2)
//!
//! ## Build e Flash
//! ```bash
//! cargo run --release --bin blink
//! ```
//!
//! ## Concetti dimostrati
//! - Configurazione GPIO come output
//! - Toggle di stato GPIO (HIGH/LOW)
//! - Uso dell'HAL Delay per timing
//! - Output digitale base

#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
#![deny(clippy::large_stack_frames)]

use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println::println;

/// Panic handler per gestire errori fatali
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("\n!!! PANIC !!!");
    esp_println::println!("{}", info);
    loop {}
}

// Bootloader descriptor richiesto da ESP-IDF
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(clippy::large_stack_frames)]
#[main]
fn main() -> ! {
    // Configura la CPU alla massima velocità
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    println!("\n=== ESP32-C6 Blink LED ===");
    println!("LED collegato a GPIO2\n");

    // Configura GPIO2 come output, inizialmente LOW (LED spento)
    // OutputConfig::default() usa push-pull, pull-up/down disabilitati
    let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());

    // Contatore per i cicli
    let mut count = 0u32;

    // Loop infinito
    loop {
        // Accende il LED (imposta GPIO HIGH)
        led.set_high();
        println!("[{}] LED ON", count);

        // Aspetta 500ms usando busy-wait con timing preciso
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(500) {}

        // Spegne il LED (imposta GPIO LOW)
        led.set_low();
        println!("[{}] LED OFF", count);

        // Aspetta 500ms usando busy-wait con timing preciso
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(500) {}

        count = count.wrapping_add(10);
    }
}
