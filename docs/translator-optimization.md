# Translator Optimization: Closing the Gap with cc24

## Problem

The Rust→MSP430→COR24 pipeline produces significantly more instructions than
MakerLisp's C compiler (cc24) for equivalent algorithms.

### Iterative Fibonacci Comparison

**Hand-written assembler** (9 instructions in loop):
```
loop:   push r0 / push r2 / ... / add r1,r0 / sub r2,r0 / brf loop
```

**C compiler (cc24)** — `_fib` recursive, ~30 instructions total:
```
_fib:   push fp / push r2 / push r1 / mov fp,sp / ...
        cls r2,r0 / brf L17 / lc r0,1 / bra L16 / ...
```
Clean, idiomatic COR24. Uses stack frames, 3 GP registers efficiently.

**Rust translator** — `fibonacci_iter` iterative, ~35 instructions:
```
fibonacci_iter:
    ceq r0, z / brt .LBB2_3
    push r0 / lc r0,1 / sw r0,27(fp) / pop r0    ; spill "a=1"
    lc r2, 1
.LBB2_2:
    push r0 / lw r0,27(fp) / sw r0,21(fp) / pop r0  ; spill shuffle
    push r0 / lw r0,21(fp) / add r0,r2 / sw r0,21(fp) / pop r0
    add r0, -1 / ceq r0, z
    sw r2,24(fp) / sw r2,27(fp) / lw r2,21(fp)   ; more spills
    brf .LBB2_2
```
Heavy spill traffic: 14 sw/lw spill operations in the loop body.

## Root Cause

MSP430 `fibonacci_iter` uses 5 registers (r11-r15):
```msp430
.LBB2_2:
    mov  r15, r11      ; temp = a
    add  r14, r11      ; temp = a + b
    add  #-1, r12      ; i--
    tst  r12           ; i == 0?
    mov  r14, r13      ; (result = b)
    mov  r14, r15      ; a = b
    mov  r11, r14      ; b = temp
    jne  .LBB2_2
```

Our translator maps: r12→r0, r13→spill, r14→spill, r15→spill, r11→spill.
Only 1 of 5 MSP430 registers maps to a COR24 GP register. The other 4
generate load/store spill traffic on every use.

## Solutions

### Solution 1: @cor24 Passthrough for Hot Paths (Recommended)

Use `asm!()` with `@cor24:` comments in the Rust source to write the inner
loop directly in COR24 assembly. The surrounding Rust code handles setup,
calling convention, and non-performance-critical logic.

```rust
pub fn fibonacci_iter(n: u16) -> u16 {
    unsafe {
        core::arch::asm!(
            // r0 = n (input), returns result in r0
            "; @cor24: lc r1, 0",          // a = 0
            "; @cor24: lc r2, 1",          // b = 1
            "; @cor24: .fib_loop:",
            "; @cor24: push r1",           // save a
            "; @cor24: mov r1, r2",        // a = b
            "; @cor24: pop r2",
            "; @cor24: add r2, r1",        // b = old_a + b
            "; @cor24: add r0, -1",        // n--
            "; @cor24: ceq r0, z",
            "; @cor24: brf .fib_loop",
            "; @cor24: mov r0, r1",        // return a
        );
    }
    // Return value is in r0 (mapped from r12 by convention)
    unreachable!()
}
```

This produces clean COR24 output: ~10 instructions, no spills, comparable
to hand-written assembly. The MSP430 intermediate is irrelevant — the
translator passes @cor24 lines through verbatim.

**Advantages:**
- No translator changes needed
- Precise control over register allocation
- COR24 output is clean and readable
- MSP430 intermediate can be messy — doesn't matter

**Disadvantages:**
- Rust source is less readable (inline asm)
- Must manually manage the COR24 calling convention
- Not portable (COR24-specific)

### Solution 2: Translator Peephole Optimizer

Add optimization passes to the msp430-to-cor24 translator:

1. **Dead store elimination**: Remove `sw rX, N(fp)` followed by `lw rX, N(fp)`
   with no intervening use of that spill slot
2. **Redundant spill removal**: If a value is stored to a spill slot and
   immediately loaded back, eliminate both
3. **Register coalescing**: When a spill slot is only used to transfer between
   two registers, eliminate the spill and use `mov` directly
4. **Live range analysis**: Track which MSP430 registers are actually live at
   each point and avoid spilling dead values

**Advantages:**
- Automatic — no source changes needed
- Improves all translated code
- Still a mechanical translation

**Disadvantages:**
- Complex to implement correctly
- May not achieve hand-written quality
- Risk of introducing bugs in optimization passes

### Solution 3: Hybrid — Both

Use @cor24 passthrough for performance-critical inner loops (fibonacci,
UART polling, etc.) and rely on the translator for glue code (function
prologues, call setup, etc.). Add simple peephole optimizations for the
common cases.

## Recommendation

**Start with Solution 1** (@cor24 passthrough) for the fibonacci demos.
It's zero risk, produces ideal output, and demonstrates the passthrough
mechanism. The Rust source clearly documents the COR24 intent.

**Later, pursue Solution 2** (peephole optimizer) to improve all translated
code. Start with dead store elimination which handles the most common
pattern (push r0 / sw ... / pop r0 around every spill access).

## Comparison: What "Good" Looks Like

Iterative fib(10) inner loop — instruction count per iteration:

| Implementation | Instructions/iteration | Spill ops |
|---|---|---|
| Hand-written asm | ~8 | 0 |
| C compiler (cc24) | ~10 | 0 (recursive, uses stack frames) |
| Rust + @cor24 | ~8 | 0 |
| Rust translator (current) | ~20 | 14 sw/lw per iteration |
