# sw-cor24-rust — Claude Instructions

## Project Overview

Rust-to-COR24 translation pipeline. Two paths: WASM-based (deprecated)
and MSP430-based (preferred). Also includes cor24-run CLI.

## Build & Test

```bash
./scripts/build.sh    # test + release build
cargo test            # 34 tests
```

## File Structure

- `src/lib.rs` — WASM-to-COR24 translator (deprecated)
- `src/msp430.rs` — MSP430-to-COR24 translator (primary)
- `src/pipeline.rs` — end-to-end Rust->WASM->COR24 pipeline
- `src/main.rs` — wasm2cor24 CLI
- `src/msp430_cli.rs` — msp430-to-cor24 CLI
- `src/run.rs` — cor24-run CLI (assemble + emulate)
- `demos/` — small Rust demo programs

## Dependencies

- `cor24-assembler` (sibling: `../sw-cor24-assembler`)
- `cor24-emulator` (sibling: `../sw-cor24-emulator`)

## Commit Discipline

Write clear commit messages. Run `cargo test` before committing.
