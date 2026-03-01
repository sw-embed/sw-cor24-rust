#!/usr/bin/env python3
"""
Extract decode ROM values from dis_rom.v Verilog file.
Generates Rust code for the complete 256-entry decode table.
"""

# Initialization values for each bit (0-11), from the Verilog file
# Format: 256-bit hex values, MSB first (bit 255 at left, bit 0 at right)
INITVALS = {
    11: "FFFFFFFFFFFFFFFF80000000000000000000000000000000000000000000000000",
    10: "FFFFFFFFFFFFFFFF8007FFFFFFFFFFFFFFFFFFFFFFEFEEEFFFC000000000000000",
    9:  "FFFFFFFFFFFFFFFFFFFFC0FFFFFFFFFFFFFF000000000100000003FFFFFFFFFF80000000",
    8:  "FFFFFFFFFFFFFFFF83F7FFFFFFFF80000FFFFFE000000000003FFFFFFF0007FF00000",
    7:  "FFFFFFFFFFFFFFFF83FFFFF000007F000FFFE01FFFD00000003F000000FC07FCFE000",
    6:  "FFFFFFFFFFFFFFFE38FFC0FF80040FC0FC01E1F802FEEEC0038FFF000E207E281E00",
    5:  "FFFFFFFFFFFFFFFF9C0800000000400000000000003C0000000000000000000381000",
    4:  "FFFFFFFFFFFFFFFF824F030E078030C30C381998701B3C03C024F00F00918603E09C0",
    3:  "FFFFFFFFFFFFFFFFFFFFD28E0C1C0780C30C30715460E1283C03C120F00F04A619398438",
    2:  "FFFFFFFFFFFFFFFF83F800000000400000001FE00013E220003F000000FC000381E00",
    1:  "FFFFFFFFFFFFFFFFFFFFD00C90A926664A28A2A6C0054930119B333FCCCCCCFC515B95F24",
    0:  "FFFFFFFFFFFFFFFFABFA4A14955561861874A010A4B1D916AABFAAAAAAFD0C27C3E92",
}

# Actually, let me re-read the Verilog file more carefully
# The values have 0x prefix stripped and should be exactly 64 hex chars

import re

def parse_verilog(filepath):
    """Parse dis_rom.v and extract initval patterns."""
    with open(filepath, 'r') as f:
        content = f.read()

    # Pattern: defparam mem_0_N.initval = 256'hXXX...XXX ;
    pattern = r"defparam mem_0_(\d+)\.initval = 256'h([0-9A-Fa-f]+)"
    matches = re.findall(pattern, content)

    initvals = {}
    for bit_num, hex_val in matches:
        initvals[int(bit_num)] = hex_val.upper()
        print(f"Bit {bit_num}: {len(hex_val)} hex chars")

    return initvals

def hex_to_bits(hex_str):
    """Convert hex string to bit array (bit 0 = LSB = rightmost)."""
    # Ensure it's 64 hex chars (256 bits)
    hex_str = hex_str.zfill(64)

    # Convert to integer and extract bits
    value = int(hex_str, 16)
    bits = []
    for i in range(256):
        bits.append((value >> i) & 1)
    return bits

def generate_decode_table(initvals):
    """Generate the 256-entry decode table."""
    # Convert each initval to bit array
    bit_arrays = {}
    for bit_num in range(12):
        bit_arrays[bit_num] = hex_to_bits(initvals[bit_num])

    # For each address, combine bits 0-11
    decode_table = []
    for addr in range(256):
        value = 0
        for bit_num in range(12):
            if bit_arrays[bit_num][addr]:
                value |= (1 << bit_num)
        decode_table.append(value)

    return decode_table

def generate_rust_code(decode_table):
    """Generate Rust code for the decode ROM."""
    lines = []
    lines.append("/// Decode ROM extracted from dis_rom.v")
    lines.append("/// Maps 8-bit instruction byte to 12-bit decoded value: [unused:opcode(5):ra(3):rb(3)]")
    lines.append("pub const DECODE_ROM: [u16; 256] = [")

    for i in range(0, 256, 8):
        row = decode_table[i:i+8]
        hex_vals = [f"0x{v:03X}" for v in row]
        comment = f"// 0x{i:02X}-0x{i+7:02X}"
        lines.append(f"    {', '.join(hex_vals)}, {comment}")

    lines.append("];")
    return '\n'.join(lines)

