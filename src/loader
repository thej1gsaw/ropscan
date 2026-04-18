use object::{Object, ObjectSection, ObjectSymbol};
use std::fs;

/// Section type enum
#[derive(Debug)]
pub enum SectionType {
    Code,
    Data,
    Other,
}

/// Symbol type enum
#[derive(Debug)]
pub enum SymbolType {
    Function,
    Data,
    Other,
}

/// Section struct
#[derive(Debug)]
pub struct Section {
  pub  name: String,
  pub  address: u64,
  pub  size: u64,
  pub  section_type: SectionType,
  pub  bytes: Vec<u8>,
}

/// Symbol struct
#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub address: u64,
    pub symbol_type: SymbolType,
    pub is_weak: bool,
}

/// Binary struct
#[derive(Debug)]
pub struct Binary {
    pub filename: String,
    pub entry_point: u64,
    pub arch: String,
    pub bits: u8,
    pub sections: Vec<Section>,
    pub symbols: Vec<Symbol>,
}

impl Binary {
    pub fn get_bytes_at(&self, addr: u64) -> Option<&[u8]> {
        for section in &self.sections {
            if addr >= section.address && addr < section.address + section.size {
                let offset =  (addr - section.address) as usize;
                return Some(&section.bytes[offset..]);
            }
        }
        None
    }
}

pub fn load_binary(path: &str) -> anyhow::Result<Binary> {

    let data = fs::read(path)?;
    let file = object::File::parse(&*data)?;


    let mut binary = Binary {
        filename: path.to_string(),
        entry_point: file.entry(),
        arch: match file.architecture() {
            object::Architecture::X86_64 => "x86_64".into(),
            object::Architecture::I386 => "x86".into(),
            object::Architecture::Aarch64 => "aarch64".into(),
            other => format!("{:?}", other),
        },
        bits: match file.architecture() {
            object::Architecture::X86_64 | object::Architecture::Aarch64 => 64,
            object::Architecture::I386 => 32,
            _ => 0,
        },
        sections: Vec::new(),
        symbols: Vec::new(),
    };

    // Load sections
    for section in file.sections() {
        let sect_type = match section.kind() {
            object::SectionKind::Text => SectionType::Code,
            object::SectionKind::Data => SectionType::Data,
            _ => SectionType::Other,
        };

        let bytes = match section.data() {
           Ok(b) => b.to_vec(),
           Err(_) => Vec::new(),
        };

        binary.sections.push(Section {
            name: section.name().unwrap_or("<unknown>").into(),
            address: section.address(),
            size: section.size(),
            section_type: sect_type,
            bytes,
        });
    }

    // Load symbols
    for symbol in file.symbols() {
        let sym_type = if symbol.kind() == object::SymbolKind::Text {
            SymbolType::Function
        } else if symbol.kind() == object::SymbolKind::Data {
            SymbolType::Data
        } else {
            SymbolType::Other
        };

        let name = symbol.name().unwrap_or("<unknown>").to_string();

        if symbol.is_weak() {
            // Only add weak symbol if no strong symbol with the same name exists
            if !binary.symbols.iter().any(|s| s.name == name && !s.is_weak) {
                binary.symbols.push(Symbol {
                    name,
                    address: symbol.address(),
                    symbol_type: sym_type,
                    is_weak: symbol.is_weak(),
                });
            }
        } else {
            // Strong symbol overrides any existing weak symbol
            binary.symbols.retain(|s| s.name != name);
            binary.symbols.push(Symbol {
                name,
                address: symbol.address(),
                symbol_type: sym_type,
                is_weak: symbol.is_weak(),
            });
        }
    }

    // Print summary

    Ok(binary)
}
