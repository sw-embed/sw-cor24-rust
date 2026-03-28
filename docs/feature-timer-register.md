# Feature Request: Timer I/O Register

## Summary

Add a millisecond timer register to the COR24 I/O address space, readable via `lb`/`lw` at a fixed memory-mapped address. Returns milliseconds since emulator start, derived from the configured instruction speed.

## Motivation

COR24 has no hardware clock. Programs that need timing (LED blink, animation, timeouts, debounce) currently rely on calibrated spin loops:

```c
int iters = ms * 50;  // calibrated for --speed 500000
while (iters > 0) iters--;
```

This breaks when:
- The emulator speed changes (different `--speed` setting)
- Running in the web UI where effective IPS varies by browser/device
- Speed is set to 0 (max speed — loop completes instantly)

A timer register lets programs poll real (emulated) time regardless of speed setting.

## Specification

### I/O Address

```
0xFF0200  TIMER_LO   (read-only, byte)  Low byte of ms counter
0xFF0201  TIMER_MID  (read-only, byte)  Mid byte of ms counter
0xFF0202  TIMER_HI   (read-only, byte)  High byte of ms counter
```

Or as a single 24-bit word read:
```
0xFF0200  TIMER      (read-only, 3 bytes)  Milliseconds since start
```

24-bit counter wraps at 16,777,215 ms (~4.6 hours). Sufficient for interactive sessions.

### Computation

The emulator already tracks `total_instructions` and knows the `speed` setting (instructions per second):

```rust
fn timer_ms(&self) -> u32 {
    if self.speed == 0 {
        // Max speed: use wall-clock time
        self.start_time.elapsed().as_millis() as u32
    } else {
        // Configured speed: derive from instruction count
        (self.total_instructions * 1000 / self.speed) as u32
    }
}
```

For `--speed 0`, use wall-clock time so programs still get meaningful timing even at uncapped speed. For configured speeds, derive from instruction count so timing is deterministic and reproducible.

### Implementation

In `CpuState::read_io()`:
```rust
IO_TIMER_LO  => (self.timer_ms() & 0xFF) as u8,
IO_TIMER_MID => ((self.timer_ms() >> 8) & 0xFF) as u8,
IO_TIMER_HI  => ((self.timer_ms() >> 16) & 0xFF) as u8,
```

The timer value should be consistent within a single instruction (cache the ms value per instruction or per batch).

### Web UI

The Yew/WASM frontend already tracks instruction count and speed. The same formula applies. The timer register is computed, not stored — it's a virtual peripheral.

## Usage from tml24c

With the timer register, the Lisp prelude would define:

```lisp
(define IO-TIMER #xFF0200)
(define millis (lambda () (peek IO-TIMER)))

(define delay (lambda (ms)
  (begin
    (define start (millis))
    (define wait (lambda () (if (< (- (millis) start) ms) (wait) nil)))
    (wait))))
```

This replaces the calibrated spin loop with a timer-polling loop that works at any speed.

### Blink demo (timer-based)

```lisp
(define blink (lambda (on-ms off-ms)
  (begin
    (set-leds 1) (delay on-ms)
    (set-leds 0) (delay off-ms)
    (blink on-ms off-ms))))
(blink 1000 1000)
```

Same code, but `delay` now works correctly regardless of emulator speed.

## Alternatives Considered

1. **Calibrated spin loops** — Current approach. Fragile, speed-dependent, doesn't work at speed=0 or in browser.
2. **Interrupt-driven timer** — COR24 only has UART RX interrupt. Adding a timer interrupt would be more complex and require ISR support in the C compiler.
3. **Instruction counter register** — Expose raw instruction count instead of ms. Programs would need to know the speed to convert. Less ergonomic.

## Testing

1. Read timer at start, delay with a spin loop of known length, read timer again. Verify delta matches expected ms.
2. Verify timer advances proportionally to instruction count at configured speed.
3. Verify timer uses wall-clock at speed=0.
4. Verify timer works in WASM/web context.
5. Verify byte reads return correct portions of the 24-bit counter.
