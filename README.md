# SPLTUI

Sistem Persamaan Linear Terminal User Interface, atau SPLTUI, merupakan sebuah aplikasi kalkulator untuk menghitung sistem persamaan linear dengan berbagai variabel berbasis *terminal user interface* atau TUI.

[![demonstrasi](https://github.com/lordpaijo/spltui/blob/master/ss-0.png)](https://youtu.be/C3TQK1qg3wk)

---

## Dependensi

- [Rust (& Cargo)](https://www.rust-lang.org/)
- [matematika.rs](https://github.com/lordpaijo/matematika.rs)
- [ratatui](https://ratatui.rs/)
- [crossterm](https://github.com/crossterm-rs/crossterm)
- [log](https://crates.io/crates/log)
- [env_logger](https://crates.io/crates/env_logger)

## Instalasi

Pastikan Rust dan Cargo sudah terinstal. Jika sudah, jalankan perintah berikut:

```sh
$ cargo install spltui
```

Dan SPLTUI berhasil terinstal.

## Penggunaan

Aplikasi ini berjalan di terminal. Untuk menjalankannya, cukup panggil SPLTUI di terminal Anda.

```sh
$ spltui
```

Tampilan seperti yang ada di [video demonstrasi](https://youtu.be/C3TQK1qg3wk) akan muncul.

### Command Line Arguments
Adapun komponen tambahan *CLI Args* dengan penggunaannya sebagai berikut:
```sh
$ spltui --help
```

| Argumen | Fungsi | Parameter | Contoh |
|---------|--------|:---------:|--------|
| `-h / --help` | Menamimplkan `help`. | Tidak ada | `spltui --help` |
| `-V / --version` | Menamimplkan `version`. | Tidak ada | `spltui --version` |
| `-v / --verbose` | Berjalan dengan mode `verbose`. | Tidak ada | `spltui --verbose` |
| `--log-file` | Mengspesifikasikan dimana file *log* disimpan dalam mode `verbose`. | Nama file `.log` | `spltui --verbose --log-file contoh_log.log` |
| `--spldv` | Masuk ke `state` SPLDV. | Tidak ada | `spltui --spldv` |
| `--splsv` | Masuk ke `state` SPLSV. | Tidak ada | `spltui --splsv` |
| `--hasil` | Masuk ke `state` hasil (langsung menunjukkan hasil). | Tidak ada | `spltui --splsv` |

---

## Kompatibilitas

| Sistem Operasi | Kestabilan |
| :------------- | :--------: |
| Linux          |     ✅     |
| Windows (cmd)  |     ✅     |
| Windows (powershell) | ⚠️    |
| MacOS          |     🛠️     |

Selamat mencoba~!
