# tc24r Integration: In-Browser C Compiler for COR24

## Vision

Add an interactive "Tiny C" tab to the COR24 emulator where users can:
1. Write/edit C code in the browser
2. Compile to COR24 assembly (in-browser, no server)
3. Assemble to machine code
4. Run/step in the emulator with full debug

Unlike the existing C and Rust tabs (which show pre-compiled read-only
examples), this tab runs the entire pipeline live in the browser.

## Feasibility: Yes

The cc24 compiler (to be renamed tc24r) is written in Rust with a
highly modular component architecture (~40 crates). Rust compiles to
WASM. The compiler has no OS dependencies (no file I/O needed for
compilation itself — source text in, assembly text out). This makes
WASM compilation feasible.

Key factors:
- Pure Rust, no FFI or system calls in the compilation path
- Input: C source string → Output: COR24 assembly string
- No linker needed (COR24 programs are flat binary at address 0)
- Standard headers can be embedded as string constants
- The cor24-rs assembler already runs in WASM

## Current cc24 Architecture

```
~/github/sw-vibe-coding/cc24/
├── components/
│   ├── frontend/          ← lexer, preprocessor, parser
│   │   ├── cc24-lexer
│   │   ├── cc24-preprocess
│   │   ├── cc24-parse-stream
│   │   └── cc24-parser
│   ├── core/              ← AST, tokens, spans, errors, traits
│   │   ├── cc24-ast
│   │   ├── cc24-token
│   │   ├── cc24-span
│   │   ├── cc24-error
│   │   └── cc24-traits
│   ├── config/            ← target config
│   │   ├── cc24-config
│   │   └── cc24-target
│   ├── dispatch/          ← statement/expression dispatch
│   │   └── cc24-dispatch
│   ├── codegen-state/     ← compiler state management
│   │   └── cc24-codegen-state
│   ├── codegen-emit/      ← assembly emission
│   │   ├── cc24-emit-core
│   │   ├── cc24-emit-load-store
│   │   └── cc24-emit-data
│   ├── codegen-expr/      ← expression code generation
│   │   ├── cc24-expr-literal
│   │   ├── cc24-expr-variable
│   │   ├── cc24-expr-pointer
│   │   ├── cc24-expr-call
│   │   └── cc24-expr-ops
│   ├── codegen-ops/       ← operator code generation
│   │   ├── cc24-ops-arithmetic
│   │   ├── cc24-ops-bitwise
│   │   ├── cc24-ops-compare
│   │   ├── cc24-ops-logical
│   │   ├── cc24-ops-unary
│   │   ├── cc24-ops-incdec
│   │   ├── cc24-ops-divmod
│   │   └── cc24-type-infer
│   ├── codegen-stmt/      ← statement code generation
│   │   ├── cc24-stmt-simple
│   │   └── cc24-stmt-control
│   ├── codegen-structure/ ← function structure
│   │   ├── cc24-struct-prologue
│   │   ├── cc24-struct-locals
│   │   └── cc24-struct-isr
│   ├── backend/           ← codegen orchestration
│   │   ├── cc24-codegen
│   │   └── cc24-codegen-validate
│   ├── macros/            ← proc macros
│   │   ├── cc24-emit-macros
│   │   └── cc24-handler-macros
│   ├── cli/               ← CLI binary
│   │   └── cc24
│   └── testing/           ← test infrastructure
│       ├── cc24-test-compile
│       ├── cc24-test-golden
│       ├── cc24-test-cor24
│       └── cc24-test-as24
├── include/               ← standard headers
├── demos/                 ← demo C programs
└── docs/                  ← documentation
```

## Architecture Plan

### Phase 1: Library Extraction (cc24 repo)

Create a single `cc24-lib` crate that re-exports the public compile API:

```rust
// cc24-lib/src/lib.rs
pub fn compile(source: &str, filename: &str, headers: &[(&str, &str)]) -> Result<String, Vec<CompileError>>;
```

**Input:** C source code string, filename for error messages, virtual
headers as `(name, content)` pairs.

**Output:** COR24 assembly text (compatible with cor24-rs assembler)
or a list of compile errors with line/column info.

This crate depends on the frontend + codegen components but NOT on:
- File I/O (headers provided as strings)
- CLI argument parsing
- Test infrastructure

The CLI binary (`cc24`) becomes a thin wrapper that reads files and
calls `cc24_lib::compile()`.

### Phase 2: WASM Build Verification (cc24 repo)

Verify the library compiles to `wasm32-unknown-unknown`:

```bash
cargo check --target wasm32-unknown-unknown -p cc24-lib
```

Fix any dependencies that don't compile to WASM:
- Replace `std::fs` with string-based API (already planned)
- Replace `std::path` with string filenames
- Ensure no `std::process`, `std::net`, etc.

### Phase 3: Integration (cor24-rs repo)

#### 3a. Add cc24-lib as a dependency

```toml
# cor24-rs/Cargo.toml
[dependencies]
cc24-lib = { path = "../sw-vibe-coding/cc24/components/cc24-lib" }
```

Or publish to a private registry / use git dependency.

#### 3b. WASM Bridge

Add to `wasm.rs`:

