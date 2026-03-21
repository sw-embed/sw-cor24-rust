# C Tab Implementation Plan

Add a "C" tab to the COR24 emulator web UI showing real C compiler output
from MakerLisp's COR24 C toolchain. Users step through C-compiled assembly
using the same emulator that runs hand-written and Rust-compiled programs.

## Examples

Two C examples, both from `docs/research/asld24/`:

1. **Sieve of Eratosthenes** (`sieve.s`) — Self-contained. Has its own `_putchr`,
   `_printn`, `_putstr`. Computes 1000 iterations, prints prime count + message
   via UART. No external dependencies.

2. **Fibonacci** (`fib.s`) — Recursive fib(33). Calls `_printf` which doesn't
   exist in the .s file. We inject a `_printf` stub in COR24 assembly that
   parses format strings and writes to UART, so users can step through printf
   execution at the instruction level.

## Printf Stub

Injected into the fib example's .s file before assembly. Implements:
- Walk format string byte-by-byte via pointer from stack arg `9(fp)`
- Literal chars → `_putchr` (UART TX with busy polling)
- `%d` → decimal conversion + output (reuse `_printn` logic from sieve)
- `%s` → walk and output string
- Null terminator → return

Uses the same C calling convention as the compiler output:
`push fp; push r2; push r1; mov fp,sp` prologue, args on stack at `9(fp)`,
return in r0, `jmp (r1)` return.

## Architecture

### Data Layer

**`src/c_examples.rs`** — New file, parallel to `rust_examples.rs`.
- `get_c_examples() -> Vec<CExample>` returns example list
- `CExample` struct: `name`, `description`, `c_source`, `cor24_assembly`
  (no intermediate MSP430 step — C compiler outputs COR24 directly)
- Assembly loaded via `include_str!()` from `src/examples/c_pipeline/`

**`src/examples/c_pipeline/`** — New directory with pre-built examples:
- `sieve.c` — C source (for display)
- `sieve.cor24.s` — Assembly from MakerLisp's compiler (self-contained, runs as-is)
- `fib.c` — C source (for display)
- `fib.cor24.s` — Assembly from compiler + injected `_printf` stub

### Component Changes

**`components/rust_pipeline.rs`** → Rename/generalize to support both pipelines.

The Rust pipeline has 4 wizard steps: Source → Compile → Translate → Assemble.
The C pipeline has 3 steps: Source → Compile → Assemble.

Two approaches (prefer option B):

**Option A**: Parameterize `RustPipeline` with a `PipelineMode` enum.
Add `mode: PipelineMode` prop that controls step labels, column count,
and which notebook cells render. Risk: makes the component more complex.

**Option B**: Create a new `CPipeline` component that reuses `DebugPanel`
and `ExamplePicker` but has its own simpler wizard (2 code cells + execution).
The C pipeline is simpler than Rust (no MSP430 intermediate), so a dedicated
component keeps each pipeline clean.

→ **Go with Option B.** The C pipeline component is simple:
- Left sidebar: Examples, ISA Ref, Help buttons
- Middle: 2-step wizard (Source, Assemble)
- Right: Notebook cells showing C source → COR24 ASM, then DebugPanel

**`components/c_pipeline.rs`** — New component.
- Props mirror RustPipeline: cpu_state, execution callbacks, modal callbacks
- Wizard steps: Source (show C source), Assemble (show COR24 ASM + debugger)
- Reuses `DebugPanel`, `ExamplePicker`, `Modal`, `Sidebar` components

### App Integration (`src/app.rs`)

- Add third tab: `Tab { id: "c", label: "C", tooltip: "C → COR24 compilation pipeline" }`
- Add third CPU instance: `c_cpu`, `c_emu_state`, `c_is_running`, `c_is_loaded`
- Add `Rc<Cell>` flags: `c_stop_requested`, `c_shared_switches`, `c_uart_queue`
- Add modal states: `c_examples_open`, `c_tutorial_open`
- Render `CPipeline` component conditionally when `active_tab == "c"`
- Wire up callbacks following same pattern as Rust tab

### File Preparation

The sieve.s and fib.s from `docs/research/asld24/` need minor adaptations:

1. **Entry point**: Add reset vector prologue (`mov fp,sp; la r0,_main; jmp (r0)`)
   and halt loop after main returns, since the emulator starts at address 0.

2. **Data section**: The `.data` and `.byte` directives need to be converted to
   COR24 assembler syntax. The existing assembler supports `.byte` — verify
   `.word`, `.comm`, and `.data` work or convert to equivalent.

3. **Sieve memory**: `_flags` needs 8191 bytes via `.comm`. This maps to BSS
   (zero-initialized). May need to allocate in SRAM and zero on startup.

4. **Stack setup**: Ensure SP starts at a reasonable location (emulator default
   is 0xFEEC00, which is fine).

5. **Printf stub for fib**: Write `_printf` as COR24 assembly, append to fib.s.
   Include `_putchr` (from sieve pattern) since fib.s doesn't have its own.

## Implementation Order

1. Write `_printf` stub in COR24 assembly, test with CLI `--trace`
2. Prepare sieve.cor24.s and fib.cor24.s (adapt from reference .s files)
3. Create `src/c_examples.rs` with `get_c_examples()`
4. Create `components/c_pipeline.rs` (simplified wizard)
5. Add C tab to `src/app.rs` with third CPU instance
6. Test in browser, verify stepping through printf works
7. Commit each step separately

## Assembler Compatibility

The existing COR24 assembler needs to handle directives from the C compiler:
- `.text` — already a no-op (everything is in text segment)
- `.globl` — already handled (ignored)
- `.data` — needs handling (switch to data segment addressing)
- `.byte` — already supported
- `.word` — needs 3-byte word support (COR24 is 24-bit)
- `.comm` — BSS allocation, needs implementation or workaround
- `sb r0,(r2)` — zero-offset store, verify assembler handles `(r2)` without offset

If adding `.data`/`.word`/`.comm` is too complex, we can manually convert
the data sections to `.byte` sequences at known addresses.

## Notes

- The C compiler uses `la r2,-65280` for UART base (=-0xFF00, which is
  0xFFFF0100 in 24-bit). Need to verify this resolves correctly or patch.
- Sieve runs ~500M instructions for 1000 iterations — too slow for web.
  Reduce to 1 iteration for the demo, or provide a "fast mode" note.
- The C calling convention (full frame setup) differs from our Rust translator's
  simplified convention. This is a feature — users can compare both approaches.
