mod loader;
use loader::*;
use capstone::prelude::*;

pub fn disassemble(path: &str) -> Vec<String> {
    let bin = match load_binary(path) {
        Ok(binary) => binary,
        Err(_) => panic!("error loading binary"), //make sure that loading process is ok
    };

    let instruct = disasm(&bin);
    instruct
}

fn disasm(binary: &Binary) -> Vec<String> {
    let text = binary.sections.iter()
        .find(|s| matches!(s.sections_type, SectionType::Code))
        .expect("failed to find anything to disassemble");

    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .syntax(arch::x86::ArchSyntax::Intel)
        .build()
        .expect("failed with disassembly engine");

    let insns = cs.disasm_all(&text.bytes, text.address)
        .expect("disassembly failure");

    let mut instruct: Vec<String> = Vec::new();

    for insn in insns.iter() {
        let mut line = format!("0x{:016x}: ", insn.address());

        let bytes = insn.bytes();
        for j in 0..16 {
            if j < bytes.len() {
                line.push_str(&format!("{:02x} ", bytes[j]));
            } else {
                line.push_str("   ");
            }
        }

        let details = format!("{:<12} {}",
            insn.mnemonic().unwrap_or(""),
            insn.op_str().unwrap_or("")
        );
        line.push_str(&details);

        instruct.push(line);
    }

    instruct
}
