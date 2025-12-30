# RustBox

## Abstract

RustBox is a portable, lightweight, MSVC-compatible Rust development environment for Windows. The project is designed to provide a Linux-like Rust workflow on Windows without requiring the installation of Visual Studio IDE, system-wide environment changes, or administrative privileges.

RustBox bundles a minimal Rust toolchain, MSVC components, and Windows SDK libraries, and exposes them through a CLI wrapper that prepares the required environment variables at runtime.

---

## Objectives

The primary objectives of RustBox are:

1. Provide full compatibility with the `x86_64-pc-windows-msvc` Rust target.
2. Eliminate the need for Visual Studio IDE installation.
3. Minimize disk usage compared to standard Rust + Visual Studio setups.
4. Offer a portable, relocatable development environment.
5. Maintain a CLI-first workflow suitable for learning, backend development, and systems programming.

---

## Scope

RustBox focuses on Rust development for Windows using the MSVC ABI. The project does not aim to replace Cargo or Rust itself, nor does it attempt to provide an IDE, debugger UI, or cross-compilation features.

---

## Package Structure

A RustBox release package has the following structure:

```
rustbox/
├─ bin/
│  └─ rustbox.exe
├─ toolchain/
│  ├─ rust/          # Portable Rust compiler and standard library
│  ├─ msvc/          # Minimal MSVC toolchain (compiler frontend and linker)
│  └─ windows-sdk/   # Minimal Windows SDK (headers and libraries)
└─ README.md
```

The release package is distributed as a ZIP archive and does not modify the host system.

---

## Usage Guide

### Installation

1. Download the `rustbox-win64.zip` archive from the GitHub Releases page.
2. Extract the archive to any directory (e.g., `D:\tools\rustbox`).
3. Open a terminal (PowerShell or Command Prompt).

### Initialization

Run the following command to initialize the RustBox environment:

```powershell
bin\rustbox.exe init
```

This command validates the bundled toolchains and prepares the runtime environment.

### Creating and Running a Project

```powershell
bin\rustbox.exe new hello
cd hello
bin\rustbox.exe run
```

No external Rust installation or Visual Studio IDE is required.

---

## Internal Architecture

RustBox operates as a wrapper around Cargo and rustc. It does not replace these tools, but instead injects the required environment variables at runtime.

```
User Command
  ↓
rustbox.exe
  ↓ (PATH, INCLUDE, LIB injection)
Cargo and rustc (bundled)
  ↓
MSVC linker and Windows SDK libraries
```

All environment modifications are temporary and scoped to the RustBox process.

---

## Development Workflow

### Development Environment

* Development is performed in WSL (Linux).
* The Git repository contains only source code.
* Toolchains and build artifacts are excluded from version control.

Typical development commands:

```bash
cargo build
cargo run -- init
```

### Windows Release Build

The final `rustbox.exe` binary must be built on Windows native, not in WSL:

```powershell
cargo build --release
```

The resulting binary is located at:

```
target\release\rustbox.exe
```

---

## Disk Usage

Approximate disk usage of the bundled components:

| Component           | Size (approx.) |
| ------------------- | -------------- |
| Rust toolchain      | ~200 MB        |
| MSVC minimal        | ~380 MB        |
| Windows SDK minimal | ~330 MB        |
| **Total**           | **< 1 GB**     |

This is significantly smaller than a standard Rust + Visual Studio IDE installation.

---

## Limitations

* RustBox currently supports Windows x64 only.
* Cross-compilation is not supported.
* Debugger GUIs are not included.
* Windows binaries cannot be executed inside WSL.

---

## Licensing Notes

* RustBox source code is released under an open-source license.
* Rust toolchains are distributed under the Rust project licenses.
* MSVC and Windows SDK components are redistributed in accordance with Microsoft licensing terms.
