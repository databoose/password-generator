#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use rand::Rng;
use std::io::{stdout,stdin,Write};

fn trim_newline(s: &mut String) {
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
}

fn main() {  
    print!("\x1B[2J\x1B[1;1H"); // clear screen	
    println!("----------------------------");
    println!("---- Password Generator ----");
    println!("----------------------------");
    println!("\n");

    let mut input = String::new();
    print!("Enter string length : ");
    stdout().flush(); // print!() by itself doesnt flush stdout for some reason
    stdin().read_line(&mut input)
    	.ok()
        .expect("Failed to read line");
    trim_newline(&mut input); // for mutable variables we also pass &mut <variable_here>
    let length = input.parse::<u16>().unwrap();

    let dict = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z', 
                     'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','O','R','S','T','U','V','W','X','Y','Z'
                     ,'!','@','#','$','%','^','&','*','(',')','-','_','+','=',
                     '1','2','3','4','5','6','7','8','9']; //75 chars
    let mut random_passsword = String::new();
    let mut rng = rand::thread_rng();

    for x in 0..length {
        random_passsword.push(dict[rng.gen_range(0..75) as usize]);
    }
    println!("{}", random_passsword);
}