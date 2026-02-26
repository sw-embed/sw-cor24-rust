# Known Issues

## Assembler

### Incomplete Instruction Encodings

The assembler has incomplete instruction encodings. Many instructions won't assemble correctly because the decode ROM is not fully populated.

**Root Cause**: Instruction byte encodings were reverse-engineered from assembly listings (fib.lst, hello.lst, loadngo.lst, sieve.lst), but not all register/operand combinations are covered.

**Affected Instructions**:
- `sub ra,rb` - Only `sub sp,imm24` is implemented
- `mul ra,rb` - Placeholder encoding only
- `and`, `or`, `xor` - Limited register combinations
- `shl`, `sra`, `srl` - Placeholder encodings
- `ceq`, `cls`, `clu` - Limited register combinations
- `lb`, `lbu`, `lw` - Only some base register combinations
- `sb`, `sw` - Only some base register combinations
- `jal` - Only `jal r1,(r0)` is implemented
- `jmp` - Limited indirect register options

**Solution**: Need to extract the full decode ROM from the Verilog source (cor24_cpu.v) or obtain additional documentation. The dis_rom.v file contains decode information that needs to be integrated.

### Forward Reference Resolution

Branch forward references work but have a limited range (-128 to +127 bytes from the next instruction).

## Decode ROM

### Missing Entries

The DecodeRom in `src/cpu/state.rs` only contains entries discovered from:
- Assembly listing files
- Verilog source analysis

Many valid instruction byte values will return `0xFFF` (invalid) because they haven't been mapped yet.

**To Add More Entries**:
1. Analyze additional .lst files for instruction encodings
2. Cross-reference with dis_rom.v decode patterns
3. Add entries to `DecodeRom::new()` in state.rs

## CPU Execution

### Halt Instruction

Currently `halt` is implemented as jumping to address 0, which relies on there being an infinite loop at that location. This matches the COR24-TB convention but may not be intuitive.

### Interrupt Handling

Interrupt handling (iv, ir registers) is defined but not fully tested. The UART interrupt example from references shows the pattern but the emulator doesn't simulate external interrupts.

## Web UI

### Memory Viewer

- Only shows first 128 bytes
- No scrolling to view full 64KB
- No memory editing capability

### Registers Panel

- Shows all 8 registers but special register visualization could be improved
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

### No Unit Tests

The project has minimal test coverage:
- `src/assembler.rs` - 2 basic tests (lc, push/pop)
- `src/cpu/state.rs` - 3 basic tests (new, memory ops, sign extend)
- `src/cpu/executor.rs` - 2 basic tests (add_immediate, lc)

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
