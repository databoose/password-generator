#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

fn get_random_buf() -> Result<[u8; 32], getrandom::Error> { // gives an array of ints, first parameter is type, second is amount of ints
    let mut buf = [0u8; 32];
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
         let alphabet = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
         let alphabet_capital = vec!["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];
         let symbolic = vec!{"!","@","#","$","%","^","&","*","(",")","-","_","+","="};
         let numeric = vec!["1","2","3","4","5","6","7","8","9","0"];
         // 76 possibles, 255 is max of u8 value

         //below 63 = alphabet
         //above 63 and below 127 = alphabet_capital
         //above 127 and below 189 = symbolic
         //above 189 and below 255 = numeric

         let randarr = match get_random_buf() {
             Ok(arr) =>  arr, // we want each element value from the array to represent a char
             Err(_) => panic!("error getting random bytes"),
         };

         for i in 0..randarr.len() {
             //println!("{}", randarr[i]);
             match randarr[i] {
                 0..=63 => println!("alpha {}", randarr[i]),
                 64..=127 => println!("capital {}", randarr[i]),
                 128..=189 => println!("symbolic {}", randarr[i]),
                 190..=255 => println!("numeric {}", randarr[i]),
             }
         }
	  },
	  Err(e) => println!("Error parsing, {}", e)
    }
}
