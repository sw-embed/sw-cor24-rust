# sw-cor24-rust

Rust-to-COR24 translation pipeline. Compiles Rust programs to COR24
machine code via the MSP430 target as a 16-bit intermediate.

## Pipeline

```
Rust source -> rustc (MSP430 target) -> MSP430 asm -> msp430-to-cor24 -> COR24 asm -> assembler -> binary -> emulator
```

## Binaries

- **msp430-to-cor24** -- MSP430 assembly to COR24 translator (primary path)
- **cor24-run** -- COR24 assembler + emulator CLI
- **wasm2cor24** -- WASM to COR24 translator (deprecated, MSP430 path preferred)

## Dependencies

Sibling repos (must be cloned alongside this repo):

- [sw-cor24-emulator](https://github.com/sw-embed/sw-cor24-emulator) -- emulator + ISA
- [sw-cor24-assembler](https://github.com/sw-embed/sw-cor24-assembler) -- assembler

## Build

```bash
./scripts/build.sh
```

## Usage

```bash
# Translate MSP430 assembly to COR24
cargo run --bin msp430-to-cor24 -- input.s -o output.s

# Compile Rust project end-to-end
cargo run --bin msp430-to-cor24 -- --compile ./demos/demo_blinky

# Assemble and run COR24 assembly
cargo run --bin cor24-run -- --run program.s

# Run built-in demo
cargo run --bin cor24-run -- --demo
```

## Demos

The `demos/` directory contains small `#![no_std]` Rust programs targeting
COR24 via the MSP430 translation path. See `demos/generate-all.sh` to build
all demos.

## Register Mapping

| MSP430 | COR24 | Role |
|--------|-------|------|
| r12 | r0 | arg0 / return value |
| r13 | r1 | arg1 |
| r14 | r2 | arg2 |
| r1 | sp | stack pointer |
| r4-r11 | stack | spilled to fp-relative offsets |

## Entry Point Convention

Every program needs `#[no_mangle] pub unsafe fn start()` as its entry
point. The translator emits a reset vector (`la r0, start` + `jmp (r0)`)
at address 0 to jump to `start`.
