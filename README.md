# RustBox

## Abstract

RustBox is a portable, lightweight, MSVC-compatible Rust development environment for Windows. The project is designed to provide a Linux-like Rust workflow on Windows without requiring the installation of the Visual Studio IDE, system-wide environment changes, or administrative privileges.

RustBox bundles a minimal Rust toolchain, MSVC components, and Windows SDK libraries, and exposes them through a CLI wrapper that prepares the required environment variables at runtime.

---

## Objectives

The primary objectives of RustBox are:

1. Provide full compatibility with the `x86_64-pc-windows-msvc` Rust target.
2. Eliminate the need for Visual Studio IDE installation.
3. Minimize disk usage compared to standard Rust + Visual Studio setups.
4. Offer a portable and relocatable development environment.
5. Maintain a CLI-first workflow suitable for learning, backend development, and systems programming.

---

## Scope

RustBox focuses on Rust development for Windows using the MSVC ABI. The project does **not** aim to replace Cargo or Rust itself, nor does it attempt to provide an IDE, debugger UI, or cross-compilation features.

RustBox acts purely as an **environment wrapper** that prepares the required runtime environment for Cargo and `rustc`.

---

## Source Code Structure

The RustBox source repository contains **only source code**. Bundled toolchains, binaries, and release artifacts are **not** part of the Git repository.

```text
src/
├─ cli/            # CLI parsing and command dispatch
├─ commands/       # High-level RustBox commands (cargo passthrough, init, new)
├─ toolchain/      # Toolchain detection, layout, and environment injection
├─ config/         # Future configuration support (rustbox.toml)
├─ utils/          # Shared helpers
├─ error.rs        # Centralized error definitions
└─ main.rs         # Application entry point
```

This separation keeps the repository lightweight and easy to maintain.

---

## Release Package Structure

A RustBox release package (ZIP) has the following structure:

```text
rustbox/
├─ bin/
│  └─ rustbox.exe
├─ toolchain/
│  ├─ rust/          # Portable Rust compiler and standard library
│  ├─ msvc/          # Minimal MSVC toolchain (compiler frontend and linker)
│  └─ windows-sdk/   # Minimal Windows SDK (headers and libraries)
├─ cargo-home/       # Cargo home directory (may be empty)
└─ README.md
```

The release package is fully portable and does not modify the host system.

---

## Usage Guide

### Installation

1. Download the `rustbox-v0.1.0-mvp-windows-x64.zip` archive from the GitHub Releases page.
2. Extract the archive to any directory (for example: `D:\\tools\\rustbox`).
3. Open a terminal (PowerShell or Command Prompt).

No installer, administrative privileges, or system-wide configuration changes are required.

---

### Initialization

Run the following command to validate the RustBox environment:

```powershell
bin\\rustbox.exe init
```

This command verifies the bundled toolchains and prepares the runtime environment.

---

### Creating and Running a Project

Create a new Rust project:

```powershell
bin\\rustbox.exe new hello
cd hello
```

Build and run the project using Cargo passthrough:

```powershell
bin\\rustbox.exe cargo run
```

> **Note**: The `run` command is provided as a convenience alias for `cargo run`.

---

## Cargo Behavior

RustBox delegates all build, test, and package management operations to Cargo.

In **v0.1.0-mvp**, Cargo uses the default user `CARGO_HOME` location (for example, `C:\\Users\\<user>\\.cargo`). This means that binaries installed using `cargo install` will be placed in the standard Cargo directory.

Fully portable Cargo home support (isolated `CARGO_HOME`) is planned for a future release.

---

## Internal Architecture

RustBox operates as a lightweight wrapper around Cargo and `rustc`. It does not replace these tools, but instead injects the required environment variables at runtime.

```text
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

RustBox can be developed on Linux or WSL. However, Windows-native builds are required for producing release binaries.

The Git repository contains only source code. Toolchains and build artifacts are excluded from version control.

Typical development commands:

```bash
cargo build
cargo run -- init
```

---

### Windows Release Build

The final `rustbox.exe` binary must be built on Windows native (not in WSL):

```powershell
cargo build --release
```

The resulting binary is located at:

```text
target\\release\\rustbox.exe
```

---

## Adding RustBox to PATH (Optional)

To run `rustbox` from any directory, add the following path to your **user** PATH environment variable:

```text
<rustbox_root>\\bin
```

Steps (Windows):

1. Open *Environment Variables*
2. Edit *User PATH*
3. Add the path above
4. Restart your terminal

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

* Windows x64 only
* Cross-compilation is not supported
* Debugger GUIs are not included
* Windows binaries cannot be executed inside WSL

---

## Licensing Notes

* RustBox source code is released under an open-source license.
* Rust toolchains are distributed under the Rust Project licenses.
* MSVC and Windows SDK components are redistributed in accordance with Microsoft licensing terms.
