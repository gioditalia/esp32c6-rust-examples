# 🦀 ESP32-C6 Rust Templates & Examples

Una collezione di template ed esempi pronti all'uso per sviluppare firmware ESP32-C6 in Rust. Perfetto per imparare Rust attraverso lo sviluppo embedded!

## 🎯 Obiettivo del Progetto

Questa repository nasce per:
- Fornire **template pronti all'uso** per progetti ESP32-C6
- Creare una **risorsa educativa** per imparare Rust su embedded
- Documentare **best practices** per lo sviluppo ESP32 in Rust
- Offrire **esempi progressivi** da semplici a complessi

## 📋 Esempi Disponibili

### Basics
| Esempio | Descrizione |  Difficoltà |
|---------|-------------|------------|
| [hello-world](src/bin/main.rs) | Stampa messaggi su UART | ⭐ |
| [blink](src/bin/blink.rs) | Lampeggio LED | ⭐ |
| [ws2812](src/bin/ws2812.rs) | RGB rainbow su ws2812 | ⭐⭐ |

### In Arrivo
- **button** - Input digitale e interrupt
- **pwm-led** - Controllo luminosità con PWM  
- **uart-echo** - Comunicazione UART bidirezionale
- **i2c-sensor** - Lettura sensore I2C
- **spi-display** - Controllo display SPI
- **wifi-connect** - Connessione WiFi base
- **ble-beacon** - Beacon BLE
- **async-tasks** - Multi-tasking con Embassy

## 🚀 Quick Start

### 1. Installa Toolchain

```bash
# Installa espup
cargo install espup

# Installa toolchain ESP32
espup install

# Carica variabili d'ambiente (esegui ad ogni nuova sessione terminal)
. ~/export-esp.sh  # Linux/macOS
# O aggiungi a ~/.bashrc per renderlo permanente
```

### 2. Installa Tool di Flash

```bash
cargo install espflash cargo-espflash
```

### 3. Clona e Testa

```bash
# Sostituisci con la tua URL dopo aver creato la repo su GitHub
git clone https://github.com/<your-username>/esp32c6-rust-examples
cd esp32c6-rust-examples

# Esegui l'esempio hello-world
cargo run --release --bin hello-world

# Esegui l'esempio blink
cargo run --release --bin blink
```

## 📦 Struttura del Progetto

```
esp32c6-rust-examples/
├── README.md              # Questo file
├── CONTRIBUTING.md        # Guida per contribuire
├── Cargo.toml             # Configurazione progetto
├── rust-toolchain.toml    # Versione Rust
├── .cargo/
│   └── config.toml        # Configurazione cargo/runner
├── src/
│   ├── lib.rs             # Libreria condivisa (opzionale)
│   └── bin/               # Esempi eseguibili
│       ├── main.rs       # hello-world
│       ├── blink.rs      # blink LED
│       └── ...           # altri esempi
└── docs/
    ├── setup.md          # Setup dettagliato
    └── troubleshooting.md # Risoluzione problemi
```

### Risorse di Apprendimento

- 📚 [The Embedded Rust Book](https://docs.rust-embedded.org/book/)
- 📚 [ESP-RS Book](https://docs.espressif.com/projects/rust/book/)
- 🎥 [ESP32 Embedded Rust at the HAL](https://www.youtube.com/playlist?list=PLkch9g9DEE0Lkm1LqcD7pZNDmXEczOo-a)
- 💬 [ESP-RS Matrix Chat](https://matrix.to/#/#esp-rs:matrix.org)

## 🐛 Troubleshooting

### Errore "Port not found"
```bash
# Linux
sudo usermod -a -G dialout $USER
# Logout e login

# Specifica porta manualmente
cargo run --release -- --port /dev/ttyUSB0
```

### Errore "Permission denied"
```bash
sudo chmod 666 /dev/ttyUSB0
```

## 🙏 Ringraziamenti

- [esp-rs](https://github.com/esp-rs) team per gli HAL e tool eccellenti
- Community Rust per supporto e documentazione
- Tutti i contributor di questa repo


**Buon coding! 🦀✨**