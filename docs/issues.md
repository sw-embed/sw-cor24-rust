# Known Issues

## Assembler

### ~~Incomplete Instruction Encodings~~ (RESOLVED)

**Status**: Fixed on 2026-02-28

The assembler now uses encoding tables generated from the decode ROM. All 211 valid instruction encodings from the hardware are now supported.

**Implementation**:
- `src/cpu/encode.rs` - Auto-generated encoding functions
- `scripts/extract_decode_rom.py` - Generates both decode and encode tables
- Assembler rewritten to use `encode::encode_*` functions

**Supported register combinations** (from hardware decode ROM):
- `add ra,rb` - 12 combinations (r0-r2, fp)
- `sub ra,rb` - 6 combinations (r0-r2)
- `mul ra,rb` - 9 combinations (r0-r2)
- `and/or/xor ra,rb` - 6 combinations each (r0-r2)
- `shl/sra/srl ra,rb` - 6 combinations each (r0-r2)
- `ceq/cls/clu ra,rb` - 6-9 combinations including z register
- `lb/lbu/lw ra,offset(rb)` - 12 combinations each (r0-r2, fp)
- `sb/sw ra,offset(rb)` - 9-12 combinations (r0-r2, fp)
- `jal ra,(rb)` - 3 combinations
- `jmp (ra)` - 4 combinations (r0-r2, r7)
- `mov ra,rb` - 16 combinations
- `push/pop ra` - 4 combinations (r0-r2, fp)

### Forward Reference Resolution

Branch forward references work but have a limited range (-128 to +127 bytes from the next instruction).

## Decode ROM

### ~~Missing Entries~~ (RESOLVED)

**Status**: Fixed on 2026-02-28

The decode ROM has been fully extracted from `dis_rom.v` Verilog source using `scripts/extract_decode_rom.py`. The ROM now contains **211 valid instruction entries** (out of 256 possible byte values).

**Implementation**:
- `src/cpu/decode_rom.rs` - Auto-generated const array
- `scripts/extract_decode_rom.py` - Extraction script for regeneration

The remaining 45 invalid entries (0xD3-0xFF) are genuinely undefined in the hardware.

## CPU Execution

### Halt Convention

~~Currently `halt` is implemented as jumping to address 0.~~ Fixed: COR24 has no halt
instruction. All examples now use the standard idiom `halt: bra halt` (branch-to-self).
The `halt` pseudo-instruction has been removed from the assembler since it is not
supported by the reference `as24` toolchain.

### Interrupt Handling

Interrupt handling (iv, ir registers) is defined but not fully tested. The UART interrupt example from references shows the pattern but the emulator doesn't simulate external interrupts.

## Web UI

### Rust Pipeline - Pre-built Examples Only

The Rust tab currently shows pre-built examples demonstrating the Rust→WASM→COR24 pipeline. Live compilation requires:

- **Server-side compilation**: Need a backend server running rustc with `wasm32-unknown-unknown` target and the wasm-to-cor24 translator
- **WebSocket or HTTP API**: For sending Rust source and receiving compiled artifacts
- **Docker container**: To sandbox compilation safely

Currently using pre-built examples (LED Blink, Add Function) until server infrastructure is implemented.

### Memory Viewer

- ~~Only shows first 128 bytes~~ Now shows full regions with sparse representation
- ~~Full-region display with zero-row collapsing~~ **Implemented**: SRAM (1MB), EBR/Stack (3KB), and I/O regions shown in their entirety using `SparseMemory` data structure. Consecutive all-zero 16-byte rows are collapsed into summary lines (e.g., `000030–0000FF: (208 bytes zero)`). Non-zero rows within collapsed ranges split into before-summary, data row, after-summary.
- Example programs (`docs/examples/oom.s`, `docs/examples/stack_overflow.s`) demonstrate the feature
- No memory editing capability

### Registers Panel

- Shows all registers but special register visualization could be improved
- Condition flag (C) shown in legend, could be more prominent

## Build/Deployment

### GitHub Pages

No GitHub Actions workflow for automatic deployment yet. Need to add `.github/workflows/deploy.yml`.

### Trunk Warning

Trunk shows deprecation warning about `address` field - should migrate to `addresses` field in Trunk.toml.

## Documentation

### Missing README

No README.md file in repository root. Should include:
- Project description and features
- Architecture overview
- Build instructions
- Usage examples
- Screenshots
- License information

### Screenshots

Only one screenshot exists (`images/cor24-interface-2026-02-26T05-07-30-868Z.png`). Need additional screenshots showing:
- Example programs running
- Step-through debugging
- Challenge mode
- Modal dialogs (Tutorial, ISA Reference, Help)

## Testing

### Test Coverage (183 tests)

- `src/assembler.rs` — 23 tests (all instruction types, labels, integration)
- `src/cpu/state.rs` — 22 tests (memory regions, I/O, UART, push/pop, EBR)
- `src/cpu/executor.rs` — 99 tests (all instruction execution paths, branches, comparisons, stack ops, load/store, shifts, sieve)
- `src/cpu/decode_rom.rs` — 5 tests (valid count, add, branch, push/pop, invalid)
- `src/cpu/encode.rs` — 5 tests (add, push/pop, mov, branch, lc)
- `src/emulator.rs` — 9 tests (breakpoints, run/step, I/O, sieve)
- `src/loader.rs` — 8 tests (LGO parsing and loading)
- `tests/integration_tests.rs` — 16 tests (full programs: LED, UART, sieve, Fibonacci, Multiply, halt detection, memory access, OOM, stack overflow)

### No CI Pipeline

No GitHub Actions workflow for running tests on push/PR.
