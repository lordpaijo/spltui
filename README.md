# SPLTUI

[![Rust](https://img.shields.io/badge/language-Rust-orange)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/spltui)](https://crates.io/crates/your_crate_name)
[![GitHub stars](https://img.shields.io/github/stars/lordpaijo/spltui)](https://github.com/lordpaijo/spltui/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/lordpaijo/spltui)](https://github.com/lordpaijo/spltui/issues)
[![GitHub forks](https://img.shields.io/github/forks/lordpaijo/spltui)](https://github.com/lordpaijo/spltui/network/members)
[![GitHub commits](https://img.shields.io/github/commit-activity/m/lordpaijo/spltui)](https://github.com/lordpaijo/spltui/commits/main)
[![MIT License](https://img.shields.io/github/license/lordpaijo/spltui)](https://github.com/lordpaijo/spltui/blob/main/LICENSE)

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
- [chrono](https://github.com/chronotope/chrono)

## Instalasi

### Via Cargo (Recommended)
Pastikan Rust dan Cargo sudah terinstal. Jika sudah, jalankan perintah berikut:

```sh
$ cargo install spltui
```

Dan SPLTUI berhasil terinstal.

### Unduh Executable (untuk Windows)
Untuk mengunduh file executable (`.exe`) dari SPLTUI, dapat dilakukan melalui dua platform ini.
- [Dropbox: spltui.exe](https://www.dropbox.com/scl/fi/18bul9bxh8eu6u4nrz1rb/spltui.exe?rlkey=6magnxbz11tno33cy90vyg4qq&st=6n2mka20&dl=0)
- [Mega: spltui.exe](https://mega.nz/file/93U2maQS#STYE598ZXtwMA1iqjhbEvyAuLe0UrtAQKLsx7YgBA8A)

**Peringatan**: Dengan menggunakan file executable, antivirus dapat mendeteksinya sebagai virus dan / hingga menghapusnya.

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
| `-h / --help` | Menampilkan `help`. | Tidak ada | `spltui --help` |
| `-V / --version` | Menampilkan `version`. | Tidak ada | `spltui --version` |
| `-v / --verbose` | Berjalan dengan mode `verbose`. | Tidak ada | `spltui --verbose` |
| `-l / --log` | Mengspesifikasikan dimana file *log* disimpan dalam mode `verbose`. | Nama file `.log` | `spltui --verbose --log contoh_log.log` |
| `-t / --theme` | Berjalan dengan tema yang dipilih. | Tema (i.e, `ligt`, `dark`) | `spltui --theme dark` |
| `--spldv` | Masuk ke `state` SPLDV. | Tidak ada | `spltui --spldv` |
| `--splsv` | Masuk ke `state` SPLSV. | Tidak ada | `spltui --splsv` |
| `--hasil` | Masuk ke `state` hasil (langsung menunjukkan hasil). | Tidak ada | `spltui --splsv` |

---

## Kompatibilitas

| Sistem Operasi | Kestabilan |
| :------------- | :--------: |
| Android (Termux) |       ✅       |
|     IOS (Ish)    |       🛠️       |
|       Linux      |       ✅       |
|       Windows    |       ✅       |
|       MacOS      |       🛠️       |

Selamat mencoba~!
