//! msp430-to-cor24 CLI
//!
//! Translates MSP430 assembly (from rustc) to COR24 assembly.
//!
//! Usage:
//!   msp430-to-cor24 <input.s> [-o output.s]
//!   msp430-to-cor24 --test           # translate built-in test case
//!   msp430-to-cor24 --compile <dir>  # compile Rust project, translate, assemble

use anyhow::Result;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // Parse named flags from anywhere in args
    let mut entry_point: Option<String> = None;
    let mut output_path: Option<String> = None;
    let mut positional: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--entry" | "-e" => {
                i += 1;
                entry_point = args.get(i).cloned();
            }
            "-o" => {
                i += 1;
                output_path = args.get(i).cloned();
            }
            _ => positional.push(args[i].clone()),
        }
        i += 1;
    }

    let cmd = positional.first().map(|s| s.as_str()).unwrap_or("");

    match cmd {
        "--test" => run_test(),
        "--compile" => {
            let dir = positional.get(1).map(|s| s.as_str()).unwrap_or(".");
            compile_and_translate(dir, entry_point.as_deref().unwrap_or("start"))
        }
        "" => {
            eprintln!("Usage: msp430-to-cor24 <input.s> [-o output.s] [--entry <func>]");
            eprintln!("       msp430-to-cor24 --test");
            eprintln!("       msp430-to-cor24 --compile <rust-project-dir> [--entry <func>]");
            std::process::exit(1);
        }
        input_path => {
            let msp430_asm = fs::read_to_string(input_path)?;
            let entry = entry_point.as_deref().unwrap_or("start");
            let cor24_asm = sw_cor24_rust::translate_msp430(&msp430_asm, entry)?;

            if let Some(out_path) = output_path {
                fs::write(&out_path, &cor24_asm)?;
                eprintln!("Wrote COR24 assembly to {}", out_path);
            } else {
                println!("{}", cor24_asm);
            }

            Ok(())
        }
    }
}

