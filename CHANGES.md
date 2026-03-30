# Changelog

## 2026-03-29 — Repository extraction

Extracted from the monolithic cor24-rs repository as part of the COR24
ecosystem refactoring (sw-cor24-project saga step 004).

### Changes

- Moved `rust-to-cor24/` contents to repository root
- Removed all non-pipeline code (emulator, assembler, web UI, ISA, CLI tools)
- Renamed package from `wasm2cor24` to `sw-cor24-rust`
- Updated dependencies to use sibling repos:
  - `cor24-assembler` from `../sw-cor24-assembler`
  - `cor24-emulator` from `../sw-cor24-emulator`
- Marked wasm2cor24 binary as deprecated (MSP430 path is preferred)
- Added `scripts/build.sh`
- All 34 tests pass (22 unit + 12 integration)
