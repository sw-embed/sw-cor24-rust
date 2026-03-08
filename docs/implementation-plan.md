# COR24 Emulator: Implementation Plan

## Goal

Build a correct CLI-first emulator and GDB-like debugger for the MakerLisp COR24
processor. Use the reference assembler (`as24`) to produce object code that our Rust
emulator loads and runs. Validate against known-good listings. Fix all specification
errors. Postpone Rust-to-COR24 compilation and Web UI updates until the CLI is solid.

---

## 1. Bugs and Misconceptions to Fix

### 1.1 CRITICAL — Wrong UART addresses

| Item | Current (wrong) | Correct |
|------|-----------------|---------|
| UART data | `0xFFFF00` | `0xFF0100` |
| UART status | `0xFFFF01` | `0xFF0101` |
| UART baud | `0xFFFF02` | (not a real register — remove) |

**Files:** `src/cpu/state.rs:19-23`

The UART is at `0xFF0100` (data) and `0xFF0101` (status). There is no baud register
in the I/O map — baud rate is fixed in hardware at 921600.

### 1.2 CRITICAL — Wrong UART status register bits

| Bit | Current (wrong) | Correct |
|-----|-----------------|---------|
| bit 0 | RX data ready | RX data ready ✓ |
| bit 1 | TX complete | CTS active |
| bit 2 | (unused) | RX overflow |
| bit 7 | (unused) | TX busy |

**Files:** `src/cpu/state.rs:158-166`

The status read logic uses made-up bit positions. Per `cor24_io.v`:
- bit 0: RX data ready
- bit 1: CTS active (always 1 in emulation)
- bit 2: RX overflow
- bit 7: TX busy (0 = ready)

For `sieve.s` to work, `_putchr` polls:
1. `lb r0,1(r2)` reads status at `r2+1 = 0xFF0101`
2. `lc r1,2; and r0,r1` — tests bit 1 (CTS)
3. `clu z,r0; brf` — loops until CTS is set (bit 1 = 1)
4. Then polls `cls r0,z; brt` — loops while status > 0 (TX busy, bit 7)

### 1.3 CRITICAL — Memory too small (64KB vs 16MB)

| Item | Current | Correct |
|------|---------|---------|
| `MEMORY_SIZE` | 65536 (64KB) | Full 24-bit address space regions |
| Memory model | Flat 64KB with modulo wrapping | Region-based: SRAM + EBR + I/O |

**Files:** `src/cpu/state.rs:7`, `rust-to-cor24/src/run.rs:45`

The `% MEMORY_SIZE` wrapping makes all addresses alias into 64KB. This is wrong — a
write to `0xFEEC00` (EBR/stack) must not alias to `0x00EC00` (SRAM). We need a
region-based memory model.

### 1.4 CRITICAL — Wrong initial SP

| Item | Current | Correct |
|------|---------|---------|
| `INITIAL_SP` | `0x00FC00` | `0xFEEC00` |

**Files:** `src/cpu/state.rs:13`

The stack lives at the top of embedded block RAM, not in low SRAM.

### 1.5 MODERATE — Interrupt enable register missing

| Address | Purpose |
|---------|---------|
| `0xFF0010` | Interrupt enable (bit 0 = UART RX interrupt enable) |

Not mapped at all. Needed for `uartcint.c` later.

### 1.6 MODERATE — LED/switch description wrong

The hardware has:
- D1: power LED (not controllable)
- D2: user LED (active low, defaults to ON)
- S1: reset switch (not readable)
- S2: momentary pushbutton, normally HIGH, LOW when pressed

`IoState` models "8 LEDs" — it's actually 1 LED (bit 0) and 1 button (bit 0).

### 1.7 MINOR — "halt" is not a real instruction

There is no halt instruction. The convention is `jmp` to self (infinite loop), or
`la r7,0; jmp (r7)` to jump to address 0. The current "halt" pseudo-instruction
(`0xC7 00 00 00` = `la r7,0`) followed by a special check in the executor is a
reasonable emulation-only convention, but should be documented as such.

