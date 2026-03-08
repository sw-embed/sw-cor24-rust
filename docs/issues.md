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

### Halt Instruction

Currently `halt` is implemented as jumping to address 0, which relies on there being an infinite loop at that location. This matches the COR24-TB convention but may not be intuitive.

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

- ~~Only shows first 128 bytes~~ Now shows three regions: Program (0→program_end), Stack (around SP), I/O (0xFFFF80→)
- No scrolling to view full 16MB address space
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

### Limited Unit Tests

The project has moderate test coverage (32 tests total):
- `src/assembler.rs` - 17 tests (all instruction types)
- `src/cpu/state.rs` - 3 tests (new, memory ops, sign extend)
- `src/cpu/executor.rs` - 2 tests (add_immediate, lc)
- `src/cpu/decode_rom.rs` - 5 tests (valid count, add, branch, push/pop, invalid)
- `src/cpu/encode.rs` - 5 tests (add, push/pop, mov, branch, lc)

**Missing Test Coverage**:
- All instruction execution paths
- Branch/jump instructions
- Stack operations (push/pop)
- Memory load/store operations
- Compare instructions and condition flag
- Forward reference resolution in assembler
- Error handling paths

### No Integration Tests

No tests for:
- Full program assembly and execution
- Challenge validation
- WASM bindings

### No CI Pipeline

No GitHub Actions workflow for running tests on push/PR.
