# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## CRITICAL: AgentRail Session Protocol (MUST follow exactly)

This project uses AgentRail. Every session follows this exact sequence:

### 1. START (do this FIRST, before anything else)
```bash
agentrail next
```
Read the output carefully. It tells you your current step, prompt, skill docs, and past trajectories.

### 2. BEGIN (immediately after reading the next output)
```bash
agentrail begin
```

### 3. WORK (do what the step prompt says)
Do NOT ask the user "want me to proceed?" or "shall I start?". The step prompt IS your instruction. Execute it.

### 4. COMMIT (after the work is done)
Commit your code changes with git.

### 5. COMPLETE (LAST thing, after committing)
```bash
agentrail complete --summary "what you accomplished" \
  --reward 1 \
  --actions "tools and approach used" \
  --next-slug "next-step-slug" \
  --next-prompt "what the next step should do" \
  --next-task-type "task-type"
```
If the step failed: `--reward -1 --failure-mode "what went wrong"`
If the saga is finished: add `--done`

### 6. STOP (after complete, DO NOT continue working)
Do NOT make any further code changes after running agentrail complete.
Any changes after complete are untracked and invisible to the next session.
If you see more work to do, it belongs in the NEXT step, not this session.

Do NOT skip any of these steps. The next session depends on your trajectory recording.

## Multi-Agent Coordination (Wiki)

This project coordinates with other agents via a shared wiki. See `docs/agent-cas-wiki.md` for the full API reference and CAS protocol.

- **Wiki server:** `http://localhost:7402` (git backend)
- **Key pages:** [[AgentToAgentRequests]], [[AgentStatus]], [[COR24RS]], [[COR24Toolchain]], [[MVP]]
- **Our role:** cor24-rs is the foundation layer — every COR24 project depends on our assembler and emulator.
- **On session start:** Read [[AgentToAgentRequests]] to check for requests targeting cor24-rs. Update [[AgentStatus]] with our status.

## Related Projects

- `~/github/sw-vibe-coding/pv24a` — P-code VM and p-code assembler (COR24 assembly)
- `~/github/softwarewrighter/pa24r` — P-code assembler (Rust, .spc → .p24)
- `~/github/softwarewrighter/pl24r` — P-code text-level linker (Rust)
- `~/github/softwarewrighter/p24p` — Pascal compiler (C, compiled by tc24r)
- `~/github/softwarewrighter/pr24p` — Pascal runtime library (.spc sources)
- `~/github/softwarewrighter/web-dv24r` — Browser-based p-code VM debugger
- `~/github/sw-vibe-coding/tc24r` — COR24 C compiler (Rust)
- `~/github/sw-vibe-coding/agentrail-domain-coding` — Coding skills domain

## Available Task Types

`rust-project-init`, `rust-clippy-fix`, `pre-commit`

## Build Commands

**CRITICAL: NEVER run `trunk` commands directly.** Always use the shell scripts below. Running bare `trunk serve` or `trunk build` with wrong flags breaks the build (wrong port, missing `--release`, wrong `--public-url`). The scripts encode the correct arguments.

```bash
# Dev server with hot reload (http://localhost:7401/cor24-rs/)
./serve.sh              # incremental build + serve
./serve.sh --clean      # clean build + serve (use after strange build errors)

# Production build (outputs to pages/)
./build.sh              # incremental build
./build.sh --clean      # clean build

# Run tests (OK to run cargo directly for non-build commands)
cargo test

# Check compilation (OK to run cargo directly)
cargo check
cargo check --target wasm32-unknown-unknown   # checks WASM-only code too
cargo clippy --target wasm32-unknown-unknown  # lint check
```

Prerequisites: Rust 1.75+, Trunk (`cargo install trunk`), `rustup target add wasm32-unknown-unknown`.

## Commit Discipline

**Commit early and often.** Each commit should do one thing. Do not accumulate large changesets.

- Commit after each logical change: a bug fix, a new feature, a refactor, an extraction — each is its own commit.
- Small commits enable cherry-picking, rebasing, and bisecting. Large commits make all of these painful.
- If a task involves multiple steps (e.g., extract data to files, then update callers, then add a new feature), commit after each step.
- Commit working code. Run `cargo check --target wasm32-unknown-unknown` before committing WASM changes.
- Deployment commits (`pages/` updates via `./build.sh`) should be separate from code changes when practical.