def analyze_opcodes(decode_table):
    """Analyze the decode table to understand opcode mappings."""
    print("\n=== Opcode Analysis ===")

    opcode_names = {
        0x00: "AddReg", 0x01: "AddImm", 0x02: "And", 0x03: "Bra",
        0x04: "Brf", 0x05: "Brt", 0x06: "Ceq", 0x07: "Cls",
        0x08: "Clu", 0x09: "Jal", 0x0A: "Jmp", 0x0B: "La",
        0x0C: "Lb", 0x0D: "Lbu", 0x0E: "Lc", 0x0F: "Lcu",
        0x10: "Lw", 0x11: "Mov", 0x12: "Mul", 0x13: "Or",
        0x14: "Pop", 0x15: "Push", 0x16: "Sb", 0x17: "Shl",
        0x18: "Sra", 0x19: "Srl", 0x1A: "Sub", 0x1B: "SubSp",
        0x1C: "Sw", 0x1D: "Sxt", 0x1E: "Xor", 0x1F: "Zxt",
    }

    reg_names = ["r0", "r1", "r2", "fp", "sp", "z", "r6", "r7"]

    valid_count = 0
    invalid_count = 0

    for addr in range(256):
        decoded = decode_table[addr]
        if decoded == 0xFFF:
            invalid_count += 1
            continue

        valid_count += 1
        opcode = (decoded >> 6) & 0x1F
        ra = (decoded >> 3) & 0x07
        rb = decoded & 0x07

        op_name = opcode_names.get(opcode, f"???({opcode:02X})")
        print(f"0x{addr:02X} -> opcode={op_name:8s} ra={reg_names[ra]:3s} rb={reg_names[rb]:3s}  (raw: 0x{decoded:03X})")

    print(f"\nValid entries: {valid_count}")
    print(f"Invalid entries (0xFFF): {invalid_count}")

def generate_encoding_tables(decode_table):
    """Generate reverse mapping: (opcode, ra, rb) -> byte."""
    # Group by opcode
    opcode_encodings = {}

    for byte_val in range(256):
        decoded = decode_table[byte_val]
        if decoded == 0xFFF:
            continue

        opcode = (decoded >> 6) & 0x1F
        ra = (decoded >> 3) & 0x07
        rb = decoded & 0x07

        if opcode not in opcode_encodings:
            opcode_encodings[opcode] = {}

        # Key is (ra, rb) tuple encoded as ra*8+rb
        key = ra * 8 + rb
        opcode_encodings[opcode][key] = byte_val

    return opcode_encodings

