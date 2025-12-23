# Come Contribuire

Grazie per l'interesse nel contribuire a questo progetto! 🎉

## 🌟 Modi per Contribuire

- 🐛 Segnalare bug
- 💡 Proporre nuovi esempi
- 📝 Migliorare la documentazione
- 🔧 Fixare issue esistenti
- ✨ Aggiungere feature

## 📋 Aggiungere un Nuovo Esempio

### 1. Struttura Base

Crea un nuovo file `src/bin/nome-esempio.rs`:

```rust
//! # ESP32-C6 Nome Esempio
//!
//! Breve descrizione dell'esempio
//!
//! ## Hardware richiesto
//! - Lista componenti
//!
//! ## Collegamenti
//! - Schema pin
//!
//! ## Concetti dimostrati
//! - Cosa si impara

#![no_std]
#![no_main]

use esp_hal::main;
// ... altri import

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("\n!!! PANIC !!!");
    esp_println::println!("{}", info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Il tuo codice qui
}
```

### 2. Aggiorna Cargo.toml

```toml
[[bin]]
name = "nome-esempio"
path = "./src/bin/nome-esempio.rs"
required-features = ["example1000"]

[dependencies]
some-dep = { version = "0.5", optional = true }

[features]
example1000 = ["some-dep", "other-dep"]
```

### 3. Testa

```bash
cargo build --release --bin nome-esempio
cargo run --release --bin nome-esempio
```

### 4. Documenta

Assicurati di includere:
- Doc comments dettagliati (//!)
- Commenti inline per codice complesso
- Schema collegamenti hardware
- Output di esempio
- Possibili modifiche/esperimenti

## ✅ Checklist Pre-PR

- [ ] Codice compila senza warning
- [ ] Testato su hardware reale
- [ ] Doc comments presenti e completi
- [ ] Codice formattato con `cargo fmt`
- [ ] Linting passato: `cargo clippy`
- [ ] README aggiornato (se necessario)
- [ ] Branch aggiornato con main

## 🎨 Stile Codice

### Rust
- Usa `cargo fmt` per formattazione automatica
- Segui le [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Preferisci chiarezza a brevità

### Commenti
- Italiano o inglese, ma consistente nel file
- Doc comments (//!) per moduli e funzioni pubbliche
- Inline comments (//) per logica complessa

### Naming
- `snake_case` per variabili e funzioni
- `CamelCase` per tipi e trait
- Nomi descrittivi e chiari


## ❓ Domande?

- Apri una Discussion su GitHub
- Chiedi su Matrix: @esp-rs:matrix.org
- Consulta la documentazione esistente

## 🙏 Grazie!

Ogni contributo, grande o piccolo, è apprezzato! ❤️
