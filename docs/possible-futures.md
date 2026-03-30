# Rust to COR24: Possible Approaches

This document surveys all viable approaches to compile Rust (or Rust-like code) to COR24 assembly.

## Summary Table

| Approach | Effort | Rust Support | Complexity |
|----------|--------|--------------|------------|
| WASM → COR24 | Low | Full no_std | Medium |
| Proc-macro DSL | Very Low | Syntax only | Low |
| MIR → Assembly | Medium | Subset | High |
| Cranelift Backend | High | Full | High |
| LLVM Backend | Very High | Full | Very High |
| mrustc → C → COR24 | Low | Limited | Medium |

---

## Option 1: WASM → COR24 Translator (Recommended Simplest)

**Pipeline:**
```
Rust Source (.rs)
    ↓ rustc (standard toolchain)
WASM Binary (.wasm)
    ↓ wasm-to-cor24 translator
COR24 Assembly (.s)
    ↓ cor24 assembler
Binary
```

**Why this is attractive:**
- Rust already compiles to `wasm32-unknown-unknown`
- WASM is well-documented, stack-based VM
- Small instruction set (~200 opcodes, but most unused for no_std)
- No rustc modifications needed

**Challenges:**
- WASM is stack-based, COR24 is register-based
- WASM has 32-bit integers, COR24 has 24-bit
- Need to implement WASM linear memory on COR24

**Effort:** 200-400 hours

---

## Option 2: Proc-Macro DSL

**Pipeline:**
```
Rust-like DSL (in .rs file)
    ↓ proc-macro expansion
COR24 Assembly (as string)
    ↓ assembled at build time
Binary data (in Rust)
```

**Example:**
```rust
cor24_asm! {
    fn add_numbers(a: r0, b: r1) -> r0 {
        add r0, r1;
    }
}
```

**Pros:**
- IDE support (syntax highlighting, completion)
- Type checking at compile time
- Very easy to implement

**Cons:**
- Not real Rust - just Rust syntax
- No standard library
- Manual register allocation

**Effort:** 50-100 hours

---

## Option 3: MIR → COR24 Assembly

**Pipeline:**
```
Rust Source
    ↓ rustc (stop at MIR)
MIR (Mid-level IR)
    ↓ custom translator
COR24 Assembly
```

**Approach:**
- Use `rustc --emit=mir` or rustc driver API
- Parse MIR text or use rustc as library
- Translate MIR operations to COR24 instructions

**Supported subset:**
- Integer arithmetic (u8, i8, u16, i16)
- Control flow (if, loop, match)
- Function calls
- Basic structs

**Not supported:**
- Generics (must be monomorphized first)
- Traits (static dispatch only)
- Heap allocation

**Effort:** 500-1000 hours

---

## Option 4: Cranelift Backend

**Pipeline:**
```
Rust Source
    ↓ rustc frontend
Cranelift IR
    ↓ COR24 backend (new)
COR24 Assembly
```

**Components needed:**
- ISA definition (~2-3K lines Rust)
- Instruction lowering
- Register allocator integration
- ABI implementation

**Pros:**
- Rust-native (no C++)
- Growing rustc support
- Good optimization passes

**Effort:** 1000-2000 hours

---

## Option 5: LLVM Backend

**Pipeline:**
```
Rust Source
    ↓ rustc frontend
LLVM IR
    ↓ COR24 backend (new)
COR24 Assembly
```

**Components needed:**
- TableGen definitions (~5-10K lines)
- Target machine (~10-20K lines C++)
- Full instruction selection

**Pros:**
- Industry standard
- Best optimizations
- Full Rust support

**Cons:**
- Massive effort
- C++ codebase
- Must track LLVM updates

**Effort:** 2000-5000 hours

---

## Option 6: mrustc → C → COR24

**Pipeline:**
```
Rust Source
    ↓ mrustc
C Source
    ↓ COR24 C compiler
COR24 Assembly
```

**Prerequisites:**
- COR24 C compiler must exist and be accessible
- mrustc configured for target

**Pros:**
- Leverages existing tools
- Proven path

**Cons:**
- Limited Rust features
- Two-stage compilation
- Debugging is hard

**Effort:** 200-500 hours (if C compiler exists)

---

## COR24-Specific Considerations

### Architecture Constraints
- **24-bit words** - Non-standard, requires masking from 32-bit
- **3 GP registers** - Heavy stack spilling needed (r0-r2 only)
- **16MB memory** - 24-bit address space
- **No hardware multiply/divide** - Need software implementations

### Reserved Registers
| Register | Alias | Purpose |
|----------|-------|---------|
| r3 | fp | Frame pointer |
| r4 | sp | Stack pointer |
| r5 | z | Zero (read-only) |
| r6 | iv | Interrupt vector |
| r7 | ir | Interrupt return |

Only r0, r1, r2 are truly general-purpose.

### Calling Convention (Proposed)
- Arguments: r0, r1, r2, then stack
- Return value: r0
- Callee-saved: fp
- Caller-saved: r0, r1, r2

---

## Recommendation

For the **simplest viable path**, we recommend:

1. **Start with Option 2 (Proc-Macro DSL)** for immediate usability
2. **Then implement Option 1 (WASM → COR24)** for real Rust support

See `planned-simplest.md` for detailed implementation plan.
