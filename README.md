# Rust-Lockpick
Rust-Lockpick aims to be an API that generates configurations of pin-tumbler lock keyways.

## Intended Output
 1 2 3 4 5 6  
▏░ ░ ░ ░ ░ ░▕  
▏▚ ▇ ▇ ▜ ▃ ▇▕  
▏█ ▄ ▇ ▄ ▃ ▀▕ 🗝  
▏▼ ▼ ▼ ▼ ▼ ▼▕  

Each column is a row of pins with differing Unicode characters that try their best to represent the generated keyway configuration.

## Legend
█ - full key pin
▼

█ - full driver pin

▄ - half driver pin

▀ - 0 move pin

▚ - serrated pin

▜ - spool pin

░ - regular spring

▕ & ▕ - chamber walls