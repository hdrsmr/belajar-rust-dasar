# Belajar Rust Dasar

Project latihan **Rust dasar** — variabel, mutabilitas, shadowing, tipe numerik, konversi tipe, dan operator aritmatika.

## Prasyarat

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024, butuh Rust 1.85+)
- Cek instalasi:

```bash
rustc --version
cargo --version
```

## Menjalankan

```bash
# Jalankan program
cargo run

# Jalankan semua test
cargo test

# Build release
cargo build --release
```

## Struktur Project

```
belajar-rust-dasar/
├── Cargo.toml      # Manifest project & dependencies
├── Cargo.lock      # Version lock dependencies
├── .gitignore
├── src/
│   └── main.rs     # Kode utama & test
└── README.md
```

## Materi yang Dipelajari

Semua materi dicoba melalui test di `src/main.rs`:

| Test | Topik |
|------|-------|
| `hello_test` | Dasar program Rust |
| `test_variable` | Deklarasi variabel `let` |
| `test_mutable` | Variabel mutable `mut` |
| `static_text` | Imutability (default) |
| `shadowing` | Shadowing variabel |
| `explicit` | Anotasi tipe eksplisit |
| `number` | Tipe numerik (`i8`, `f32`) |
| `number_conversion` | Konversi tipe dengan `as` |
| `numeric_operator` | Operator aritmatika (`+ - * /`) |

### Catatan Integer

Rust punya 12 tipe integer: `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (bertanda) dan `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (tidak bertanda).

- Default integer literal: **`i32`**
- `usize` dipakai untuk indexing, lebarnya mengikuti platform (di sistem 64-bit = 64 bit)
- Konversi antar tipe pakai `as`, contoh: `let b: i16 = a as i16;`

## Lisensi

Project untuk keperluan belajar.