### 1.8 MINOR — NOP encoding

NOP is not a distinct opcode. Per the designer: `NOP = jmp -2` (encoded as a 2-byte
branch-always to self: the instruction occupies 2 bytes, and the PC-relative
displacement of -2 makes it loop to itself, but in practice the pipeline just
advances). The specific encoding depends on whether it's `bra -4` accounting for PC+2.

### 1.9 INFO — `run.rs` has a separate incomplete CPU

`rust-to-cor24/src/run.rs` contains a second, stripped-down CPU implementation with
hard-coded opcode byte matches. It has wrong SP init (`0xFE00`), incomplete instruction
coverage, and wrong `brt` encoding (`0x12` — should be `0x15` per decode ROM). This
needs to be replaced with the library CPU.

---

## 2. Memory Model Redesign

### 2.1 COR24 Memory Map (from hardware)

```
Address Range         Size    Description
--------------------- ------- --------------------------------
0x000000 - 0x0FFFFF   1 MB    External SRAM (on-board)
0x100000 - 0xFDFFFF   ~15 MB  External SRAM (addressable, unpopulated)
0xFE0000 - 0xFEFFFF   64 KB   EBR window (only 3 KB populated: FE0000-FE0BFF)
0xFF0000 - 0xFFFFFF   64 KB   I/O space
```

Reset vector: `0xFEE000` (boot ROM in EBR)
Initial SP: `0xFEEC00`

### 2.2 Emulation Memory Layout

For the emulator, we don't need 16MB. Allocate:

```rust
struct Memory {
    sram: Vec<u8>,     // 1 MB (0x000000 - 0x0FFFFF)
    ebr: Vec<u8>,      // 8 KB (0xFE0000 - 0xFE1FFF), only 3KB populated
    // I/O handled separately
}
```

Address decoding:
- `0xFF____` → I/O dispatch
- `0xFE____` → EBR (offset = addr - 0xFE0000, bounds-checked)
- `< 0x100000` → SRAM
- Otherwise → return 0 (unmapped)

### 2.3 Program loading