fn run_test() -> Result<()> {
    // The MSP430 assembly from our test compilation
    let msp430_asm = r#"
	.file	"msp430_test.93892fd9a69df810-cgu.0"
	.section	.text.add,"ax",@progbits
	.globl	add
	.p2align	1
	.type	add,@function
add:
	add	r13, r12
	ret
.Lfunc_end1:
	.size	add, .Lfunc_end1-add

	.section	.text.bitmask,"ax",@progbits
	.globl	bitmask
	.p2align	1
	.type	bitmask,@function
bitmask:
	and	r13, r12
	ret
.Lfunc_end2:
	.size	bitmask, .Lfunc_end2-bitmask

	.section	.text.delay,"ax",@progbits
	.globl	delay
	.p2align	1
	.type	delay,@function
delay:
	sub	#2, r1
	tst	r12
	jeq	.LBB5_3
	add	#-1, r12
.LBB5_2:
	mov	r12, 0(r1)
	add	#-1, r12
	cmp	#-1, r12
	jne	.LBB5_2
.LBB5_3:
	add	#2, r1
	ret
.Lfunc_end5:
	.size	delay, .Lfunc_end5-delay

	.section	.text.mmio_read,"ax",@progbits
	.globl	mmio_read
	.p2align	1
	.type	mmio_read,@function
mmio_read:
	mov	0(r12), r12
	ret
.Lfunc_end6:
	.size	mmio_read, .Lfunc_end6-mmio_read

	.section	.text.mmio_write,"ax",@progbits
	.globl	mmio_write
	.p2align	1
	.type	mmio_write,@function
mmio_write:
	mov	r13, 0(r12)
	ret
.Lfunc_end7:
	.size	mmio_write, .Lfunc_end7-mmio_write

	.section	.text.blink_loop,"ax",@progbits
	.globl	blink_loop
	.p2align	1
	.type	blink_loop,@function
blink_loop:
.LBB2_1:
	mov	#-256, r12
	mov	#1, r13
	call	#mmio_write
	mov	#1000, r12
	call	#delay
	mov	#-256, r12
	clr	r13
	call	#mmio_write
	mov	#1000, r12
	call	#delay
	jmp	.LBB2_1
.Lfunc_end2b:
	.size	blink_loop, .Lfunc_end2b-blink_loop

	.section	.text.button_echo,"ax",@progbits
	.globl	button_echo
	.p2align	1
	.type	button_echo,@function
button_echo:
.LBB3_1:
	mov	#-256, r12
	call	#mmio_read
	mov	r12, r13
	and	#1, r13
	mov	#-256, r12
	call	#mmio_write
	jmp	.LBB3_1
.Lfunc_end3:
	.size	button_echo, .Lfunc_end3-button_echo

	.section	.text.compare_branch,"ax",@progbits
	.globl	compare_branch
	.p2align	1
	.type	compare_branch,@function
compare_branch:
	cmp	r13, r12
	jlo	.LBB4_2
	mov	r13, r12
.LBB4_2:
	ret
.Lfunc_end4:
	.size	compare_branch, .Lfunc_end4-compare_branch

	.section	.text.add_three,"ax",@progbits
	.globl	add_three
	.p2align	1
	.type	add_three,@function
add_three:
	add	r13, r12
	add	r14, r12
	ret
.Lfunc_end1b:
	.size	add_three, .Lfunc_end1b-add_three

	.section	.text.shift_and_mask,"ax",@progbits
	.globl	shift_and_mask
	.p2align	1
	.type	shift_and_mask,@function
shift_and_mask:
	and.b	#15, r13
	cmp.b	#0, r13
	jeq	.LBB8_2
.LBB8_1:
	clrc
	rrc	r12
	sub.b	#1, r13
	jne	.LBB8_1
.LBB8_2:
	and	#15, r12
	ret
.Lfunc_end8:
	.size	shift_and_mask, .Lfunc_end8-shift_and_mask

	.section	.text.start,"ax",@progbits
	.globl	start
	.p2align	1
	.type	start,@function
start:
	call	#blink_loop
.Lfunc_end9:
	.size	start, .Lfunc_end9-start
"#;

    let cor24_asm = sw_cor24_rust::translate_msp430(msp430_asm, "start")?;

    println!("=== MSP430 -> COR24 Translation ===\n");
    println!("{}", cor24_asm);

    // Also try assembling it to verify syntax
    println!("\n=== Verifying assembly... ===\n");
    let mut assembler = cor24_assembler::Assembler::new();
    let result = assembler.assemble(&cor24_asm);
    if result.errors.is_empty() {
        println!(
            "Assembly successful! {} bytes generated.",
            result.bytes.len()
        );
        for line in &result.lines {
            if !line.source.trim().is_empty() {
                println!(
                    "  {:06X}: {:16} {}",
                    line.address,
                    line.bytes
                        .iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<_>>()
                        .join(" "),
                    line.source
                );
            }
        }
    } else {
        println!("Assembly errors:");
        for err in &result.errors {
            println!("  ERROR: {}", err);
        }
    }

    Ok(())
}

/// Compile a Rust project to MSP430 assembly, then translate to COR24
fn compile_and_translate(dir: &str, entry_point: &str) -> Result<()> {
    use std::process::Command;

    eprintln!("Compiling {} to MSP430 assembly...", dir);

    let output = Command::new("rustup")
        .args([
            "run",
            "nightly",
            "cargo",
            "rustc",
            "--target",
            "msp430-none-elf",
            "-Z",
            "build-std=core",
            "--release",
            "--",
            "--emit",
            "asm",
        ])
        .current_dir(dir)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Compilation failed:\n{}", stderr);
    }

    // Find the generated .s file
    let target_dir = format!("{}/target/msp430-none-elf/release/deps", dir);
    let mut asm_file = None;
    for entry in fs::read_dir(&target_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|e| e == "s").unwrap_or(false) {
            asm_file = Some(path);
            break;
        }
    }

    let asm_path = asm_file.ok_or_else(|| anyhow::anyhow!("No .s file found in {}", target_dir))?;
    eprintln!("Found MSP430 assembly: {}", asm_path.display());

    let msp430_asm = fs::read_to_string(&asm_path)?;
    let cor24_asm = sw_cor24_rust::translate_msp430(&msp430_asm, entry_point)?;

    println!("{}", cor24_asm);

    Ok(())
}
