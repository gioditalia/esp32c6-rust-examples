//! # ESP32-C6 WS2812 RGB LED
//!
//! Controllo LED WS2812 (NeoPixel) con SPI.
//! Effetto arcobaleno ciclico sul LED integrato GPIO8.
//!
//! ## Hardware
//! - LED WS2812 su GPIO8
//! - GPIO6 per clock SPI (richiesto ma non collegato al LED)
//!
//! ## Build
//! ```bash
//! cargo run --release --bin ws2812
//! ```

#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
#![deny(clippy::large_stack_frames)]

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant, Rate};
use esp_hal::spi::master::{Spi, Config};
use ws2812_spi::Ws2812;
use smart_leds_trait::{SmartLedsWrite, RGB8};
use esp_println::println;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("\n!!! PANIC !!!");
    esp_println::println!("{}", info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(clippy::large_stack_frames)]
#[main]
fn main() -> ! {
    // Inizializza ESP32-C6 alla massima velocità
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    println!("\n=== ESP32-C6 WS2812 LED ===");
    println!("LED su GPIO8\n");

    // Configura pin SPI per WS2812
    // GPIO6: clock (richiesto da SPI ma non usato dal LED)
    // GPIO8: data line per il LED WS2812
    let sclk = peripherals.GPIO6;
    let mosi = peripherals.GPIO8;

    // Configura SPI a 3.2 MHz (frequenza corretta per timing WS2812)
    let spi_config = Config::default().with_frequency(Rate::from_hz(3_200_000));
    let spi = Spi::new(peripherals.SPI2, spi_config)
        .expect("SPI init failed")
        .with_sck(sclk)
        .with_mosi(mosi);

    // Crea driver WS2812 che converte RGB in pattern SPI
    let mut ws = Ws2812::new(spi);
    
    // Variabile per la tonalità del colore (0-255)
    let mut hue = 0u8;

    loop {
        let start = Instant::now();
        
        // Converte HSV a RGB e invia al LED
        let color = hsv_to_rgb(hue, 255, 100);
        ws.write([color].iter().cloned()).ok();
        
        println!("Hue: {}", hue);
        
        // Incrementa tonalità per effetto arcobaleno
        hue = hue.wrapping_add(2);
        
        // Mantiene 50 FPS (~20ms per frame)
        while start.elapsed() < Duration::from_millis(20) {}
    }
}

/// Converte colore da HSV a RGB
/// 
/// HSV (Hue, Saturation, Value) è più intuitivo per animazioni:
/// - h: tonalità 0-255 (posizione nella ruota dei colori)
/// - s: saturazione 0-255 (intensità del colore)
/// - v: luminosità 0-255 (brightness)
fn hsv_to_rgb(h: u8, s: u8, v: u8) -> RGB8 {
    if s == 0 {
        return RGB8::new(v, v, v);
    }

    // Dividi la ruota dei colori in 6 regioni
    let region = h / 43;
    let remainder = (h - (region * 43)) * 6;

    // Calcola valori intermedi (usa u16 per evitare overflow)
    let p = (v as u16 * (255 - s as u16)) >> 8;
    let q = (v as u16 * (255 - ((s as u16 * remainder as u16) >> 8))) >> 8;
    let t = (v as u16 * (255 - ((s as u16 * (255 - remainder as u16)) >> 8))) >> 8;

    // Seleziona RGB in base alla regione
    match region {
        0 => RGB8::new(v, t as u8, p as u8),        // Rosso → Giallo
        1 => RGB8::new(q as u8, v, p as u8),        // Giallo → Verde
        2 => RGB8::new(p as u8, v, t as u8),        // Verde → Ciano
        3 => RGB8::new(p as u8, q as u8, v),        // Ciano → Blu
        4 => RGB8::new(t as u8, p as u8, v),        // Blu → Magenta
        _ => RGB8::new(v, p as u8, q as u8),        // Magenta → Rosso
    }
}