For programs assembled with `as24`, code starts at address 0 (SRAM). The monitor
normally loads code at 0 and jumps to it. So `RESET_ADDRESS = 0x000000` is fine for
user programs (the real reset vector at `0xFEE000` is for the boot ROM/monitor, which
we're not emulating).

---

## 3. Implementation Phases

### Phase 1: Fix the core library CPU (the `src/cpu/` crate)

1. Fix UART addresses: `0xFF0100`, `0xFF0101`
2. Fix UART status bits: bit 7=TX busy, bit 1=CTS, bit 0=RX ready
3. Implement region-based memory (SRAM + EBR + I/O)
4. Fix INITIAL_SP to `0xFEEC00`
5. Fix LED/switch to single-bit model
6. Remove fake UART baud register
7. Add interrupt enable register at `0xFF0010`
8. Update all existing tests

### Phase 2: Load format — parse `as24` output

The assembler (`as24`) without `-l` produces monitor load/go commands:
```
LAAAAAAHH HH HH ...    (load hex bytes at address)
GAAAAAA                 (go — start execution at address)
```

Through `longlgo`, these become consolidated long lines.

We need a loader that:
1. Parses `L` lines: extract 6-digit hex address, then pairs of hex bytes
2. Parses `G` lines: extract start address
3. Loads bytes into emulator memory
4. Sets PC to the `G` address

### Phase 3: CLI debugger (`cor24-dbg`)

A new binary crate: `src/bin/cor24-dbg.rs` (or a separate crate). GDB-like interface:

```
cor24-dbg <file.lgo>       # load a .lgo file
cor24-dbg --asm <file.s>   # assemble with as24, load result
```

Commands:
- `r` / `run` — run until halt/breakpoint
- `s` / `step [N]` — single step (N instructions)
- `n` / `next` — step over (step but skip `jal` calls)
- `c` / `continue` — continue from breakpoint
- `b <addr>` / `break <addr>` — set breakpoint
- `d <N>` / `delete <N>` — delete breakpoint
- `i r` / `info registers` — show all registers
- `i b` / `info breakpoints` — list breakpoints
- `x/<N> <addr>` — examine N bytes at address
- `p <expr>` — print register or memory
- `disas [addr] [len]` — disassemble
- `load <file>` — load .lgo file
- `reset` — reset CPU
- `q` / `quit` — exit

I/O display:
- LED state shown after each step/run
- UART output printed to terminal in real time
- UART input from terminal stdin (raw mode)

### Phase 4: Validate with reference assembler output

Use `as24 -l` listings to create golden test vectors:

1. **sieve.s** — self-contained, links successfully, has UART output
2. **fib.s** — assemble-only (needs printf stub), but validates instruction encoding

For sieve.s:
```bash
cd docs/research/asld24
./cor24-as < sieve.s | ./longlgo > sieve.lgo
```
Then: `cor24-dbg sieve.lgo` should run and produce `1899 primes.` on the UART output.

### Phase 5: Web UI updates (deferred)

Once the CLI is correct, port fixes to `app.rs` and WASM bindings. Not in scope now.

---

## 4. TDD Test Plan — Red/Green Tests

Write these tests FIRST (they will fail = RED), then fix code until they pass (GREEN).

### 4.1 Memory Model Tests (`src/cpu/state.rs`)

```rust
#[test]
fn test_initial_sp_is_feec00() {
    let cpu = CpuState::new();
    assert_eq!(cpu.get_reg(4), 0xFEEC00, "SP must init to 0xFEEC00");
}

#[test]
fn test_sram_region() {
    let mut cpu = CpuState::new();
    cpu.write_byte(0x000100, 0x42);
    assert_eq!(cpu.read_byte(0x000100), 0x42);
    // Must NOT alias to other regions
    assert_eq!(cpu.read_byte(0xFE0100), 0x00, "SRAM must not alias into EBR");
}

#[test]
fn test_ebr_region() {
    let mut cpu = CpuState::new();
    cpu.write_byte(0xFE0000, 0xAA);
    assert_eq!(cpu.read_byte(0xFE0000), 0xAA);
    assert_eq!(cpu.read_byte(0x000000), 0x00, "EBR must not alias into SRAM");
}

#[test]
fn test_ebr_stack_area() {
    let mut cpu = CpuState::new();
    // Write a word to the stack init area
    cpu.write_word(0xFEEC00 - 3, 0x123456);
    assert_eq!(cpu.read_word(0xFEEC00 - 3), 0x123456);
}

#[test]
fn test_io_region_not_in_sram() {
    let mut cpu = CpuState::new();
    cpu.write_byte(0xFF0000, 0x01); // LED write
    // Must not appear in SRAM
    assert_eq!(cpu.read_byte(0x000000), 0x00);
}

#[test]
fn test_unmapped_memory_returns_zero() {
    let cpu = CpuState::new();
    assert_eq!(cpu.read_byte(0x500000), 0x00, "Unmapped region returns 0");
}
```

### 4.2 UART Address Tests (`src/cpu/state.rs`)

```rust
#[test]
fn test_uart_data_at_ff0100() {
    let mut cpu = CpuState::new();
    cpu.write_byte(0xFF0100, b'H');
    assert_eq!(cpu.io.uart_tx, b'H');
    assert!(cpu.io.uart_output.contains('H'));
}

#[test]
fn test_uart_status_at_ff0101() {
    let cpu = CpuState::new();
    let status = cpu.read_byte(0xFF0101);
    // TX not busy (bit 7 = 0), CTS active (bit 1 = 1), no RX data (bit 0 = 0)
    assert_eq!(status & 0x82, 0x02, "CTS=1, TX not busy");
}

#[test]
fn test_uart_status_rx_ready() {
    let mut cpu = CpuState::new();
    cpu.io.uart_rx = b'A';
    cpu.io.uart_rx_ready = true;
    let status = cpu.read_byte(0xFF0101);
    assert_eq!(status & 0x01, 0x01, "RX ready bit 0 set");
}

#[test]
fn test_uart_old_address_not_mapped() {
    let mut cpu = CpuState::new();
    cpu.write_byte(0xFFFF00, b'X');
    assert_ne!(cpu.io.uart_tx, b'X', "Old UART address must not work");
}

#[test]
fn test_uart_read_clears_rx_ready() {
    let mut cpu = CpuState::new();
    cpu.io.uart_rx = b'Z';
    cpu.io.uart_rx_ready = true;
    let _data = cpu.read_byte(0xFF0100);
    assert!(!cpu.io.uart_rx_ready, "Reading UART data auto-clears RX ready");
}
```

### 4.3 LED/Switch Tests (`src/cpu/state.rs`)

```rust
#[test]
fn test_led_write_bit0() {
    let mut cpu = CpuState::new();
    cpu.write_byte(0xFF0000, 0x01);
    assert_eq!(cpu.io.leds, 0x01);
}

#[test]
fn test_switch_read_bit0() {
    let mut cpu = CpuState::new();
    cpu.io.switches = 0x01; // button pressed = LOW, but we set high for test
    assert_eq!(cpu.read_byte(0xFF0000), 0x01);
}
```

### 4.4 Instruction Encoding Tests (validate against `as24 -l`)

```rust
/// Validate our decode matches as24 output for sieve.s
#[test]
fn test_push_fp_encodes_0x80() {
    // as24 listing: 000000 80  push fp
    let rom = DecodeRom::new();
    let decoded = rom.decode(0x80);
    let opcode = (decoded >> 6) & 0x1F;
    let ra = (decoded >> 3) & 0x07;
    assert_eq!(opcode, 0x15, "push opcode"); // push = 0x15
    assert_eq!(ra, 3, "push fp → ra=3");
}

#[test]
fn test_la_r2_negative_encodes_correctly() {
    // as24 listing: 000004 2B 00 01 FF  la r2,-65280
    // -65280 decimal = 0xFF0100 as 24-bit
    let rom = DecodeRom::new();
    let decoded = rom.decode(0x2B);
    let opcode = (decoded >> 6) & 0x1F;
    let ra = (decoded >> 3) & 0x07;
    assert_eq!(opcode, 0x0B, "la opcode");
    assert_eq!(ra, 2, "la r2 → ra=2");
}

#[test]
fn test_lb_r0_offset_encodes_0x2e() {
    // as24 listing: 000008 2E 01  lb r0,1(r2)
    let rom = DecodeRom::new();
    let decoded = rom.decode(0x2E);
    let opcode = (decoded >> 6) & 0x1F;
    assert_eq!(opcode, 0x0C, "lb opcode = 0x0C");
}
```

### 4.5 Execution Tests (run small programs)

```rust
#[test]
fn test_push_pop_round_trip_at_feec00() {
    let mut cpu = CpuState::new();
    // SP should be 0xFEEC00
    assert_eq!(cpu.get_reg(4), 0xFEEC00);

    // push r0 (value 0x123456)
    cpu.set_reg(0, 0x123456);
    // Manually execute: sp -= 3, store word
    let sp = cpu.get_reg(4) - 3;
    cpu.set_reg(4, sp);
    cpu.write_word(sp, 0x123456);

    assert_eq!(cpu.get_reg(4), 0xFEEBFD);
    assert_eq!(cpu.read_word(0xFEEBFD), 0x123456);
}

#[test]
fn test_sieve_putchr_uart_poll() {
    // Simulate what _putchr does:
    // la r2, -65280  → r2 = 0xFF0100
    // lb r0,1(r2)    → read status at 0xFF0101
    // lc r1,2; and r0,r1 → test bit 1 (CTS)
    let mut cpu = CpuState::new();
    cpu.set_reg(2, 0xFF0100);

    // Read UART status
    let status = cpu.read_byte(0xFF0101);
    // CTS should be active (bit 1 = 1) in emulation
    assert!(status & 0x02 != 0, "CTS must be active for putchr to proceed");
}
```

### 4.6 LGO Loader Tests

```rust
#[test]
fn test_parse_lgo_load_line() {
    // L000000807F7E652B0001FF...
    let line = "L00000080";
    let (addr, bytes) = parse_lgo_line(line).unwrap();
    assert_eq!(addr, 0x000000);
    assert_eq!(bytes, vec![0x80]);
}

#[test]
fn test_parse_lgo_go_line() {
    let line = "G000093";
    let addr = parse_lgo_go(line).unwrap();
    assert_eq!(addr, 0x000093);
}

#[test]
fn test_load_sieve_lgo() {
    let lgo = std::fs::read_to_string("docs/research/asld24/sieve.lgo")
        .expect("sieve.lgo must exist — run: cd docs/research/asld24 && ./cor24-as < sieve.s | ./longlgo > sieve.lgo");
    let mut cpu = CpuState::new();
    let start_addr = load_lgo(&lgo, &mut cpu).unwrap();
    // First instruction should be push fp = 0x80
    assert_eq!(cpu.read_byte(0x000000), 0x80);
    // Start address from G line
    assert_eq!(start_addr, 0x000093); // _main entry point
}
```

### 4.7 Integration Test — run sieve.s to completion

```rust
#[test]
fn test_sieve_produces_correct_output() {
    // Assemble sieve.s with as24, load, run, check UART output
    let lgo = std::fs::read_to_string("docs/research/asld24/sieve.lgo")
        .expect("generate with: ./cor24-as < sieve.s | ./longlgo > sieve.lgo");
    let mut cpu = CpuState::new();
    let start = load_lgo(&lgo, &mut cpu).unwrap();
    cpu.pc = start;

    // Run up to 100M instructions (sieve with 1000 iterations takes a while)
    for _ in 0..100_000_000 {
        if cpu.halted { break; }
        Executor::step(&mut cpu);
    }

    // sieve.c prints "1000 iterations\n1899 primes.\n"
    assert!(cpu.io.uart_output.contains("1899 primes"),
        "Expected '1899 primes' in UART output, got: '{}'", cpu.io.uart_output);
}
```

---

## 5. TODO Checklist

### Phase 1: Core fixes (do first, TDD)

- [ ] Write failing tests for SP = 0xFEEC00
- [ ] Write failing tests for UART at 0xFF0100/0xFF0101
- [ ] Write failing tests for UART status bits (bit 7=TX busy, bit 1=CTS)
- [ ] Write failing tests for region-based memory (no aliasing)
- [ ] Write failing test for UART read auto-clears RX ready
- [ ] Implement region-based memory (SRAM 1MB + EBR 8KB + I/O)
- [ ] Fix `INITIAL_SP` to `0xFEEC00`
- [ ] Fix `IO_UARTDATA` to `0xFF0100`
- [ ] Fix `IO_UARTSTAT` to `0xFF0101`
- [ ] Remove `IO_UARTBAUD`
- [ ] Fix UART status bit layout
- [ ] Implement UART read auto-acknowledge (reading data clears RX ready)
- [ ] Add `IO_INTENABLE` at `0xFF0010`
- [ ] Fix LED to 1-bit model (bit 0 only)
- [ ] Update all existing tests to match new addresses/SP
- [ ] Run `cargo test` — all green

### Phase 2: LGO loader

- [ ] Write failing test for LGO line parsing
- [ ] Write failing test for LGO file loading
- [ ] Implement `parse_lgo_line()` and `load_lgo()`
- [ ] Generate `sieve.lgo` from reference assembler
- [ ] Write failing test: load sieve.lgo, check first byte = 0x80
- [ ] Run `cargo test` — all green

### Phase 3: CLI debugger

- [ ] Create `src/bin/cor24-dbg.rs` (or new crate)
- [ ] Implement REPL loop with readline
- [ ] `load <file.lgo>` command
- [ ] `step` / `s` command with register dump
- [ ] `run` / `r` command
- [ ] `break` / `b <addr>` command
- [ ] `info registers` / `i r` command
- [ ] `examine` / `x/<N> <addr>` command
- [ ] `disas` command (disassemble from address)
- [ ] UART output to terminal
- [ ] UART input from terminal (raw mode)
- [ ] LED state display

### Phase 4: Validation

- [ ] Run `sieve.lgo` to completion → "1899 primes."
- [ ] Step through `fib.s` (needs printf stub or just test encoding)
- [ ] Create minimal test programs:
  - [ ] `led_on.s` — write 1 to LED, halt
  - [ ] `hello_uart.s` — write "Hello\n" to UART
  - [ ] `echo.s` — read UART char, write it back
- [ ] Assemble each with `as24`, load with debugger, verify behavior
- [ ] Cross-check all instruction encodings against `as24 -l` listings

### Phase 5: Web UI (deferred)

- [ ] Port memory model fixes to `app.rs`
- [ ] Port UART address fixes
- [ ] Port LED/switch fixes
- [ ] Update examples and challenges
- [ ] Test in browser

---

## 6. File Organization

```
cor24-rs/
├── src/
│   ├── cpu/
│   │   ├── state.rs       ← fix memory model, UART, SP
│   │   ├── executor.rs    ← update for new I/O addresses
│   │   ├── instruction.rs
│   │   ├── decode_rom.rs
│   │   └── encode.rs
│   ├── loader.rs          ← NEW: parse LGO/monitor format
│   ├── debugger.rs        ← NEW: GDB-like debugger engine
│   ├── disasm.rs          ← NEW: disassembler (reuse decode ROM)
│   ├── lib.rs
│   ├── app.rs             ← defer updates
│   └── ...
├── src/bin/
│   └── cor24-dbg.rs       ← NEW: CLI debugger binary
├── tests/
│   ├── integration.rs     ← NEW: run sieve.lgo, check output
│   └── golden/            ← NEW: as24 listing files as test vectors
├── docs/
│   ├── as24-notes.md
│   └── implementation-plan.md  ← this file
└── ...
```

---

## 7. Reference: Correct I/O Behavior for sieve.s

The `_putchr` function in `sieve.s` (the most important I/O test):

```asm
_putchr:
    push    fp
    push    r2
    push    r1
    mov     fp,sp
    la      r2,-65280       ; r2 = 0xFF0100 (UART base)
L14:                         ; poll CTS
    lb      r0,1(r2)        ; read status at 0xFF0101
    lc      r1,2            ; mask = 0x02
    and     r0,r1           ; test bit 1 (CTS)
    clu     z,r0            ; compare: 0 < (status & 2)?
    brf     L14             ; loop if CTS not active
L16:                         ; poll TX busy
    lb      r0,1(r2)        ; read status again
    cls     r0,z            ; compare: status < 0? (tests sign bit = bit 7 = TX busy)
    brt     L16             ; loop while TX busy (bit 7 set → negative signed)
    lb      r0,9(fp)        ; load character argument
    sb      r0,(r2)         ; write to UART data at 0xFF0100
    mov     sp,fp
    pop     r1
    pop     r2
    pop     fp
    jmp     (r1)            ; return
```

This means our emulator MUST:
1. Map UART status at `0xFF0101`
2. Return bit 1 (CTS) = 1 (always ready in emulation)
3. Return bit 7 (TX busy) = 0 (instant TX in emulation)
4. Map UART data write at `0xFF0100`
5. Handle `lb` (load byte with sign extension) correctly — `cls r0,z` tests if the
   sign-extended status byte is negative, which happens when bit 7 is set