```rust
#[wasm_bindgen]
pub fn compile_c(source: &str, filename: &str) -> String {
    let headers = vec![
        ("stdio.h", include_str!("headers/stdio.h")),
        ("stdlib.h", include_str!("headers/stdlib.h")),
    ];
    match cc24_lib::compile(source, filename, &headers) {
        Ok(asm) => format!(r#"{{"ok":true,"asm":"{}"}}"#, asm.replace('"', "'")),
        Err(errors) => {
            let msgs: Vec<String> = errors.iter()
                .map(|e| format!("{}:{}: {}", e.line, e.col, e.message))
                .collect();
            format!(r#"{{"ok":false,"errors":{:?}}}"#, msgs)
        }
    }
}
```

#### 3c. Tiny C Tab UI

New Yew component `TinyCPipeline` (similar to existing pipeline components):

```
┌──────────────────────────────────────────────┐
│ [Tiny C]  tab                                │
├──────────┬──────────┬────────────────────────┤
│ Sidebar  │ Wizard   │ Notebook Cells         │
│          │ Steps    │                        │
│ Tutorial │ Source ✓ │ ┌──────────────────┐   │
│ Examples │ Compile  │ │ C Source (editor) │   │
│ ISA Ref  │ Assemble │ │ editable textarea │   │
│ Help     │          │ └──────────────────┘   │
│          │[Compile] │ ┌──────────────────┐   │
│          │          │ │ COR24 Assembly    │   │
│          │          │ │ (read-only)       │   │
│          │          │ └──────────────────┘   │
│          │          │ ┌──────────────────┐   │
│          │          │ │ Execution/Debug   │   │
│          │          │ │ (DebugPanel)      │   │
│          │          │ └──────────────────┘   │
└──────────┴──────────┴────────────────────────┘
```

Key differences from existing C tab:
- **Source cell is editable** (textarea, not pre block)
- **Compile button runs cc24-lib in WASM** (not just visual step)
- **Compile errors shown inline** with line numbers
- **Examples are editable templates** (user can modify and recompile)

#### 3d. Virtual Header System

Standard C headers (`stdio.h`, `stdlib.h`, `string.h`) embedded as
`include_str!()` constants. The compiler's preprocessor resolves
`#include <stdio.h>` by looking up the header name in the embedded
table instead of the filesystem.

The headers provide:
- `printf` declaration (maps to UART output runtime)
- `malloc`/`free` stubs (bump allocator in SRAM)
- Standard type definitions (`NULL`, `size_t`, etc.)
- COR24-specific: `mmio_read()`, `mmio_write()`, I/O addresses

#### 3e. Runtime Stubs

Like the existing C pipeline, compiled programs need runtime stubs:
- `_printf` → UART output with format string parsing
- `_putchr` → single character to UART (with TX busy poll)
- `_main` wrapper → reset vector + call main()

These are assembly stubs appended to the compiled output before
assembly. They're already defined in the existing C pipeline examples
and can be reused.

### Phase 4: Tab Integration

Add the Tiny C tab to `app.rs`:
- New CPU instance (`tinyc_cpu`)
- New state (editable source, compile output, errors)
- Tab order: Assembler, C (read-only), Rust, Tiny C

The compile flow:
1. User writes C code in editor
2. Clicks "Compile" → calls `compile_c()` via WASM
3. Success: COR24 assembly shown in next cell
4. Clicks "Assemble" → assembles using existing `WasmCpu.assemble()`
5. Clicks "Run"/"Step" → emulator runs as usual

### Phase 5: Example Programs

Port the existing C pipeline examples (fib, sieve) plus new ones
that demonstrate C features the compiler supports:
- Hello World (printf)
- Variables and arithmetic
- If/else, while, for
- Functions and recursion
- Pointers and arrays
- Structs (when supported)
- Interrupt handlers (when supported)

## WASM Size Considerations

The cc24 compiler is ~40 crates. The WASM binary will be larger than
the current emulator-only build. Mitigation:
- Tree shaking removes unused code
- `wasm-opt -Oz` for size optimization
- Lazy loading: compile WASM only loaded when Tiny C tab is opened
- Estimated: 500KB-2MB additional WASM (acceptable)

## Timeline

| Phase | Scope | Effort |
|-------|-------|--------|
| 1. Library extraction | cc24 repo | 1-2 days |
| 2. WASM verification | cc24 repo | 1 day |
| 3a-b. Integration | cor24-rs | 1 day |
| 3c. UI component | cor24-rs | 2-3 days |
| 3d-e. Headers + runtime | both repos | 1-2 days |
| 4. Tab integration | cor24-rs | 1 day |
| 5. Examples | cor24-rs | 1-2 days |

Total: ~8-12 days

## Open Questions

1. **Repo structure**: Should cc24-lib be a git submodule of cor24-rs,
   a path dependency, or published to a registry?
2. **Header completeness**: Which C standard library functions should
   the embedded headers declare?
3. **Error display**: How to show compile errors with source highlighting
   in the editor (red underlines, gutter markers)?
4. **Tab naming**: "Tiny C" vs "C Editor" vs "cc24" vs "tc24r"?
5. **Incremental compilation**: Worth caching AST/parse results for
   re-compiles after small edits?