def generate_encoding_rust_code(decode_table):
    """Generate Rust code for instruction encoding."""
    encodings = generate_encoding_tables(decode_table)

    opcode_names = {
        0x00: "AddReg", 0x01: "AddImm", 0x02: "And", 0x03: "Bra",
        0x04: "Brf", 0x05: "Brt", 0x06: "Ceq", 0x07: "Cls",
        0x08: "Clu", 0x09: "Jal", 0x0A: "Jmp", 0x0B: "La",
        0x0C: "Lb", 0x0D: "Lbu", 0x0E: "Lc", 0x0F: "Lcu",
        0x10: "Lw", 0x11: "Mov", 0x12: "Mul", 0x13: "Or",
        0x14: "Pop", 0x15: "Push", 0x16: "Sb", 0x17: "Shl",
        0x18: "Sra", 0x19: "Srl", 0x1A: "Sub", 0x1B: "SubSp",
        0x1C: "Sw", 0x1D: "Sxt", 0x1E: "Xor", 0x1F: "Zxt",
    }

    reg_names = ["r0", "r1", "r2", "fp", "sp", "z", "r6", "r7"]

    lines = []
    lines.append("//! COR24 instruction encoding tables")
    lines.append("//! Auto-generated from dis_rom.v - do not edit manually")
    lines.append("//!")
    lines.append("//! Generated by: scripts/extract_decode_rom.py")
    lines.append("//! Provides reverse mapping: (opcode, ra, rb) -> instruction byte")
    lines.append("")
    lines.append("use crate::cpu::instruction::Opcode;")
    lines.append("")

    # Generate a single large encoding function
    lines.append("/// Encode instruction to byte. Returns None if encoding not found.")
    lines.append("/// For single-byte instructions, returns the opcode byte.")
    lines.append("/// For multi-byte instructions, returns the first byte (opcode + register info).")
    lines.append("pub fn encode_instruction(opcode: Opcode, ra: u8, rb: u8) -> Option<u8> {")
    lines.append("    let ra = ra & 0x07;")
    lines.append("    let rb = rb & 0x07;")
    lines.append("    ")
    lines.append("    match opcode {")

    for opcode in sorted(encodings.keys()):
        op_name = opcode_names.get(opcode, f"Unknown{opcode:02X}")
        enc_map = encodings[opcode]

        lines.append(f"        Opcode::{op_name} => match (ra, rb) {{")

        for key in sorted(enc_map.keys()):
            ra = key // 8
            rb = key % 8
            byte_val = enc_map[key]
            lines.append(f"            ({ra}, {rb}) => Some(0x{byte_val:02X}), // {reg_names[ra]},{reg_names[rb]}")

        lines.append("            _ => None,")
        lines.append("        },")

    lines.append("        Opcode::Invalid => None,")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    # Generate convenience functions for common patterns
    lines.append("/// Encode add register instruction: add ra,rb")
    lines.append("pub fn encode_add_reg(ra: u8, rb: u8) -> Option<u8> {")
    lines.append("    encode_instruction(Opcode::AddReg, ra, rb)")
    lines.append("}")
    lines.append("")

    lines.append("/// Encode add immediate instruction first byte: add ra,imm8")
    lines.append("pub fn encode_add_imm(ra: u8) -> Option<u8> {")
    lines.append("    // AddImm uses rb=7 as marker in decode ROM")
    lines.append("    encode_instruction(Opcode::AddImm, ra, 7)")
    lines.append("}")
    lines.append("")

    lines.append("/// Encode mov instruction: mov ra,rb")
    lines.append("pub fn encode_mov(ra: u8, rb: u8) -> Option<u8> {")
    lines.append("    encode_instruction(Opcode::Mov, ra, rb)")
    lines.append("}")
    lines.append("")

    lines.append("/// Encode push instruction: push ra")
    lines.append("pub fn encode_push(ra: u8) -> Option<u8> {")
    lines.append("    // Push uses rb=4 (sp) in decode ROM")
    lines.append("    encode_instruction(Opcode::Push, ra, 4)")
    lines.append("}")
    lines.append("")

    lines.append("/// Encode pop instruction: pop ra")
    lines.append("pub fn encode_pop(ra: u8) -> Option<u8> {")
    lines.append("    // Pop uses rb=4 (sp) in decode ROM")
    lines.append("    encode_instruction(Opcode::Pop, ra, 4)")
    lines.append("}")
    lines.append("")

    lines.append("/// Encode branch instruction first byte")
    lines.append("pub fn encode_branch(opcode: Opcode) -> Option<u8> {")
    lines.append("    // Branch instructions use ra=7, rb=7 marker")
    lines.append("    encode_instruction(opcode, 7, 7)")
    lines.append("}")
    lines.append("")

    lines.append("/// Encode load/store instruction first byte: lb/lbu/lw/sb/sw ra,offset(rb)")
    lines.append("pub fn encode_load_store(opcode: Opcode, ra: u8, rb: u8) -> Option<u8> {")
    lines.append("    encode_instruction(opcode, ra, rb)")
    lines.append("}")
    lines.append("")

    lines.append("/// Encode lc/lcu instruction first byte: lc ra,imm8")
    lines.append("pub fn encode_lc(ra: u8, unsigned: bool) -> Option<u8> {")
    lines.append("    // Uses rb=7 as marker")
    lines.append("    let opcode = if unsigned { Opcode::Lcu } else { Opcode::Lc };")
    lines.append("    encode_instruction(opcode, ra, 7)")
    lines.append("}")
    lines.append("")

    lines.append("/// Encode la instruction first byte: la ra,addr24")
    lines.append("pub fn encode_la(ra: u8) -> Option<u8> {")
    lines.append("    // Uses rb=7 as marker")
    lines.append("    encode_instruction(Opcode::La, ra, 7)")
    lines.append("}")
    lines.append("")

    lines.append("/// Encode sub sp,imm24 first byte")
    lines.append("pub fn encode_sub_sp() -> Option<u8> {")
    lines.append("    // SubSp uses ra=4 (sp), rb=7 as marker")
    lines.append("    encode_instruction(Opcode::SubSp, 4, 7)")
    lines.append("}")
    lines.append("")

    lines.append("/// Encode jmp (ra) instruction")
    lines.append("pub fn encode_jmp(ra: u8) -> Option<u8> {")
    lines.append("    // Jmp uses rb=7 as marker")
    lines.append("    encode_instruction(Opcode::Jmp, ra, 7)")
    lines.append("}")
    lines.append("")

    lines.append("/// Encode jal ra,(rb) instruction")
    lines.append("pub fn encode_jal(ra: u8, rb: u8) -> Option<u8> {")
    lines.append("    encode_instruction(Opcode::Jal, ra, rb)")
    lines.append("}")
    lines.append("")

    # Add tests
    lines.append("#[cfg(test)]")
    lines.append("mod tests {")
    lines.append("    use super::*;")
    lines.append("")
    lines.append("    #[test]")
    lines.append("    fn test_encode_add_reg() {")
    lines.append("        assert_eq!(encode_add_reg(0, 0), Some(0x00)); // add r0,r0")
    lines.append("        assert_eq!(encode_add_reg(0, 1), Some(0x01)); // add r0,r1")
    lines.append("        assert_eq!(encode_add_reg(1, 0), Some(0x03)); // add r1,r0")
    lines.append("    }")
    lines.append("")
    lines.append("    #[test]")
    lines.append("    fn test_encode_push_pop() {")
    lines.append("        assert_eq!(encode_push(0), Some(0x7D)); // push r0")
    lines.append("        assert_eq!(encode_push(1), Some(0x7E)); // push r1")
    lines.append("        assert_eq!(encode_pop(0), Some(0x79));  // pop r0")
    lines.append("        assert_eq!(encode_pop(1), Some(0x7A));  // pop r1")
    lines.append("    }")
    lines.append("")
    lines.append("    #[test]")
    lines.append("    fn test_encode_mov() {")
    lines.append("        assert_eq!(encode_mov(3, 4), Some(0x65)); // mov fp,sp")
    lines.append("        assert_eq!(encode_mov(4, 3), Some(0x69)); // mov sp,fp")
    lines.append("        assert_eq!(encode_mov(0, 2), Some(0x57)); // mov r0,r2")
    lines.append("    }")
    lines.append("")
    lines.append("    #[test]")
    lines.append("    fn test_encode_branch() {")
    lines.append("        assert_eq!(encode_branch(Opcode::Bra), Some(0x13));")
    lines.append("        assert_eq!(encode_branch(Opcode::Brf), Some(0x14));")
    lines.append("        assert_eq!(encode_branch(Opcode::Brt), Some(0x15));")
    lines.append("    }")
    lines.append("")
    lines.append("    #[test]")
    lines.append("    fn test_encode_lc() {")
    lines.append("        assert_eq!(encode_lc(0, false), Some(0x44)); // lc r0,imm")
    lines.append("        assert_eq!(encode_lc(1, false), Some(0x45)); // lc r1,imm")
    lines.append("        assert_eq!(encode_lc(0, true), Some(0x47));  // lcu r0,imm")
    lines.append("    }")
    lines.append("}")

    return '\n'.join(lines)