## Deployment

The `pages/` directory contains pre-built production assets and is committed to git. GitHub Actions deploys from `pages/` on push to `main` — no CI build step, just upload.

**To deploy changes to the live site:**
1. `./build.sh --clean` — always use `--clean` to avoid stale cached WASM artifacts
2. `git add pages/` and commit (separate from code changes)
3. `git push`

**IMPORTANT:** Use `./build.sh --clean`, not `./build.sh`. Incremental builds can serve stale code if `include_str!()` data files changed but Rust source didn't.

## Architecture

This is a browser-based COR24 CPU emulator written in Rust, compiled to WebAssembly via Trunk. The COR24 is a real 24-bit RISC architecture (C-Oriented RISC) designed for embedded systems education.

### Workspace Structure

- **`src/`** — Main application crate (`cor24-emulator`)
- **`components/`** — Reusable Yew UI components library
- **`rust-to-cor24/`** — Standalone CLI tool (not part of workspace). Pipeline: Rust → `rustc --target msp430-none-elf --emit asm` → MSP430 ASM → `msp430-to-cor24 --entry <func>` → COR24 ASM (with `bra <entry>` reset vector prologue at address 0). See `rust-to-cor24/README.md` for full pipeline documentation. Not compiled to WASM — used offline to generate pipeline examples shown in the Web UI's Rust tab.

### Core Modules (src/)

- **`cpu/`** — CPU emulator core
  - `state.rs` — CPU state, memory (64KB subset of 24-bit address space), memory-mapped I/O (LED/switch at `0xFF0000`, UART at `0xFFFF00-02`)
  - `executor.rs` — Instruction execution engine
  - `decode_rom.rs` — Decode ROM extracted from actual FPGA Verilog hardware
  - `encode.rs` — Instruction encoding tables
  - `instruction.rs` — Opcode definitions, variable-length instructions (1/2/4 bytes)
- **`assembler.rs`** — Two-pass assembler producing machine code from COR24 assembly
- **`wasm.rs`** — `WasmCpu` wrapper exposing CPU to JavaScript/Yew via `wasm_bindgen`
- **`app.rs`** — Main Yew `#[function_component(App)]` — all application state and UI logic. This is the largest file; it manages two independent CPU instances (assembler tab and Rust pipeline tab)
- **`challenge.rs`** — Example programs and challenge definitions

### UI Components (components/)

Yew components: `Header`, `Sidebar`, `TabBar`, `ProgramArea`, `RegisterPanel`, `MemoryViewer`, `Modal`, `Collapsible`, `RustPipeline`. The `RustPipeline` component implements a wizard-driven 3-column view showing the Rust→MSP430 ASM→COR24 ASM→Machine Code pipeline with pre-built examples.

### Key Patterns

- **Two CPU instances**: `app.rs` maintains separate `WasmCpu` state for the Assembler tab and Rust Pipeline tab
- **Animated run with stop**: Uses `Rc<Cell<bool>>` for stop flags and `Rc<Cell<u8>>` for switch state to ensure immediate visibility across async closures (Yew state updates are deferred)
- **Hardware-accurate I/O**: Matches COR24-TB test board — single LED (D2) and button (S2) using bit 0 of `IO_LEDSWDAT` (`0xFF0000`). Reference hardware docs are in `references/COR24-TB/`
- **Conditional compilation**: `app.rs` and `wasm.rs` are `#[cfg(target_arch = "wasm32")]` only; `cpu/`, `assembler`, and `challenge` modules compile on native targets for `cargo test`
- **`build.rs`**: Embeds git SHA, build timestamp, and hostname into the binary via env vars

### CSS

Two stylesheet files in `styles/`: `asm-game.css` (component styles) and `layout.css` (page structure). Referenced in `index.html` via Trunk's `data-trunk` attributes.

### Reference Materials

`references/COR24-TB/` contains the actual hardware documentation: Verilog source (including `cor24_io.v` for I/O address decoding), demo C programs (blinky, sieve, etc.), and FPGA project files. The decode ROM in `decode_rom.rs` was extracted from `cor24_cpu.v` using `scripts/extract_decode_rom.py`.
