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
	     //println!("{}", length);
	  },
	  Err(e) => println!("Error parsing, {}", e)
    }
}
