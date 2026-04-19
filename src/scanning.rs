use regex::Regex;

pub fn scan(insn: &Vec<String>) -> Vec<String> {
    let mut gadgets: Vec<String> = Vec::new();
    
    for i in 0..insn.len() {
        if insn[i].contains("ret") {
            if i >= 5 {
                let slice = &insn[i-5..=i]; 
                gadgets.push(process_gadget_slice(slice));
            }
        }
    }
    gadgets
}

fn process_gadget_slice(slice: &[String]) -> String {
    let re = Regex::new(r"^0x([0-9a-fA-F]+):\s*(?:[0-9a-fA-F]{2}\s| {3}){16}").unwrap();
    
    let mut formatted_instrs = Vec::new();
    let mut start_address = String::new();

    for (idx, line) in slice.iter().enumerate() {
        if let Some(cap) = re.captures(line) {
            if idx == 0 {
                start_address = cap.get(1).map_or("".into(), |m| m.as_str().into());
            }

            let raw_instr = re.replace(line, "").trim().to_string();
            
            let collapsed: Vec<&str> = raw_instr.split_whitespace().collect();
            let clean_instr = collapsed.join(" "); 

            if !clean_instr.is_empty() {
                formatted_instrs.push(clean_instr);
            }
        }
    }

    format!("0x{}: {};", start_address, formatted_instrs.join("; "))
}
