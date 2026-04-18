//scanner
use crate::disassemble::{*};

pub fn scan(insn: &Vec<String>) -> Vec<String> {
	let mut temp : Vec<String> = Vec::new();
	let mut gadgets : Vec<String> = Vec::new();
	for i in 0..insn.len() {
		if insn[i].contains("ret") {
			temp.push(insn[i].clone());
		}
	}
	
	for i in &temp {
		println!("{}", i);
	}
	temp
}
