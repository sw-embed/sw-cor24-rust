# COR24 Compilation Pipelines

Three paths to running code on the COR24 emulator.

## 1. Assembler Tab — Write COR24 assembly directly

```
COR24 assembly (.s)  →  Assembler  →  bytes  →  Emulator
```

Write or edit assembly in the browser editor. Click Assemble, then
Step or Run. The assembler and emulator run entirely in the browser
via WebAssembly. The same assembler is available as `cor24-run` on
the command line.

**Example:**
```
lc  r0, 42
la  r1, 256
sb  r0, 0(r1)
halt:
        bra halt
```

All `.s` examples are as24-compatible — they assemble with both our
Rust assembler and MakerLisp's reference `as24` assembler.

## 2. C Tab — View pre-compiled C examples

```
C source (.c)  →  cc24 compiler  →  as24 assembler  →  COR24 assembly (.cor24.s)
                  (MakerLisp's toolchain, offline)
                                                           ↓
                                              Rust assembler  →  bytes  →  Emulator
```

The C examples were compiled offline using MakerLisp's `cc24` C
compiler and `as24` assembler. The Web UI shows the resulting `.cor24.s`
files. The wizard steps (Source → Compile → Assemble) are for
visualization — the compilation already happened; clicking Compile
just reveals the next panel.

Runtime stubs (`_printf`, `_putchr`) are injected to provide UART
output, since the emulator doesn't have a full C runtime.

## 3. Rust Tab — Rust → MSP430 → COR24 pipeline

```
Rust source (.rs)
    ↓  rustc --target msp430-none-elf --emit asm
MSP430 assembly (.msp430.s)
    ↓  msp430-to-cor24 translator
COR24 assembly (.cor24.s)
    ↓  Rust assembler
Machine code (bytes)
    ↓  Emulator
Execution
```

The Rust pipeline compiles Rust to COR24 via MSP430 as an intermediate.
The MSP430 target is used because Rust supports it natively (via LLVM),
and its 16-bit register model is close enough to COR24's 24-bit model
for mechanical translation.

The translator maps:
- **Registers**: MSP430 r12→r0, r13→r1, r14→r2; others spill to stack
- **Instructions**: add→add, mov→mov, cmp→ceq/clu/cls, call→jal, etc.
- **Addresses**: MSP430 16-bit → COR24 24-bit with I/O remapping
- **Passthrough**: `@cor24:` asm comments emit literal COR24 instructions

Like the C tab, the compilation happens offline. The Web UI shows all
intermediate stages. The wizard steps reveal each stage progressively.

### @cor24 passthrough

For performance-critical code, Rust functions can include literal
COR24 assembly via asm comments:

```rust
core::arch::asm!(
    "; @cor24: lc r1, 1",
    "; @cor24: add r0, r1",
    "; @cor24: jmp (r1)",
);
```

The translator passes these through verbatim. The MSP430 intermediate
is irrelevant for passthrough sections — only the COR24 output matters.

## No Linker

COR24 programs are flat binary — code starts at address 0, no sections,
no relocations, no ELF headers. The assembler resolves labels internally
and produces a contiguous byte array. Loading is just copying bytes into
SRAM. There is no separate link step.

## Memory Layout

```
000000-0FFFFF   SRAM (1 MB) — code at low addresses, data above
FEE000-FEFFFF   EBR (8 KB range, 3 KB populated) — stack, SP init FEEC00
FF0000          LED D2 (write bit 0) / Button S2 (read bit 0)
FF0010          Interrupt enable (bit 0 = UART RX)
FF0100          UART data (read=RX, write=TX)
FF0101          UART status (bit 7=TX busy, bit 1=RX ready)
```
