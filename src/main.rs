//main
use std::{env, process};
mod disassemble;
use disassemble::*;
mod scanning;
use scanning::*;
mod loader;

fn main() {
	let args: Vec<String> = env::args().collect(); 
	if args.len() < 2 { //ensure that they put in the correct number of arguments
		panic!("usage: <ropscan> <ELF>");
	}    
	let filename = &args[1]; //get the file name
	
	let disassembled : Vec<String> = disassemble::disassemble(&args[1]);
	let gadgets : Vec<String> = scanning::scan(&disassembled);
	
	for i in gadgets {
		println!("{}", i);
	} 
}
