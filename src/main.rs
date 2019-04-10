#[cfg(test)]
mod tests {
	
	use super::*;

	#[test]	
	fn test_sum(){
		let mut int_args: Vec<i64> = Vec::with_capacity(3);
		int_args.push(3);
		int_args.push(2);
		int_args.push(1);
		assert_eq!(sum(int_args), 6);
	}

	#[test]
	fn test_prod(){
		let mut int_args: Vec<i64> = Vec::with_capacity(3);
		int_args.push(3);
		int_args.push(2);
		int_args.push(4);
		assert_eq!(product(int_args),24);
	}

	#[test]
	fn test_gcd(){
		let mut int_args: Vec<i64> = Vec::with_capacity(3);
		int_args.push(70);
		int_args.push(49);
		assert_eq!(gcd(&mut int_args),7);
	}

	#[test]
	fn test_lcm(){
		let mut int_args: Vec<i64> = Vec::with_capacity(3);
		int_args.push(10);
		int_args.push(5);
		int_args.push(65);
		assert_eq!(lcm(&mut int_args),130);
	}
	
}

use std::env;
use std::convert::AsRef;

fn main() {
	let args: Vec<String> = env::args().collect();
	match args.len() {
		1 | 2 => return,
		3 => {
			println!("{}", args[2]); 
			return
			 },
		_ => {},	
	}	
	
    
    let mut int_args: Vec<i64> = Vec::with_capacity(args.len() - 2  as usize);
    
    for i in 2..args.len(){
        int_args.push(args[i].parse::<i64>().unwrap());
    }
	    
    match args[1].as_ref(){
		"sum" => println!("{}",sum(int_args)),
		"product" => println!("{}",product(int_args)),
		"gcd" => {
					let mut div:i64 = 0;
					while int_args.len() > 1{
						div = gcd(&mut int_args);
					}
					println!("{}",div)
				 },
		"lcm" => println!("{}",lcm(&mut int_args)),
		_ => println!("Invalid function!"),
    }	 
}

fn sum(int_args: Vec<i64>) -> i64{
	let mut sum:i64 = 0;
	for i in 0..int_args.len(){
		sum = sum + int_args[i]
	}
	sum
}

fn product(int_args: Vec<i64>) -> i64{
	let mut product:i64 = 1;
	for i in 0..int_args.len(){
		product = product * int_args[i]
	}
	product
}

fn gcd(int_args: &mut Vec<i64>) -> i64{
	for i in 0..int_args.len(){
		assert!(int_args[i] != 0);
	}
	
	
	while int_args[0] != 0 {
		if int_args[0] < int_args[1]{
			let t = int_args[0];
			int_args[0] = int_args[1];
			int_args[1] = t;
		}
		int_args[0] = int_args[0] % int_args[1];
	}
	
	int_args.remove(0);
	
	
	int_args[0]
} 

fn lcm(mut int_args:&mut Vec<i64>) -> i64 {
	for i in 0..int_args.len(){
		assert!(int_args[i] !=0);
	}
	let mut arg1 = int_args.to_vec();
	
	let mut least:i64 = 0;
	while arg1.len() > 1 {
		least = (arg1[0]*arg1[1])/gcd(&mut int_args);
		arg1.remove(0);
		arg1[0] = least;
		int_args[0] = least;
	}
	
	least
}


