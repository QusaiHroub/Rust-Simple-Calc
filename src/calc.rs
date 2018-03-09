pub struct Calc;

impl Calc {
	pub fn analysis ( mathematical_str : String , num : &mut Vec < f64 >, opr : &mut Vec < char > ) -> bool {
		let mut _str : String = String::new();
		for s in  mathematical_str.chars() {
			if (s >= '0' && s <= '9') || s == '.' {
				_str.push(s);
			} else if s  == '+' || s  == '-' || s  == '*' || s  == '/' || s  == '^' {
				if _str.len() == 0 {
					num.push(0.0);
				} else {
					num.push(_str.parse::<f64>().unwrap());
					_str = String::new();
				}
				opr.push(s);			
			} else if s == '=' {
				if _str.len() == 0 {
					num.push(0.0);
				} else {
					num.push(_str.parse::<f64>().unwrap());
					_str = String::new();
				}
				return true;
			} else if s == ' ' {
				
			} else {
				println!("\t{} Unsupported symbol.\n" , s);
				return false;
			}
		}
		println!("\tsymbol \" = \" not found.\n");
		_str = String::new();
		return false;
	}

	pub fn get_result (mut num : Vec < f64 >, mut opr : Vec < char > , result : &mut f64) -> bool {
		let mut ctr = 0;
		while ctr < opr.len() {
			if opr[ctr] == '^' {
				num[ctr] = num[ctr].powf(num[ctr + 1]);
				num.remove(ctr + 1);
				opr.remove(ctr);
			} else {
				ctr += 1;
			}
		}
		ctr = 0;
		while ctr < opr.len() {
			if opr[ctr] == '*' {
				num[ctr] *= num[ctr + 1];
				num.remove(ctr + 1);
				opr.remove(ctr);
			} else if opr[ctr] == '/' {
				if num[ctr + 1] != 0.0 {
					num[ctr] /= num[ctr + 1];
					num.remove(ctr + 1);
					opr.remove(ctr);
				} else {
					println!("\n\tCan't divide by zero.\n");
					return false;
				}
			} else {
				ctr += 1;
			}
		}
		ctr = 0;
		while ctr < opr.len() {
			if opr[ctr] == '+' {
				num[ctr] += num[ctr + 1];
				num.remove(ctr + 1);
				opr.remove(ctr);
			} else if opr[ctr] == '-' {
				num[ctr] -= num[ctr + 1];
				num.remove(ctr + 1);
				opr.remove(ctr);
			} else {
				ctr += 1;
			}
		}

		*result = num[0];
		return true;
	}
}
