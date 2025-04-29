# rust-on-riscv-starter-kit 🦀🚀

A minimal bare-metal Rust "Hello, RISC-V" starter project.

This project runs on the 32-bit RISC-V `virt` machine in QEMU and prints `Hello, RISC-V!` to the UART console.
No `std`, no external crates, no magic - just you, Rust, and the hardware.

## 🛠 Requirements

- Rust toolchain (`rustup`) with the RISC-V target:
  ```
  rustup target add riscv32imac-unknown-none-elf
  ```
- QEMU with RISC-V support:
  ```
  sudo apt install qemu-system-misc
  ```

## ▶️ Running the Code

1. Clone the repository:
   ```
   git clone https://github.com/ddrcode/rust-on-riscv-starter-kit
   cd rust-on-riscv-starter-kit
   ```

2. Build the binary:
   ```
   cargo build --release
   ```

3. Run it in QEMU:
   ```
   qemu-system-riscv32 -machine virt -nographic -bios none -kernel target/riscv32imac-unknown-none-elf/release/hello-riscv
   ```

You should see:

```
Hello, RISC-V!
```

## 🧠 What’s Inside

- `#![no_std]`, `#![no_main]`
- Direct MMIO write to UART
- Custom panic handler
- Manual linker script (`linker.ld`)
- No runtime, no external dependencies

## 📚 Why This Exists

To give you a clean and minimal starting point for:
- Learning how Rust works without an OS
- Exploring RISC-V memory-mapped I/O
- Building your own microkernel or embedded project

## 🔒 Safety Note

This project uses `unsafe` — but in a minimal, contained way to talk to hardware.
The goal is to show how Rust can *own* bare-metal safely, step by step.

## 🧩 Next Ideas

Want to go further? Try:
- Moving to user mode + `ecall`
- Adding interrupt handling
- Printing from panic info
- Porting to a real RISC-V board

---

Made with ❤️ and `panic()` by [@ddrcode](https://github.com/ddrcode)
