# Planned Simplest: WASM → COR24

This document describes the planned approach for the simplest path to compile Rust to COR24.

## Chosen Approach: WASM as Intermediate Format

```
Rust Source (.rs)
    │
    ▼ rustc --target wasm32-unknown-unknown
    │
WASM Binary (.wasm)
    │
    ▼ wasm2cor24 (our tool)
    │
COR24 Assembly (.s)
    │
    ▼ cor24 assembler (existing)
    │
COR24 Binary
```

## Why WASM?

1. **Zero rustc modifications** - Use standard toolchain
2. **Well-documented format** - WASM spec is clear and stable
3. **Existing tools** - `wasmparser`, `walrus` crates for parsing
4. **Proven for embedded** - WASM runs on microcontrollers already
5. **Stack-based simplifies register allocation** - We control the mapping

## Project Structure

```
rust-to-cor24/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Library interface
│   ├── main.rs          # CLI: wasm2cor24
│   ├── wasm_parser.rs   # Parse WASM binary
│   ├── translator.rs    # WASM → COR24 IR
│   ├── codegen.rs       # COR24 IR → Assembly
│   ├── runtime.rs       # Runtime support (mul, div, etc.)
│   └── abi.rs           # Calling convention
├── runtime/
│   └── cor24_runtime.s  # Assembly runtime (softmul, etc.)
├── tests/
│   └── *.rs             # Integration tests using emulator
└── docs/
    ├── possible-futures.md
    └── planned-simplest.md  # This file
```

## Implementation Phases

### Phase 1: Minimal Translator (MVP)

**Goal:** Translate simplest WASM to COR24

**Supported WASM subset:**
- `i32.const`, `i32.add`, `i32.sub`
- `local.get`, `local.set`
- `call`, `return`
- `block`, `br`, `br_if`

**Example input (Rust):**
```rust
#![no_std]
#![no_main]

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**Compiles to WASM:**
```wasm
(func $add (param i32 i32) (result i32)
  local.get 0
  local.get 1
  i32.add)
```

**Translates to COR24:**
```asm
; add(a: r0, b: r1) -> r0
add:
    add     r0, r1
    jmp     (r1)        ; return (r1 = return addr)
```

**Deliverable:** `wasm2cor24` CLI that handles this case

### Phase 2: Control Flow

**Add support for:**
- `if`, `else`, `end`
- `loop`, `br`, `br_if`, `br_table`
- `block` nesting

**WASM control flow → COR24 branches:**
```wasm
(if (result i32)
  (local.get 0)
  (then (i32.const 1))
  (else (i32.const 0)))
```

**Becomes:**
```asm
    ; if r0 != 0
    ceq     r0, z
    brt     else_0
    lc      r0, 1
    bra     end_0
else_0:
    lc      r0, 0
end_0:
```

### Phase 3: Memory Operations

**Add support for:**
- `i32.load`, `i32.store`
- `i32.load8_s`, `i32.load8_u`
- `i32.store8`
- Linear memory layout

**Memory mapping:**
```
COR24 Emulation Layout (64KB subset of 16MB):
0x0000 - 0x00FF: Reserved (vectors, etc.)
0x0100 - 0x7FFF: WASM linear memory (32KB)
0x8000 - 0xFEFF: Stack (grows down)
0xFF00 - 0xFFFF: I/O (if applicable)
```

**WASM memory ops → COR24:**
```wasm
(i32.load offset=4 (local.get 0))
```

**Becomes:**
```asm
    ; r0 = mem[r0 + 4]
    add     r0, 4
    lw      r0, 0(r0)
```

### Phase 4: Full Integer Operations

**Add support for:**
- `i32.mul` → software multiply
- `i32.div_s`, `i32.div_u` → software divide
- `i32.rem_s`, `i32.rem_u` → software modulo
- `i32.and`, `i32.or`, `i32.xor`
- `i32.shl`, `i32.shr_s`, `i32.shr_u`
- Comparison ops

**Runtime library (`runtime/cor24_runtime.s`):**
```asm
; Software multiply: r0 = r0 * r1
; Uses r2 as temp
__mul:
    push    r2
    lc      r2, 0           ; result = 0
mul_loop:
    ceq     r1, z
    brt     mul_done
    ; if r1 & 1: result += r0
    ; ... shift and add algorithm
mul_done:
    mov     r0, r2
    pop     r2
    jmp     (ir)
```

### Phase 5: Function Calls & Stack

**Calling convention:**
```
Arguments:  r0, r1, r2, then stack (right to left)
Return:     r0
Saved:      fp (callee), r0-r2 (caller)
Link:       r1 used for return address in simple calls
```

**Function prologue/epilogue:**
```asm
func:
    push    fp
    mov     fp, sp
    ; ... allocate locals ...

    ; ... body ...

    mov     sp, fp
    pop     fp
    jmp     (r1)
