use std::io;
use std::io::{Write};

mod calc;

use calc::Calc;

fn main() {
	let mut m_str = String::new();
	println!("Calc V : 0.0.1\n");
	println!("--------------------------------------------------------------\n");
	println!("You can use | + for Addition , - for Subtraction ,           |");
	println!("            | * for Multiplication , / for Division,         |");
	println!("            | ^ for Power , Every equation should end with = |");
	println!("            | Ctrl + D on Unix like OS or Ctrl + C to Exit.  |\n");
	println!("--------------------------------------------------------------\n");
	print!("Enter your equation : ");
	let _ = io::stdout().flush();
	
	while io::stdin().read_line(&mut m_str).expect("Filed to read line") != 0 as usize {
		let mut num : Vec < f64 > = vec![];
		let mut opr : Vec < char > = vec![];
		let mut result : f64 = 0.0;
		if Calc::analysis(m_str, &mut num , &mut opr) {
			if Calc::get_result(num , opr , &mut result) {
				println!("\n\tresult = {}\n" , result );
			}
		}
		print!("Enter your equation : ");
		m_str = String::new();
		let _ = io::stdout().flush();
	}
	return;
}
