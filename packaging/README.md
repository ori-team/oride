# Oride Distribution Packaging (`packaging/`)

This directory contains packaging specifications, recipes, and templates for distributing **Oride** across diverse Linux distributions, package managers, and operating systems.

---

## Supported Ecosystems

| Distribution / Target | Format / Spec | Location | Install / Build Command |
|---|---|---|---|
| **Arch Linux** / Manjaro / EndeavourOS | `PKGBUILD` | [`packaging/arch/PKGBUILD`](arch/PKGBUILD) | `cd packaging/arch && makepkg -si` |
| **Fedora** / RHEL / CentOS / Rocky | RPM Spec | [`packaging/fedora/oride.spec`](fedora/oride.spec) | `rpmbuild -ba packaging/fedora/oride.spec` |
| **Debian** / Ubuntu / Pop!_OS / Mint | `.deb` package | [`packaging/debian/`](debian/) | `./packaging/debian/build-deb.sh` |
| **Void Linux** | `xbps-src` template | [`packaging/void/template`](void/template) | `./xbps-src pkg oride` |
| **Nix** / NixOS | Flake / Nix expression | [`flake.nix`](../flake.nix) · [`default.nix`](../default.nix) | `nix run github:ori-team/oride` |
| **Windows** (PowerShell) | One-line installer | [`scripts/install.ps1`](../scripts/install.ps1) | `irm https://raw.githubusercontent.com/ori-team/oride/main/scripts/install.ps1 \| iex` |
| **Universal Linux & macOS** | Universal installer | [`scripts/install.sh`](../scripts/install.sh) | `curl -fsSL https://raw.githubusercontent.com/ori-team/oride/main/scripts/install.sh \| bash` |

---

## Pre-compiled Release Binaries

Official pre-built binaries and packages for Linux (glibc and musl static), Windows (x64, x86, ARM64), and macOS are automatically generated on every tag release via GitHub Actions:
👉 **[https://github.com/ori-team/oride/releases](https://github.com/ori-team/oride/releases)**
