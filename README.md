# 🗜️ GigliZip

**GigliZip** is a fast, modern, and user-friendly file & folder compression tool built in Rust. It features an interactive terminal interface, high-performance compression algorithms, and native cross-platform support — designed for both developers and everyday users who need efficient, reliable compression.

---

## 🚀 Features

- ✅ **Interactive CLI Prompt** – Simple, guided workflow for compression and decompression
- ⚡ **High-Speed Performance** – Built with Rust using Zstd and LZ4 algorithms
- 📁 **File & Directory Support** – Compress entire folders recursively
- 🔄 **Round-Trip Reliable** – Compress `.ggz` files and restore with full fidelity
- 🖥️ **Cross-Platform** – Runs on Windows, Linux, and macOS
- 📦 **Native Installer for Windows** – Double-click install and run

---



## 📦 Installation

### 🔹 Windows (Recommended)

### 📥 Download GigliZip

Download the latest installer for your platform:






- 🪟 **[Download GigliZip v2.0 for Windows (.exe)](https://github.com/jasgigli/giglizip/releases/latest)**
- 🍎 **[Download for macOS (Coming Soon)]()**
- 🐧 **[Download for Linux (Coming Soon)]()**



Or visit the [Releases Page](https://github.com/jasgigli/GigliZip/releases) for all versions.


1. Run `giglizip-setup.exe`
2. Open **GigliZip** from the Start Menu
3. Use the interactive prompt to compress or decompress files

### 🔹 Cargo (For Rust Developers)

```bash
cargo install giglizip
````

> Requires Rust 1.74 or later

---

## 🧪 Usage

### ▶️ Run in Interactive Mode in CMD


Then follow the prompts:


What would you like to do?: compress
Enter path to source file or folder: C:\Users\JasGigli\Desktop\report.pdf
Enter desired output filename (leave blank to auto-generate):


### ⚙️ Command-Line Usage


# Compress a file
giglizip compress --input file.txt --output file.ggz

# Decompress a file
giglizip decompress --input file.ggz --output file.txt


---

## 📁 Output Format

GigliZip compresses files into a custom `.ggz` format using efficient algorithms like Zstd and LZ4. It supports most file types and full folder structures.

---

## 🔧 Build from Source


git clone https://github.com/jasgigli/giglizip.git
cd giglizip
cargo build --release



Your binary will be located at:

target/release/giglizip.exe




## 📌 Roadmap

* [x] Interactive CLI support
* [x] Windows installer
* [ ] GUI drag-and-drop interface
* [ ] Archive format support (ZIP, TAR)
* [ ] Multi-threaded compression



## 👤 Author

**JasGigli**
🔗 [JasGigli](https://junaidalishah.vercel.app)
💻 [JasGigli](https://github.com/jasgigli)

---

## 📄 License

Licensed under the MIT License. See [`LICENSE`](./LICENSE) for details.