```

## 32-bit to 24-bit Handling

WASM uses i32, COR24 is 24-bit. Strategy:

1. **Mask on store:** When storing to memory, mask to 24 bits
2. **Sign-extend on load:** Use `lb`/`lbu` appropriately
3. **Overflow behavior:** Let it wrap naturally (same as WASM)

```asm
; Mask r0 to 24 bits after 32-bit operation
    la      r1, 0xFFFFFF
    and     r0, r1
```

For most embedded use cases, values stay within 24-bit range.

## Testing Strategy

Use the existing cor24-rs emulator:

```rust
#[test]
fn test_add_function() {
    // Compile Rust to WASM
    let wasm = compile_to_wasm("fn add(a: i32, b: i32) -> i32 { a + b }");

    // Translate to COR24
    let asm = wasm2cor24::translate(&wasm);

    // Assemble
    let binary = cor24_assembler::assemble(&asm);

    // Run on emulator
    let mut cpu = CpuState::new();
    cpu.load_program(&binary);
    cpu.set_reg(0, 5);  // a = 5
    cpu.set_reg(1, 3);  // b = 3
    cpu.run();

    assert_eq!(cpu.get_reg(0), 8);
}
```

## Milestones

| Milestone | Description | Est. Hours |
|-----------|-------------|------------|
| M1 | Parse WASM, emit skeleton | 20 |
| M2 | Arithmetic ops working | 40 |
| M3 | Control flow (branches) | 40 |
| M4 | Memory load/store | 40 |
| M5 | Function calls | 40 |
| M6 | Runtime library | 40 |
| M7 | Full i32 ops | 40 |
| M8 | Polish & docs | 20 |
| **Total** | | **280** |

## Dependencies

```toml
[dependencies]
wasmparser = "0.121"     # Parse WASM binary
anyhow = "1.0"           # Error handling
clap = "4.0"             # CLI parsing

[dev-dependencies]
cor24-emulator = { path = "../" }  # For testing
```

## Next Steps

1. [ ] Create `rust-to-cor24/Cargo.toml`
2. [ ] Implement basic WASM parser wrapper
3. [ ] Create COR24 IR types
4. [ ] Implement `i32.add` translation
5. [ ] Test with emulator
6. [ ] Iterate on more opcodes

## Open Questions

1. **Stack size:** How much stack to reserve? 8KB? 16KB?
2. **Globals:** Where to place WASM globals in memory?
3. **Indirect calls:** Support `call_indirect`? (tables)
4. **Interrupts:** How does WASM code interact with COR24 interrupts?

## Future: I2C and Sensor Simulation

### Goal
Demonstrate a complete embedded workflow: Rust driver → COR24 assembly → emulator with simulated I2C sensor.

### Simulated Hardware
- **BMP280/BME280** pressure/temperature sensor
- Memory-mapped I2C controller at 0xFFFE00
- Sensor connected at I2C address 0x76

### I2C Controller Registers
```
0xFFFE00: I2C_DATA    - Read/write data byte
0xFFFE01: I2C_STATUS  - Bit 0: busy, Bit 1: NACK, Bit 2: done
0xFFFE02: I2C_CONTROL - Bit 0: start, Bit 1: stop, Bit 2: read/write
0xFFFE03: I2C_ADDR    - 7-bit slave address
```

### 24-bit Challenges
Sensors like BMP280 return 20-bit pressure readings in 3 bytes:
- COR24 is 24-bit native, so raw ADC values fit
- Compensation formulas use 32-bit math
- Solution: Use 24-bit for raw data, implement 32-bit math in software or split calculations

### Rust Driver Approach
```rust
#![no_std]

pub fn read_pressure() -> u32 {
    // Select pressure register
    i2c_write(0x76, &[0xF7]);

    // Read 3 bytes: MSB, LSB, XLSB
    let mut buf = [0u8; 3];
    i2c_read(0x76, &mut buf);

    // Combine into 20-bit raw value
    let raw = ((buf[0] as u32) << 12)
            | ((buf[1] as u32) << 4)
            | ((buf[2] as u32) >> 4);

    // Apply compensation (simplified)
    compensate_pressure(raw)
}
```

### Implementation Steps
1. Add I2C memory-mapped peripheral to emulator
2. Implement BMP280 sensor simulation with realistic values
3. Write Rust no_std I2C driver
4. Compile to WASM → translate to COR24
5. Run on emulator, display temperature/pressure readings

## References

- [WASM Binary Format](https://webassembly.github.io/spec/core/binary/)
- [wasmparser crate](https://docs.rs/wasmparser/)
- [COR24 ISA Reference](../docs/isa-reference.md)
- [Possible Futures](./possible-futures.md)
