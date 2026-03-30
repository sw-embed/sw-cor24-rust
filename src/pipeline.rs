//! Complete Rust→COR24 Pipeline
//!
//! This module provides an end-to-end pipeline:
//! 1. Compile Rust to WASM (using rustc)
//! 2. Translate WASM to COR24 assembly (wasm2cor24)
//! 3. Assemble COR24 assembly to machine code
//! 4. Run on emulator with LED output

use cor24_assembler::Assembler;
use cor24_emulator::emulator::EmulatorCore;
use std::fs;
use std::process::Command;

/// Print LED state
pub fn print_leds(leds: u8) {
    print!("LEDs: ");
    for i in (0..8).rev() {
        if (leds >> i) & 1 == 1 {
            print!("\x1b[91m●\x1b[0m");
        } else {
            print!("○");
        }
    }
    println!("  (0x{:02X})", leds);
}

/// Full pipeline: compile Rust → WASM → assembly → machine code → run
pub fn run_pipeline(rust_dir: &str, verbose: bool) -> Result<(), String> {
    println!("=== Rust → COR24 Pipeline ===\n");

    // Step 1: Compile Rust to WASM
    println!("Step 1: Compiling Rust to WASM...");
    let output = Command::new("cargo")
        .args(["build", "--target", "wasm32-unknown-unknown", "--release"])
        .current_dir(rust_dir)
        .output()
        .map_err(|e| format!("Failed to run cargo: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Cargo build failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Find the WASM file
    let wasm_path = format!(
        "{}/target/wasm32-unknown-unknown/release/{}.wasm",
        rust_dir,
        std::path::Path::new(rust_dir)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
    );

    if !std::path::Path::new(&wasm_path).exists() {
        return Err(format!("WASM file not found: {}", wasm_path));
    }

    let wasm_size = fs::metadata(&wasm_path).map(|m| m.len()).unwrap_or(0);
    println!("   Generated: {} ({} bytes)\n", wasm_path, wasm_size);

    // Step 2: Translate WASM to COR24 assembly
    println!("Step 2: Translating WASM to COR24 assembly...");
    let wasm_bytes = fs::read(&wasm_path).map_err(|e| format!("Failed to read WASM: {}", e))?;
    let asm_source =
        crate::translate_wasm(&wasm_bytes).map_err(|e| format!("Translation failed: {}", e))?;

    let asm_path = format!("{}/output.s", rust_dir);
    fs::write(&asm_path, &asm_source).map_err(|e| format!("Failed to write assembly: {}", e))?;
    println!("   Generated: {} ({} bytes)\n", asm_path, asm_source.len());

    if verbose {
        println!("--- Assembly ---");
        println!("{}", asm_source);
        println!("----------------\n");
    }

    // Step 3: Assemble to machine code
    println!("Step 3: Assembling to machine code...");
    let mut assembler = Assembler::new();
    let result = assembler.assemble(&asm_source);
    if !result.errors.is_empty() {
        return Err(result.errors.join("\n"));
    }

    let byte_count: usize = result.lines.iter().map(|l| l.bytes.len()).sum();
    println!("   Generated: {} bytes of machine code\n", byte_count);

    if verbose {
        print!("   Bytes: ");
        let mut i = 0;
        for line in &result.lines {
            for b in &line.bytes {
                if i > 0 && i % 16 == 0 {
                    print!("\n          ");
                }
                print!("{:02X} ", b);
                i += 1;
            }
        }
        println!("\n");
    }

    // Step 4: Run on emulator
    println!("Step 4: Running on COR24 emulator...\n");
    let mut emu = EmulatorCore::new();
    for line in &result.lines {
        for (i, &b) in line.bytes.iter().enumerate() {
            emu.write_byte(line.address + i as u32, b);
        }
    }

    emu.resume();
    let batch = emu.run_batch(10000);
    println!("Executed {} steps\n", batch.instructions_run);

    // Show LED output
    let led = emu.get_led();
    if led != 0 {
        println!("LED state:");
        print_leds(led);
    }

    // Show UART output
    let uart = emu.get_uart_output();
    if !uart.is_empty() {
        println!("UART output: {}", uart);
    }

    println!("\n=== Pipeline Complete ===");
    Ok(())
}
