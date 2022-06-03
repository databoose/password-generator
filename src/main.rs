#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

fn get_random_buf() -> Result<[u8; 64], getrandom::Error> { // gives an array of ints, first parameter is type, second is amount of ints
    let mut buf = [0u8; 64];
    getrandom::getrandom(&mut buf)?;
    Ok(buf)
}

fn trim_newline(s: &mut String) {
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
}

fn main() {
    use std::io::{stdin,stdout,Write};
    
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
    let length;
    match input.parse::<i32>() {
	  Ok(n) => {
	     length = n;
         let dict = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z', 
                         'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','O','R','S','T','U','V','W','X','Y','Z'
                         ,'!','@','#','$','%','^','&','*','(',')','-','_','+','=',
                         '1','2','3','4','5','6','7','8','9']; //75 chars
         let randarr = match get_random_buf() {
             Ok(arr) =>  arr, // we want each element value from the array to represent a char
             Err(_) => panic!("error getting random bytes"),
         };

         let mut random_passsword = String::new();
         for i in 0..randarr.len() {
             match randarr[i] {
                 0..=75 => {
                     random_passsword.push(dict[randarr[i] as usize]);
                },
                76..=255 => {},
             }
         }
         println!("{}", random_passsword);
	  },
	  Err(e) => println!("Error parsing, {}", e)
    }
}