if __name__ == "__main__":
    import sys

    verilog_path = "references/COR24-TB/diamond/source/dis_rom/dis_rom.v"

    print("Parsing Verilog file...")
    initvals = parse_verilog(verilog_path)

    print("\nGenerating decode table...")
    decode_table = generate_decode_table(initvals)

    print("\nAnalyzing opcodes...")
    analyze_opcodes(decode_table)

    print("\n=== Generating decode_rom.rs ===")
    rust_code = generate_rust_code(decode_table)

    # Write decode ROM
    with open("src/cpu/decode_rom.rs", 'w') as f:
        f.write("//! COR24 instruction decode ROM\n")
        f.write("//! Auto-generated from dis_rom.v - do not edit manually\n")
        f.write("//!\n")
        f.write("//! Generated by: scripts/extract_decode_rom.py\n")
        f.write("//! Source: references/COR24-TB/diamond/source/dis_rom/dis_rom.v\n")
        f.write("//!\n")
        f.write("//! Format: 12-bit values where bits [10:6] = opcode, [5:3] = ra, [2:0] = rb\n")
        f.write("//! Invalid entries are marked with 0xFFF\n\n")
        f.write(rust_code)
        f.write("\n")

    print("Wrote to src/cpu/decode_rom.rs")

    print("\n=== Generating encode.rs ===")
    encode_code = generate_encoding_rust_code(decode_table)

    # Write encoding tables
    with open("src/cpu/encode.rs", 'w') as f:
        f.write(encode_code)
        f.write("\n")

    print("Wrote to src/cpu/encode.rs")

    # Print summary
    encodings = generate_encoding_tables(decode_table)
    print("\n=== Encoding Summary ===")
    opcode_names = {
        0x00: "AddReg", 0x01: "AddImm", 0x02: "And", 0x03: "Bra",
        0x04: "Brf", 0x05: "Brt", 0x06: "Ceq", 0x07: "Cls",
        0x08: "Clu", 0x09: "Jal", 0x0A: "Jmp", 0x0B: "La",
        0x0C: "Lb", 0x0D: "Lbu", 0x0E: "Lc", 0x0F: "Lcu",
        0x10: "Lw", 0x11: "Mov", 0x12: "Mul", 0x13: "Or",
        0x14: "Pop", 0x15: "Push", 0x16: "Sb", 0x17: "Shl",
        0x18: "Sra", 0x19: "Srl", 0x1A: "Sub", 0x1B: "SubSp",
        0x1C: "Sw", 0x1D: "Sxt", 0x1E: "Xor", 0x1F: "Zxt",
    }
    for opcode in sorted(encodings.keys()):
        name = opcode_names.get(opcode, f"???")
        count = len(encodings[opcode])
        print(f"  {name:8s}: {count} encodings")
